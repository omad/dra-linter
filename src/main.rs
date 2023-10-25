use std::{fs::File, io::Read};
//use std::fs::File;
use walkdir::WalkDir;
use serde_yaml::Value;
use serde::{Serialize, Deserialize};

fn main() {
    for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
//        let f_name = entry.file_name().to_string_lossy();
        let f_name = entry.path();
//        let sec = entry.metadata()?.modified()?;

        if f_name.to_string_lossy().ends_with(".yaml") {
            println!("{}", f_name.display());
            let mut s = String::new();
            let _ = File::open(f_name).unwrap().read_to_string(&mut s).unwrap(); // map(|_| ());
            for document in serde_yaml::Deserializer::from_str(&s) {
                let v = Value::deserialize(document).unwrap();
                println!("{:?}", v);
            }
//            let v: Value = serde_yaml::from_str(&s).unwrap();
//            println!("{:?}", v);
        }
        // if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
        //     println!("{}", f_name);
        // }
    }
}


// This allows us to split the rules without caring about their contents.
#[derive(Debug, Serialize, Deserialize)]
struct GenericYamlFile {
    rules: Vec<serde_yaml::Mapping>,
}
