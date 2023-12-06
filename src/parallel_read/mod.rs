//! check guide https://jorgecarleitao.github.io/arrow2/main/guide/io/parquet_read.html

pub mod generic_parallel_read;
pub mod parallel_write_parquet;

use arrow2::{
    array::{Array, PrimitiveArray, Utf8Array},
    chunk::Chunk,
    error::Error,
    error::Result,
    io::parquet::read::{self, ArrayIter},
};
// use log::trace;
use rayon::prelude::*;
use std::{env, fs::File, io::BufReader, time::SystemTime};

pub fn run() {
    let _ = run_multithread();
}

/// Advances each iterator in parallel
/// # Panic
/// If the iterators are empty
fn deserialize_parallel(iters: &mut [ArrayIter<'static>]) -> Result<Chunk<Box<dyn Array>>> {
    // CPU-bounded
    let arrays = iters
        .par_iter_mut()
        .map(|iter| iter.next().transpose())
        .collect::<Result<Vec<_>>>()?;

    Chunk::try_new(arrays.into_iter().map(|x| x.unwrap()).collect())
}

fn parallel_read(path: &str, row_group: usize) -> Result<()> {
    // open the file
    let mut file = BufReader::new(File::open(path)?);

    // read Parquet's metadata and infer Arrow schema
    let metadata = read::read_metadata(&mut file)?;
    let schema = read::infer_schema(&metadata)?;
    // dbg!(&schema);

    let names = &schema
        .fields
        .iter()
        .map(|f| f.name.clone())
        .collect::<Vec<_>>();
    // println!("{:?}", &names);

    let row_group_num = row_group;
    // select the row group from the metadata
    // let row_group = &metadata.row_groups[row_group];
    for row_group in metadata.row_groups.into_iter() {
        let chunk_size = 1024 * 8 * 8;

        // read (IO-bounded) all columns into memory (use a subset of the fields to project)
        let mut columns = read::read_columns_many(
            &mut file,
            &row_group,
            schema.fields.clone(),
            Some(chunk_size),
            None,
            None,
        )?;

        // deserialize (CPU-bounded) to Arrow in chunks
        let mut num_rows = row_group.num_rows();
        while num_rows > 0 {
            num_rows = num_rows.saturating_sub(chunk_size);
            println!("[parquet/deserialize][start]");
            let chunk = deserialize_parallel(&mut columns)?;
            println!("[parquet/deserialize][end][{}]", chunk.len());
            assert!(!chunk.is_empty());
            // dbg!(&chunk);

            let offset = 0; // offset in group
            let row = row_group_num + offset; // index row within all data
            let arrays = chunk.arrays();

            {
                let f1 = 0;
                let f2 = 5;
                let f3 = 10;

                let array_f1 = arrays[f1]
                    .as_any()
                    .downcast_ref::<PrimitiveArray<i64>>()
                    .unwrap();

                let array_f2 = arrays[f2]
                    .as_any()
                    .downcast_ref::<Utf8Array<i32>>()
                    .unwrap();

                let array_f3 = arrays[f3]
                    .as_any()
                    .downcast_ref::<PrimitiveArray<i64>>()
                    .unwrap();

                println!(
                    "entry[{row}] = {}: {:?}, {}: {:?}, {}: {:?}",
                    &names[f1],
                    array_f1.get(offset).unwrap(),
                    &names[f2],
                    array_f2.get(offset).unwrap(),
                    &names[f3],
                    array_f3.get(offset).unwrap(),
                );
            }
        }
    }
    Ok(())
}

pub fn run_multithread() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "./resource/House_Price.parquet"
    };

    let row_group = if args.len() > 2 {
        args[2].parse::<usize>().unwrap()
    } else {
        0
    };

    let start = SystemTime::now();
    parallel_read(file_path, row_group)?;
    println!("took: {} ms", start.elapsed().unwrap().as_millis());

    Ok(())
}

pub fn run_single_threaded() -> Result<()> {
    // say we have a file
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "./resource/House_Price.parquet"
    };

    let mut reader = File::open(file_path)?;

    // we can read its metadata:
    let metadata = read::read_metadata(&mut reader)?;

    // and infer a [`Schema`] from the `metadata`.
    let schema = read::infer_schema(&metadata)?;

    // we can filter the columns we need (here we select all)
    let schema = schema.filter(|_index, _field| true);

    // we can read the statistics of all parquet's row groups (here for each field)
    for field in &schema.fields {
        let statistics = read::statistics::deserialize(field, &metadata.row_groups)?;
        println!("{statistics:#?}");
    }

    // say we found that we only need to read the first two row groups, "0" and "1"
    let row_groups = metadata
        .row_groups
        .into_iter()
        .enumerate()
        .filter(|(index, _)| *index == 0 || *index == 1)
        .map(|(_, row_group)| row_group)
        .collect();

    // we can then read the row groups into chunks
    let chunks = read::FileReader::new(reader, row_groups, schema, Some(1024 * 8 * 8), None, None);

    let start = SystemTime::now();
    for maybe_chunk in chunks {
        let chunk = maybe_chunk?;
        print!("{:?}", chunk);
        assert!(!chunk.is_empty());
    }
    println!("took: {} ms", start.elapsed().unwrap().as_millis());

    Ok(())
}
