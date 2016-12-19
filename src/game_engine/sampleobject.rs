use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable, Clone, PartialEq)]
pub enum ObjectType {
    Asteroid,
    Builder,
    Battlecruiser,
    Harvester,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum RadarType {
    None,
    Simple,
    Middle,
    Military,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum WeaponType {
    None,
    Mining,
    Laser,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum CargoType {
    None,
    Mining,
    Battery,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum ArmorType {
    Asteroid,
    Light,
    Middle,
    Heavy,
    Building,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SampleObject {
    pub owner: String,
    pub name: String,
    pub otype: ObjectType,
    pub x: f64,
    pub y: f64,

    pub drive_speed: f64,
    pub drive_dest_x: f64,
    pub drive_dest_y: f64,

    pub radar_radius: f64,
    pub radar_type: RadarType,

    pub weapon_active: bool,
    pub weapon_type: WeaponType,
    pub weapon_radius: f64,
    pub weapon_target_x: f64,
    pub weapon_target_y: f64,

    pub cargo_type: CargoType,
    pub cargo_max: f64,
    pub cargo_current: f64,

    pub shell_health: f64,
    pub shell_type: ArmorType,
}

impl SampleObject {
    pub fn new(owner: String, name: String, otype: ObjectType, x: f64, y: f64) -> Self {
        match otype {
            ObjectType::Asteroid => {
                SampleObject {
                    owner: owner,
                    name: name,
                    otype: otype,
                    x: x,
                    y: y,
                    drive_speed: 0.0,
                    drive_dest_x: x,
                    drive_dest_y: y,
                    radar_radius: 0.0,
                    radar_type: RadarType::None,
                    weapon_active: false,
                    weapon_type: WeaponType::None,
                    weapon_radius: 0.0,
                    weapon_target_x: x,
                    weapon_target_y: y,
                    cargo_type: CargoType::None,
                    cargo_max: 0.0,
                    cargo_current: 0.0,
                    shell_health: 1000.0,
                    shell_type: ArmorType::Asteroid,
                }
            }
            ObjectType::Harvester => {
                SampleObject {
                    owner: owner,
                    name: name,
                    otype: otype,
                    x: x,
                    y: y,
                    drive_speed: 0.001,
                    drive_dest_x: x,
                    drive_dest_y: y,
                    radar_radius: 10.0,
                    radar_type: RadarType::Middle,
                    weapon_active: false,
                    weapon_type: WeaponType::Mining,
                    weapon_radius: 10.0,
                    weapon_target_x: x,
                    weapon_target_y: y,
                    cargo_type: CargoType::Mining,
                    cargo_max: 100.0,
                    cargo_current: 0.0,
                    shell_health: 100.0,
                    shell_type: ArmorType::Light,
                }
            }
            ObjectType::Battlecruiser => {
                SampleObject {
                    owner: owner,
                    name: name,
                    otype: otype,
                    x: x,
                    y: y,
                    drive_speed: 0.002,
                    drive_dest_x: x,
                    drive_dest_y: y,
                    radar_radius: 100.0,
                    radar_type: RadarType::Military,
                    weapon_active: false,
                    weapon_type: WeaponType::Laser,
                    weapon_radius: 1.0,
                    weapon_target_x: x,
                    weapon_target_y: y,
                    cargo_type: CargoType::Battery,
                    cargo_max: 100.0,
                    cargo_current: 100.0,
                    shell_health: 300.0,
                    shell_type: ArmorType::Light,
                }
            }
            ObjectType::Builder => {
                SampleObject {
                    owner: owner,
                    name: name,
                    otype: otype,
                    x: x,
                    y: y,
                    drive_speed: 0.001,
                    drive_dest_x: x,
                    drive_dest_y: y,
                    radar_radius: 100.0,
                    radar_type: RadarType::Simple,
                    weapon_active: false,
                    weapon_type: WeaponType::None,
                    weapon_radius: 0.0,
                    weapon_target_x: x,
                    weapon_target_y: y,
                    cargo_type: CargoType::Mining,
                    cargo_max: 10000.0,
                    cargo_current: 0.0,
                    shell_health: 2000.0,
                    shell_type: ArmorType::Building,
                }
            }
        }
    }

    pub fn drive_move_to(&mut self, x: f64, y: f64) {
        self.drive_dest_x = x;
        self.drive_dest_y = y;
    }

    pub fn radar_scan(&self,
                      objects: HashMap<String, Arc<RwLock<SampleObject>>>)
                      -> Option<Vec<Arc<RwLock<SampleObject>>>> {
        let result: Vec<Arc<RwLock<SampleObject>>> = objects.iter()
            .filter_map(|x| {
                let (_, v) = x;
                let object = v.read().unwrap();
                if distance(self.x, self.y, object.x, object.y) <= self.radar_radius {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .collect();
        Some(result)
    }

    pub fn weapon_fire(&mut self, x: f64, y: f64) {
        self.weapon_target_x = x;
        self.weapon_target_y = y;
        self.weapon_active = true;
    }

    pub fn weapon_stop(&mut self) {
        self.weapon_active = false;
    }

    pub fn cargo_add(&mut self, size: f64) -> bool {
        if self.cargo_current + size > self.cargo_max {
            return false;
        }
        self.cargo_current += size;
        true
    }

    pub fn cargo_remove(&mut self, size: f64) -> bool {
        if self.cargo_current - size < 0.0 {
            return false;
        }
        self.cargo_current -= size;
        true
    }

    pub fn shell_damage(&mut self, wtype: WeaponType, dmg: f64) {
        match self.shell_type {
            ArmorType::Asteroid => {
                match wtype {
                    WeaponType::None => {}
                    WeaponType::Mining => self.shell_health -= dmg,
                    WeaponType::Laser => self.shell_health -= dmg,
                }
            }
            ArmorType::Building => {
                match wtype {
                    WeaponType::None => {}
                    WeaponType::Mining => self.shell_health -= dmg * 0.0,
                    WeaponType::Laser => self.shell_health -= dmg * 0.001,
                }
            }
            ArmorType::Heavy => {
                match wtype {
                    WeaponType::None => {}
                    WeaponType::Mining => self.shell_health -= dmg * 0.0,
                    WeaponType::Laser => self.shell_health -= dmg * 0.01,
                }
            }
            ArmorType::Middle => {
                match wtype {
                    WeaponType::None => {}
                    WeaponType::Mining => self.shell_health -= dmg * 0.0,
                    WeaponType::Laser => self.shell_health -= dmg * 0.1,
                }
            }
            ArmorType::Light => {
                match wtype {
                    WeaponType::None => {}
                    WeaponType::Mining => self.shell_health -= dmg * 0.001,
                    WeaponType::Laser => self.shell_health -= dmg * 1.0,
                }
            }
        }
    }

    pub fn engine_update(&mut self, elapsed: f64) {
        if !((self.x - self.drive_dest_x).abs() < ::FLOAT_ERR) {
            if self.x < self.drive_dest_x {
                self.x += self.drive_speed * elapsed;
            } else if self.x > self.drive_dest_x {
                self.x -= self.drive_speed * elapsed;
            }
        }
        if !((self.y - self.drive_dest_y).abs() < ::FLOAT_ERR) {
            if self.y < self.drive_dest_y {
                self.y += self.drive_speed * elapsed;
            } else if self.y > self.drive_dest_y {
                self.y -= self.drive_speed * elapsed;
            }
        }
    }
}

pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}
