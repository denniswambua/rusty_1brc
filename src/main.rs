use log::{error, info};
use std::{
    cmp,
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    env_logger::init();
    let path = "measurements.txt";

    let mut cities: BTreeMap<String, (f32, f32, f32, f32, f32)> = BTreeMap::new();

    info!("Starting ...");

    let _ = read_line(path, &mut cities);

    info!("Cities Size: {}", cities.len());

    for (key, value) in &cities {
        print!("{}={:.1}/{:.1}/{:.1}", key, value.0, value.1, value.2);
    }
}

fn load_cities(line: &str, cities: &mut BTreeMap<String, (f32, f32, f32, f32, f32)>) {
    // Kyoto;8.4
    info!("Reading line {}", line);
    let city: Vec<&str> = line.trim().split(';').collect();
    let city_name: String = city[0].to_string();
    let t: f32 = city[1].parse::<f32>().unwrap();

    cities
        .entry(city_name)
        .and_modify(|temps| {
            let (_mi, avg, _ma, total, count) = temps;
            let t1 = t;
            let t2 = t;
            *_mi = cmp::min_by(t1, *_mi, |a, b| a.total_cmp(b));
            *_ma = cmp::max_by(t2, *_ma, |a, b| a.total_cmp(b));
            *total += t;
            *count += 1.0;
            *avg = *total / *count;
        })
        .or_insert((t, t, t, t, 1.0));
}

fn read_line(
    filename: &str,
    cities: &mut BTreeMap<String, (f32, f32, f32, f32, f32)>,
) -> Result<(), std::io::Error> {
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
