use serde::{Serialize, Deserialize};
use std::fs;

use crate::object::Object;
use crate::math_utils::FloatType;


pub struct Scene {
    pub object: Object,
}

#[derive(Serialize, Deserialize)]
struct SceneConfig<'a> {
    obj_filename: &'a str,
    position: [FloatType; 3]
}

impl Scene {
    pub fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let scene_config: SceneConfig = serde_json::from_str(&contents).unwrap();
        let object = Object::new(scene_config.obj_filename).unwrap();

        Scene {
            object
        }
    }
}
