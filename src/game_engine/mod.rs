pub mod sampleobject;

use self::sampleobject::*;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerInfo {
    name: String,
    status: String,
    tps: u16,
}

pub struct GameEngine {
    pub info: ServerInfo,
    pub objects: Vec<SampleObject>,
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
        self.objects.push(SampleObject::new(owner, object_name, otype, coord_x, coord_y));
    }

    pub fn get_object(&self, name: String) -> Option<&mut SampleObject> {
        for object in self.objects.clone() {
            if object.name == name {
                return Some(&mut object);
            }
        }
        None
    }

    pub fn set_object_dest(&mut self, object_name: String, x: f64, y: f64, owner: String) {
        match self.get_object(object_name) {
            Some(data) => data.drive_move_to(x, y),
            None => {}
        }
    }

    pub fn game_loop(&mut self, elapsed: f64) {
        for mut object in &mut self.objects.clone() {
            // .iter().enumerate() {
            // if !object.armor.check_health() {
            //    self.objects.remove(i);
            //    continue;
            // }

            object.update(&mut self.objects.clone(), elapsed);

            // object.clone().weapon.update(&mut object, &mut self.objects);
        }
    }
}