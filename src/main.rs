// Add dependencies in Cargo.toml:
// [dependencies]
// csv = "1.1.6"

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.csv")?;
    let mut rdr = ReaderBuilder::new().has_header(true).from_reader(file);

    let mut total = 0;
    let mut count = 0;

    for result in rdr.records() {
        let record = result?;
        if let Some(value) = record.get(1) {
            if let Ok(num) = value.parse::<i32>() {
                total += num;
                count += 1;
            }
        }
    }

    let average = if count > 0 {
        total as f32 / count as f32
    } else {
        0.0
    };

    let mut output = File::create("output.txt")?;
    write!(output, "Average: {:.2}", average)?;

    Ok(())
}

