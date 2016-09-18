pub mod modules;

use self::modules::*;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerInfo {
    name: String,
    status: String,
    tps: u16,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum ObjectType {
    Ship,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SampleObject {
    pub name: String,
    pub otype: ObjectType,
    pub x: f64,
    pub y: f64,
    pub drive: DriveModule,
    pub radar: RadarModule,
}

pub struct GameEngine {
    pub info: ServerInfo,
    pub objects: Vec<SampleObject>,
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
        }
    }
    pub fn update_tps(&mut self, tps: u16) {
        self.info.tps = tps;
    }

    pub fn add_object(&mut self,
                      object_name: String,
                      coord_x: f64,
                      coord_y: f64,
                      radar_radius: f64,
                      radar_type: RadarTypes) {
        self.objects.push(SampleObject {
            name: object_name,
            otype: ObjectType::Ship,
            x: coord_x,
            y: coord_y,
            drive: DriveModule {
                speed: 0.001f64,
                dest_x: coord_x,
                dest_y: coord_y,
            },
            radar: RadarModule {
                radius: radar_radius,
                rtype: radar_type,
            },
        });
    }

    pub fn get_object(&self, name: String) -> Option<Box<SampleObject>> {
        for object in self.objects.clone() {
            if object.name == name {
                return Some(Box::new(object));
            }
        }
        None
    }

    pub fn set_object_dest(&mut self, object_name: String, x: f64, y: f64) {
        for obj in &mut self.objects {
            if obj.name == object_name {
                obj.drive.set_dest(x, y);
                break;
            }
        }
    }
    pub fn game_loop(&mut self, elapsed: f64) {
        for mut object in &mut self.objects {
            let mut drv = &mut object.drive.clone();
            drv.update(&mut object, elapsed);
        }
    }
}