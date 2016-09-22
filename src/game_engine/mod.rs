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
    Harvester,
    Battlecruiser,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SampleObject {
    pub owner: String,
    pub name: String,
    pub otype: ObjectType,
    pub x: f64,
    pub y: f64,
    pub drive: DriveModule,
    pub radar: RadarModule,
    pub weapon: WeaponModule,
    pub cargo: CargoModule,
    pub armor: ArmorModule,
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
        match otype {
            ObjectType::Harvester => {
                self.objects.push(SampleObject {
                    owner: owner,
                    name: object_name,
                    otype: ObjectType::Harvester,
                    x: coord_x,
                    y: coord_y,
                    drive: DriveModule::new(0.001, coord_x, coord_y),
                    radar: RadarModule::new(100.0, RadarTypes::Middle),
                    weapon: WeaponModule::new(WeaponType::Mining, 10.0),
                    cargo: CargoModule::new(CargoType::Mining, 100.0, 0.0),
                    armor: ArmorModule::new(100.0, ArmorType::Light),
                });
            }
            ObjectType::Battlecruiser => {
                self.objects.push(SampleObject {
                    owner: owner,
                    name: object_name,
                    otype: ObjectType::Battlecruiser,
                    x: coord_x,
                    y: coord_y,
                    drive: DriveModule::new(0.002, coord_x, coord_y),
                    radar: RadarModule::new(300.0, RadarTypes::Military),
                    weapon: WeaponModule::new(WeaponType::Laser, 30.0),
                    cargo: CargoModule::new(CargoType::Battery, 100.0, 100.0),
                    armor: ArmorModule::new(300.0, ArmorType::Light),
                });
            }
        }
    }

    pub fn get_object(&self, name: String) -> Option<Box<SampleObject>> {
        for object in self.objects.clone() {
            if object.name == name {
                return Some(Box::new(object));
            }
        }
        None
    }

    pub fn set_object_dest(&mut self, object_name: String, x: f64, y: f64, owner: String) {
        for obj in &mut self.objects {
            if obj.name == object_name {
                if obj.owner == owner {
                    obj.drive.set_dest(x, y);
                }
                break;
            }
        }
    }
    pub fn game_loop(&mut self, elapsed: f64) {
        for mut object in &mut self.objects {
            // .iter().enumerate() {
            // if !object.armor.check_health() {
            //    self.objects.remove(i);
            //    continue;
            // }

            object.clone().drive.update(&mut object, elapsed);

            // object.clone().weapon.update(&mut object, &mut self.objects);
        }
    }
}