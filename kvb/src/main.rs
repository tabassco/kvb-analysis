use std::path::PathBuf;

use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BrokenElevators {
    features: Vec<Elevator>,
}

#[derive(Serialize, Deserialize)]
struct Elevator {
    geometry: Geometry,
    geometry_name: String,
    properties: Properties,
}

#[derive(Serialize, Deserialize)]
struct Geometry {
    r#type: String,
    coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Properties {
    Kennung: String,
    Bezeichnung: String,
    Haltestellenbereich: String,
    Info: Option<String>,
    timestamp: String,
}

fn broken_elevator(path: &PathBuf) -> BrokenElevators {
    let data = std::fs::read_to_string(path).unwrap();
    let ev: BrokenElevators = serde_json::from_str(&data).unwrap();

    ev
}
fn main() {
    for entry in glob("data/escalator*.json").expect("Failed to read glob pattern") {
        println!("{:?}", entry);
        let path: PathBuf = entry.unwrap();
        if std::fs::File::open(&path)
            .unwrap()
            .metadata()
            .unwrap()
            .len()
            == 0
        {
            println!("File is empty");
            continue;
        }
        let ev = broken_elevator(&path);
        println!("{} Elevators are broken", ev.features.len());
    }
}
