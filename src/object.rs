use std::{error::Error, fs::File};
use std::io::BufReader;

use obj::{Obj, load_obj};

use crate::transform::Transform;


pub struct Object {
    transform: Transform,
    mesh: Obj
}

impl Object {
    pub fn new(filename: String) -> Result<Self, Box<dyn Error>> {
        let input = BufReader::new(File::open(filename)?);
        Ok(Self {
            transform: Transform::default(),
            mesh: load_obj(input)?
        })
    }
}
