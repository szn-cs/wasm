//! Example demonstrating how to write to parquet in parallel. https://jorgecarleitao.github.io/arrow2/main/guide/io/parquet_write.html
//!
use std::{collections::VecDeque, env, sync::atomic};

use rayon::prelude::*;

use arrow2::{
    array::*,
    chunk::Chunk as AChunk,
    datatypes::*,
    error::{Error, Result},
    io::parquet::{read::ParquetError, write::*},
};

type Chunk = AChunk<Box<dyn Array>>;

struct Bla {
    columns: VecDeque<CompressedPage>,
    current: Option<CompressedPage>,
}

impl Bla {
    pub fn new(columns: VecDeque<CompressedPage>) -> Self {
        Self {
            columns,
            current: None,
        }
    }
}

impl FallibleStreamingIterator for Bla {
    type Item = CompressedPage;
    type Error = Error;

    fn advance(&mut self) -> Result<()> {
        self.current = self.columns.pop_front();
        Ok(())
    }

    fn get(&self) -> Option<&Self::Item> {
        self.current.as_ref()
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn parallel_write(path: &str, schema: Schema, chunks: &[Chunk]) -> Result<usize> {
    // declare the options
    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Snappy,
        version: Version::V2,
        data_pagesize_limit: None,
    };

    let encoding_map = |data_type: &DataType| {
        match data_type.to_physical_type() {
            // remaining is plain
            _ => Encoding::Plain,
        }
    };

    // declare encodings
    let encodings = (&schema.fields)
        .iter()
        .map(|f| transverse(&f.data_type, encoding_map))
        .collect::<Vec<_>>();

    // derive the parquet schema (physical types) from arrow's schema.
    let parquet_schema = to_parquet_schema(&schema)?;

    // let mut duration_processing_ms = &atomic::AtomicUsize::new(0);
    let row_groups;
    {
        row_groups = chunks.into_iter().map(|chunk| {
            // write batch to pages; parallelized by rayon
            let columns = chunk
                .columns() // Arrays in the Chunk
                .par_iter() // parallelize
                .zip(parquet_schema.fields().to_vec())
                .zip(encodings.par_iter())
                .flat_map(move |((array, type_), encoding)| {
                    let result;

                    // let start = std::time::SystemTime::now();
                    {
                        let encoded_columns =
                            array_to_columns(array, type_, options, encoding).unwrap();
                        result = encoded_columns
                            .into_iter()
                            .map(|encoded_pages| {
                                let encoded_pages =
                                    DynIter::new(encoded_pages.into_iter().map(|x| {
                                        x.map_err(|e| ParquetError::OutOfSpec(e.to_string()))
                                    }));
                                encoded_pages
                                    .map(|page| {
                                        compress(page?, vec![], options.compression)
                                            .map_err(|x| x.into())
                                    })
                                    .collect::<Result<VecDeque<_>>>()
                            })
                            .collect::<Vec<_>>()
                    }
                    // let duration_ms = start.elapsed().unwrap().as_millis();
                    // duration_processing_ms
                    //     .fetch_add(duration_ms as usize, atomic::Ordering::Relaxed);

                    result
                })
                .collect::<Result<Vec<VecDeque<CompressedPage>>>>()?;

            let row_group = DynIter::new(
                columns
                    .into_iter()
                    .map(|column| Ok(DynStreamingIterator::new(Bla::new(column)))),
            );
            Result::Ok(row_group)
        });
    }

    // Create a new empty file
    let file = std::io::BufWriter::new(std::fs::File::create(path)?);

    let mut writer = FileWriter::try_new(file, schema, options)?;

    let start = std::time::SystemTime::now();
    let c = row_groups.collect::<Vec<_>>(); // activate lazy parallel iterator
    let duration_ms = start.elapsed().unwrap().as_millis();

    // Write the file.
    for group in c {
        writer.write(group?)?;
    }
    let _size = writer.end(None)?;

    // Ok(duration_processing_ms.load(atomic::Ordering::Relaxed))
    Ok(duration_ms as usize)
}

fn create_chunk(size: usize, num_columns: usize) -> Result<Chunk> {
    // NOTE: all types implementing Array are immutable (no append operation);
    // let c1: Int32Array = Int32Array::new_empty(DataType::Int32);
    // let c1: Box<dyn Array> = c1.boxed();
    // println!("{}", c1.is_empty());

    let c1: Int32Array = (0..size)
        .map(|x| if x % 9 == 0 { None } else { Some(x as i32) })
        .collect::<Int32Array>();

    let c2: Utf8Array<i64> = (0..size)
        .map(|x| {
            if x % 8 == 0 {
                None
            } else {
                Some(x.to_string())
            }
        })
        .collect();

    let mut column_vector = Vec::new();
    for i in (0..num_columns).skip(2) {
        column_vector.push(if i % 2 == 0 {
            c1.clone().boxed()
        } else {
            c2.clone().boxed()
        });
    }
    // move ownership for last 2 elements
    if (num_columns > 1) {
        column_vector.push(c1.boxed());
        if (num_columns > 2) {
            column_vector.push(c2.boxed());
        }
    }

    Chunk::try_new(column_vector)
}

pub fn run(num_columns: usize, column_size: usize, num_threads: usize) -> Result<()> {
    // check memory limits: ensure the stack usage and heap together doesn't exceed the total address space (rough estimate).
    let (a, b) = (
        (column_size * num_columns * 2) as u32,
        (u32::MAX /* 4GB limit of memory */ - 2_u32.pow(20)/* assuming amount needed for other parts of the running program */),
    );
    assert!(a < b);

    let file_path = "./resource/example.parquet";
    let start = std::time::SystemTime::now();

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global() // fixed number of threads in pool (to initialize only once);
        // .build() // non-global setting exhibits a dynamic behavior (see docs);
        .unwrap();

    let mut fields = Vec::new();
    for i in 0..num_columns {
        fields.push(if i % 2 == 0 {
            Field::new(format!("c{}", i), DataType::Int32, true)
        } else {
            Field::new(format!("c{}", i), DataType::LargeUtf8, true)
        });
    }

    // generate data
    let chunk = create_chunk(column_size, num_columns)?;
    // process & write data
    let duration_processing_ms = parallel_write(&file_path, fields.into(), &[chunk])?;

    let duration_total_ms = start.elapsed().unwrap().as_millis();
    let file_size = std::fs::metadata(file_path)?.len();
    println!(
        "{}\n{}\n{}",
        duration_processing_ms, duration_total_ms, file_size
    );

    Ok(())
}
