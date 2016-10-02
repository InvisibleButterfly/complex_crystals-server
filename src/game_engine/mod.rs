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
            Some(data) => {
                let mut object = data.write().unwrap();
                if object.owner == owner {
                    object.drive_move_to(x, y)
                }
            }
            None => {}
        }
    }

    pub fn game_loop(&mut self, elapsed: f64) {
        for (i, object) in self.objects.clone().iter().enumerate() {
            {
                // Уничтожение объекта в случае закончившихся HP
                let object = object.read().unwrap();
                if object.shell_health <= 0.0 {
                    self.objects.remove(i);
                    continue;
                }
            }

            // Работа двигателя
            object.write().unwrap().engine_update(elapsed);

            // Очень много страшного кода для боевой системы
            let weapon_target_x;
            let weapon_target_y;
            let weapon_active;
            let weapon_radius;
            let weapon_type;
            let x;
            let y;
            {
                // Расход боеприпасов при выстреле
                let mut object = object.write().unwrap();
                if object.weapon_active &&
                   sampleobject::distance(object.x,
                                          object.y,
                                          object.weapon_target_x,
                                          object.weapon_target_y) <=
                   object.weapon_radius {
                    match object.weapon_type {
                        WeaponType::None => {}
                        WeaponType::Mining => {
                            if !object.cargo_add(0.1) {
                                // TODO: Тут баг. Хранилище не должно пополняться в случае направления шахтерского луча в пустоту.
                                object.weapon_active = false;
                            }
                        }
                        WeaponType::Laser => {
                            if !object.cargo_remove(0.1) {
                                object.weapon_active = false;
                            }
                        }
                    }
                }
                weapon_target_x = object.weapon_target_x;
                weapon_target_y = object.weapon_target_y;
                weapon_active = object.weapon_active;
                weapon_radius = object.weapon_radius;
                weapon_type = object.weapon_type.clone();
                x = object.x;
                y = object.y;
            }

            // Нанесение выстрелом повреждений
            if weapon_active &&
               sampleobject::distance(x, y, weapon_target_x, weapon_target_y) <= weapon_radius {
                for obj in &self.objects {
                    let mut obj = obj.write().unwrap();
                    if obj.x == weapon_target_x && obj.y == weapon_target_y {
                        match weapon_type {
                            WeaponType::None => {}
                            WeaponType::Mining => {
                                obj.shell_damage(WeaponType::Mining, 1.0);
                            }
                            WeaponType::Laser => {
                                obj.shell_damage(WeaponType::Laser, 1.0);
                            }
                        }
                    }
                }
            }
        }
    }
}