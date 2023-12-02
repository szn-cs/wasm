#[cfg(feature = "wasm_multithread")]
pub fn read() {
    use parquet2;
    use std::fs::File;
    use std::thread;

    use parquet2::page::Page;
    use parquet2::read::{get_page_iterator, read_metadata};

    let mut file = File::open("tmp/House_Price.parquet").unwrap();
    /*

    /// here we read the metadata.
    let metadata = read_metadata(&mut file)?;

    /// Here we get an iterator of pages (each page has its own data)
    /// This can be heavily parallelized; not even the same `file` is needed here...
    /// feel free to wrap `metadata` under an `Arc`
    let row_group = 0;
    let column = 0;
    let mut iter = get_page_iterator(&metadata, row_group, column, &mut file)?;

    /// A page. It is just (compressed) bytes at this point.
    let page = iter.next().unwrap().unwrap();
    println!("{:#?}", page);

    /// from here, we can do different things. One of them is to convert its buffers to native Rust.
    /// This consumes the page.
    use parquet2::serialization::native::page_to_array;
    let array = page_to_array(page, &descriptor).unwrap();

    let handles = vec![];
    for column in columns {
        let compressed_pages =
        get_page_iterator(&metadata, row_group, column, &mut file, file)?.collect()?;
        // each compressed_page has a buffer; cloning is expensive(!). We move it so that the memory
        // is released at the end of the processing.
        handles.push(thread::spawn(move || {
            page_iter_to_array(compressed_pages.into_iter())
        }))
    }
    let columns_from_all_groups = handles.join_all();
    */
}
