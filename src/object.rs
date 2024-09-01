use std::fs::File;
use std::io::{BufReader, self};
use std::fmt::Display;

use obj::{Obj, load_obj, ObjError};

use crate::transform::Transform;


pub struct Object {
    transform: Transform,
    mesh: Obj
}

pub enum ObjectError {
    IOError(io::Error),
    ObjError(ObjError)
}

impl From<io::Error> for ObjectError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ObjError> for ObjectError {
    fn from(value: ObjError) -> Self {
        Self::ObjError(value)
    }
}

impl Display for ObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => err.fmt(f),
            Self::ObjError(err) => err.fmt(f)
        }
    }
}

impl Object {
    pub fn new(filename: String) -> Result<Self, ObjError> {
        let input = BufReader::new(File::open(filename)?);
        Ok(Self {
            transform: Transform::default(),
            mesh: load_obj(input)?
        })
    }
}
