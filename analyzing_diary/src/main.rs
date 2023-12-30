use std::{error::Error, io, process, collections::HashMap, };
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    title: String,
    time: String,
    theater: String,
    play_type: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    price: Option<f64>,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut play_diary = HashMap::<String, u32>::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        play_diary.entry(record.title).and_modify(|count| *count += 1).or_insert(1);
        //println!("{:?}", record);
    }
    for (title, count) in &play_diary {
        println!("{title:?} had been watched for {count} counts");
    }
    Ok(())
}

fn main() {

    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
