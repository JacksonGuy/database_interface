use std::{error::Error, io, process};

pub fn read_csv(path: String) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.deserialize() {
        
    }

    Ok(())
}