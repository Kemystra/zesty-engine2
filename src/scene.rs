use serde::{Serialize, Deserialize};
use std::fs;

use crate::math_utils::vector::Vector3;
use crate::object::Object;
use crate::math_utils::FloatType;


pub struct Scene {
    pub object: Object,
}

#[derive(Serialize, Deserialize)]
struct SceneConfig<'a> {
    filename: &'a str,
    position: [FloatType; 3]
}

impl Scene {
    pub fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let scene_config: SceneConfig = serde_json::from_str(&contents).unwrap();
        let mut object = Object::new(scene_config.filename).unwrap();
        object.transform.set_position(Vector3::<FloatType>::new(scene_config.position));
        object.transform.update();

        Scene {
            object
        }
    }
}
