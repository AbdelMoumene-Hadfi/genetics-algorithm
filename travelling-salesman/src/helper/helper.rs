use crate::city::city::City;
use rand::{Rng,thread_rng};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn select_index(weights:&[f64]) -> usize {
    let max = weights.last().unwrap();
    let r:f64 = thread_rng().gen_range(0.0,*max);
    weights.iter().rposition(|&w| w<r).unwrap()
}

pub fn dot_file(cities:&[City]) {
    let path = Path::new("test");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}:{}",display,why),
        Ok(file) => file,
    };
    for i in cities {
        let a=format!("\t{} [pos=\"{},{}!\"]\n",i.id,i.x,i.y);
        match file.write_all(a.as_bytes()) {
            Err(why) => panic!("couldn't write to {}:{}",display,why),
            Ok(_) => println!("successefly wrote to {}",display),
        };
    }
}
