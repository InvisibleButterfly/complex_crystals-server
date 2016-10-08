use std::sync::{Arc, Mutex};
use ::game_engine::GameEngine;
use ::rustc_serialize::json;
use ::network_engine::structures::*;
use ::game_engine::sampleobject::{SampleObject, RadarType, ObjectType};

pub fn world_size(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    let response = WorldSizeResponse {
        width: engine.world_size_x,
        height: engine.world_size_y,
    };
    Some(json::encode(&response).unwrap())
}

pub fn objects(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    let mut objects = vec![];
    for obj in &engine.objects {
        let obj = obj.read().unwrap().clone();
        objects.push(SampleObject {
            owner: obj.owner,
            name: obj.name,
            otype: obj.otype,
            x: obj.x,
            y: obj.y,
            drive_speed: obj.drive_speed,
            drive_dest_x: obj.drive_dest_x,
            drive_dest_y: obj.drive_dest_y,
            radar_radius: obj.radar_radius,
            radar_type: obj.radar_type,
            weapon_active: obj.weapon_active,
            weapon_type: obj.weapon_type,
            weapon_radius: obj.weapon_radius,
            weapon_target_x: obj.weapon_target_x,
            weapon_target_y: obj.weapon_target_y,
            cargo_type: obj.cargo_type,
            cargo_max: obj.cargo_max,
            cargo_current: obj.cargo_current,
            shell_health: obj.shell_health,
            shell_type: obj.shell_type,
        });
    }
    Some(json::encode(&objects).unwrap())
}

pub fn move_object(mutex: &Arc<Mutex<GameEngine>>, input: String, owner: String) -> bool {
    let engine = mutex.lock().unwrap();
    match json::decode(&input) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let mut mvr: MoveObjectRequest = data;
            let object = match engine.get_object(mvr.name) {
                Some(expr) => expr,
                None => return false,
            };
            let mut object = object.write().unwrap();

            if object.owner != owner {
                return false;
            }

            if mvr.x > engine.world_size_x {
                mvr.x = engine.world_size_x;
            }
            if mvr.y > engine.world_size_y {
                mvr.y = engine.world_size_y;
            }

            object.drive_move_to(mvr.x, mvr.y);
        }
    }
    true
}

pub fn info(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    Some(json::encode(&engine.info).unwrap())
}

pub fn radar(mutex: &Arc<Mutex<GameEngine>>, request: String, owner: String) -> Option<String> {
    let engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return None;
        }
        Ok(data) => {
            let name: NameResponse = data;
            let option_object = engine.get_object(name.name).unwrap();
            let object = option_object.read().unwrap();

            if object.owner != owner {
                return None;
            }

            let objects = match object.radar_scan(&engine.objects) {
                Some(expr) => expr,
                None => return None,
            };

            match object.radar_type {
                RadarType::None => return None,
                RadarType::Simple => {
                    let mut result = Vec::new();
                    for obj in objects {
                        let obj = obj.read().unwrap();
                        result.push(SimpleRadarRequest {
                            x: obj.x,
                            y: obj.y,
                        });
                    }
                    return Some(json::encode(&result).unwrap());
                }
                RadarType::Middle => {
                    let mut result = Vec::new();
                    for obj in objects {
                        let obj = obj.read().unwrap().clone();
                        result.push(MiddleRadarRequest {
                            x: obj.x,
                            y: obj.y,
                            name: obj.name.clone(),
                            otype: obj.otype,
                        });
                    }
                    return Some(json::encode(&result).unwrap());
                }
                RadarType::Military => {
                    let mut result = Vec::new();
                    for obj in objects {
                        let obj = obj.read().unwrap();
                        result.push(MilitaryRadarRequest {
                            x: obj.x,
                            y: obj.y,
                            name: obj.name.clone(),
                            otype: obj.otype.clone(),
                            speed: obj.drive_speed,
                        });
                    }
                    return Some(json::encode(&result).unwrap());
                }
            }
        }        
    }

    None
}

pub fn weapon_fire(mutex: &Arc<Mutex<GameEngine>>, request: String, owner: String) -> bool {
    let engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let wfr: WeaponFireRequest = data;
            for obj in &engine.objects {
                let mut obj = obj.write().unwrap();
                if obj.name == wfr.name && obj.owner == owner {
                    obj.weapon_fire(wfr.x, wfr.y);
                }
            }
        }
    }
    true
}

pub fn weapon_stop(mutex: &Arc<Mutex<GameEngine>>, request: String, owner: String) -> bool {
    let engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let name: NameResponse = data;
            for obj in &engine.objects {
                let mut obj = obj.write().unwrap();
                if obj.name == name.name && obj.owner == owner {
                    obj.weapon_stop();
                }
            }
        }
    }
    true
}

pub fn build(mutex: &Arc<Mutex<GameEngine>>, request: String, owner: String) -> bool {
    let mut engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let req: BuildRequest = data;

            let mut flag = false;
            let mut obj_x = 0.0;
            let mut obj_y = 0.0;

            for obj in &engine.objects {
                let obj = obj.read().unwrap();
                if obj.name == req.name && obj.owner == owner.clone() &&
                   obj.otype == ObjectType::Builder {
                    obj_x = obj.x;
                    obj_y = obj.y;
                    flag = true;
                    break;
                }
            }
            if flag {
                engine.add_object(req.oname.clone(),
                                  obj_x,
                                  obj_y,
                                  req.otype.clone(),
                                  owner.clone());
            }
        }
    }
    true
}