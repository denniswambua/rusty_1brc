use log::{error, info};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    env_logger::init();
    let path = "measurements.txt";

    let mut cities: HashMap<String, f32> = HashMap::new();

    info!("Starting ...");

    let _ = read_line(path, &mut cities);

    info!("Cities Size: {}", cities.len());
}

fn load_cities(line: &str, cities: &mut HashMap<String, f32>) {
    // Kyoto;8.4
    info!("Reading line {}", line);
    let city: Vec<&str> = line.trim().split(';').collect();
    cities
        .entry(city[0].to_string())
        .or_insert(city[1].parse::<f32>().unwrap());
}

fn read_line(filename: &str, cities: &mut HashMap<String, f32>) -> Result<(), std::io::Error> {
    info!("Opening file {}", filename);
    // open targe file
    let file = File::open(filename)?;
    // uses a reader buffer
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    info!("EOF Reached");
                    break;
                }

                load_cities(&line, cities);

                line.clear();
            }
            Err(err) => {
                error!("err");
                return Err(err);
            }
        };
    }

    Ok(())
}
