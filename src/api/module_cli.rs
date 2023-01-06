
use std::{fs::File,io::Write};

pub fn read_file(s_file:String)-> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let _file = std::fs::File::open(s_file)?;
    let _reader = std::io::BufReader::new(_file);
    let edata = serde_json::from_reader(_reader).unwrap();
    //let eqmtdata = std::fs::read_to_string("eqmtdata.json").unwrap();
    Ok(edata)
}