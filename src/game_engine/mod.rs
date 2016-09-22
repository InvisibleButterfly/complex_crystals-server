pub mod sampleobject;

use self::sampleobject::*;
use std::sync::{Arc, RwLock};

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerInfo {
    name: String,
    status: String,
    tps: u16,
}

pub struct GameEngine {
    pub info: ServerInfo,
    pub objects: Vec<Arc<RwLock<SampleObject>>>,
    pub world_size_x: f64,
    pub world_size_y: f64,
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            objects: vec![],
            info: ServerInfo {
                name: "ServerName".to_string(),
                status: "Ok".to_string(),
                tps: 0u16,
            },
            world_size_x: 800.0,
            world_size_y: 600.0,
        }
    }
    pub fn update_tps(&mut self, tps: u16) {
        self.info.tps = tps;
    }

    pub fn add_object(&mut self,
                      object_name: String,
                      coord_x: f64,
                      coord_y: f64,
                      otype: ObjectType,
                      owner: String) {
        self.objects.push(Arc::new(RwLock::new(SampleObject::new(owner,
                                                                 object_name,
                                                                 otype,
                                                                 coord_x,
                                                                 coord_y))));
    }

    pub fn get_object(&self, name: String) -> Option<Arc<RwLock<SampleObject>>> {
        for object in &self.objects {
            if object.read().unwrap().name == name {
                return Some(object.clone());
            }
        }
        None
    }

    pub fn set_object_dest(&mut self, object_name: String, x: f64, y: f64, owner: String) {
        match self.get_object(object_name) {
            Some(data) => data.write().unwrap().drive_move_to(x, y),
            None => {}
        }
    }

    pub fn game_loop(&mut self, elapsed: f64) {
        for object in &self.objects {
            // .iter().enumerate() {
            // if !object.armor.check_health() {
            //    self.objects.remove(i);
            //    continue;
            // }

            object.write().unwrap().update(&mut self.objects.clone(), elapsed);
        }
    }
}