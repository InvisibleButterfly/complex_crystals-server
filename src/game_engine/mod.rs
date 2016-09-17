pub mod modules;

use self::modules::DriveModule;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerInfo {
    name: String,
    status: String,
    tps: u16,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SampleObject {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub drive: DriveModule,
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

    pub fn add_object(&mut self, object_name: String, coord_x: f64, coord_y: f64) {
        self.objects.push(SampleObject {
            name: object_name,
            x: coord_x,
            y: coord_y,
            drive: DriveModule {
                speed: 0.1f64,
                dest_x: coord_x,
                dest_y: coord_y,
            },
        });
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
            drv.update(&mut object);
        }
    }
}