use std::sync::{Arc, Mutex};
use ::game_engine::GameEngine;
use ::rustc_serialize::json;
use ::network_engine::structures::*;
use ::game_engine::sampleobject::{SampleObject, RadarType};

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
    Some(json::encode(&engine.objects).unwrap())
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
            let mut object = match engine.get_object(mvr.name) {
                Some(expr) => expr.clone(),
                None => return false,
            };

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

            let object = match engine.get_object(name.name) {
                Some(expr) => expr.clone(),
                None => return None,
            };

            if object.owner != owner {
                return None;
            }

            let objects = match object.radar_scan(&engine.objects) {
                Some(expr) => expr,
                None => return None,
            };

            match object.radar_type {
                RadarType::Simple => {
                    let mut result = Vec::new();
                    for obj in objects {
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
            for obj in engine.objects {
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
            for obj in engine.objects {
                if obj.name == name.name && obj.owner == owner {
                    obj.weapon_stop();
                }
            }
        }
    }
    true
}