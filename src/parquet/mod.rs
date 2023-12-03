use parquet2::{
    error::Error,
    metadata::SchemaDescriptor,
    page::Page,
    read::{decompress, get_page_iterator, read_metadata},
    schema::types::{ParquetType, PhysicalType},
    write::{write_metadata_sidecar, FileWriter, Version, WriteOptions},
};

/**
 * Read metadata (row group, column chunk, page) → decompress → decode → deserialize
 */
pub fn read() -> Result<(), Error> {
    use std::{env, fs::File, thread};

    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "./resource/House_Price.parquet"
    };

    println!("{}", &path);
    let mut reader = File::open(path).unwrap();

    let metadata = read_metadata(&mut reader)?;

    let row_group = 0;
    let column = 0;
    let columns = metadata.row_groups[row_group].columns();
    let column_metadata = &columns[column];

    let pages = get_page_iterator(column_metadata, &mut reader, None, vec![], 1024 * 1024)?;

    let mut decompress_buffer = vec![];
    let mut dict = None;
    for maybe_page in pages {
        let page = maybe_page?;
        let page = decompress(page, &mut decompress_buffer)?;

        match page {
            Page::Dict(page) => {
                // the first page may be a dictionary page, which needs to be deserialized
                // depending on your target in-memory format, you may want to deserialize
                // the values differently...
                // let page = deserialize_dict(&page)?;
                dict = Some(page);

                println!("{:#?}", dict);
            }
            Page::Data(page) => {
                let _array = deserialize(&page, dict.as_ref())?;
            }
        }
    }

    use parquet2::encoding::Encoding;
    use parquet2::page::{split_buffer, DataPage, DictPage, Page};
    use parquet2::schema::types::PhysicalType;

    fn deserialize(page: &DataPage, dict: Option<&DictPage>) -> Result<(), Error> {
        // split the data buffer in repetition levels, definition levels and values
        let (_rep_levels, _def_levels, _values_buffer) = split_buffer(page)?;

        // decode and deserialize.
        match (
            page.descriptor.primitive_type.physical_type,
            page.encoding(),
            dict,
        ) {
            (
                PhysicalType::Int32,
                Encoding::PlainDictionary | Encoding::RleDictionary,
                Some(_dict_page),
            ) => {
                // plain encoded page with a dictionary
                // _dict_page can be downcasted based on the descriptor's physical type
                // todo!()

                println!("{:#?}", _dict_page);
                Ok(())
            }
            (PhysicalType::Int32, Encoding::Plain, None) => {
                // plain encoded page
                // todo!()
                println!("{:#?}", page);
                Ok(())
            }
            _ => {
                println!("{:#?}", page);
                //todo!()
                Ok(())
            }
        }
    }

    /*
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
            column_iter_to_array(compressed_pages.into_iter())
        }))
    }
    let columns_from_all_groups = handles.join_all();
    */
    Ok(())
}
