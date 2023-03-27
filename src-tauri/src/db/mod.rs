use std::fs;
use arrow::datatypes::{Float64Type, Int64Type};
use arrow::record_batch::RecordBatch;
use arrow::{array::*, ipc::writer::FileWriter, ipc::writer::StreamWriter, ipc::writer::IpcWriteOptions};
use std::fs::File;
use std::sync::Arc;
use arrow::ipc::reader::FileReader;

pub(crate) fn get_tags_list() -> Vec<String> {
    let paths = fs::read_dir("db/tags/").unwrap();

    let mut tags_list: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path().display().to_string();
        let tag = path.split("/").last().unwrap().split(".").next().unwrap().to_string();
        tags_list.push(tag);
    }

    tags_list
}

pub fn write_arrow_file() -> Result<(), arrow::error::ArrowError> {
    // Define schema for Arrow file
    let schema = arrow::datatypes::Schema::new(vec![
        arrow::datatypes::Field::new("a", arrow::datatypes::DataType::Int64, false),
        arrow::datatypes::Field::new("b", arrow::datatypes::DataType::Float64, false),
    ]);

    // Create Arrow record batch
    let a = Int64Array::from(vec![1, 2, 3, 4]);
    let b = Float64Array::from(vec![1.1, 2.2, 3.3, 4.4]);
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(a) as Arc<dyn Array>,
            Arc::new(b) as Arc<dyn Array>,
        ],
    )?;

    let file = File::create("example.arrow").unwrap();
    let mut writer = FileWriter::try_new(file, &schema).unwrap();
    writer.write(&batch).unwrap();
    writer.finish().unwrap();

    Ok(())
}

pub fn read_arrow_file() -> Result<(), arrow::error::ArrowError> {
    let file = File::open("example.arrow").unwrap();

    let mut reader = FileReader::try_new(file, None).unwrap();
    let batch = reader.next().unwrap().unwrap();
    println!("{:?}", batch);
    Ok(())
}