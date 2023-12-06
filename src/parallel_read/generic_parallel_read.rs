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

pub fn parallel_read() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "./resource/train.parquet"
    };

    let row_group = if args.len() > 2 {
        args[2].parse::<usize>().unwrap()
    } else {
        0
    };

    let start = SystemTime::now();
    println!("took: {} ms", start.elapsed().unwrap().as_millis());

    // open the file
    let mut file = BufReader::new(File::open(file_path)?);

    // read Parquet's metadata and infer Arrow schema
    let metadata = read::read_metadata(&mut file)?;
    let schema = read::infer_schema(&metadata)?;
    // dbg!(&schema);

    let names = &schema
        .fields
        .iter()
        .map(|f| f.name.clone())
        .collect::<Vec<_>>();
    println!("{:?}", &names);

    let num_groups = metadata.row_groups.len();

    // select the row group from the metadata
    // let row_group = &metadata.row_groups[row_group];
    for (i, row_group) in metadata.row_groups.into_iter().enumerate() {
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

        println!("row group {i} with {num_rows} rows;");
        while num_rows > 0 {
            num_rows = num_rows.saturating_sub(chunk_size);
            println!("[parquet/deserialize][start]");
            let chunk = deserialize_parallel(&mut columns)?;
            println!("[parquet/deserialize][end][{}]", chunk.len());
            assert!(!chunk.is_empty());
            // dbg!(&chunk);

            // let arrays = chunk.arrays();

            // dbg!(chunk.get(0).unwrap());
            // for (n, _) in names.into_iter().enumerate() {}

            // for offset in 0..num_rows {
            //     let row = i * num_rows + offset + page_num; // index row within all data
            //     println!("{row}");
            // }
        }
    }
    Ok(())
}
