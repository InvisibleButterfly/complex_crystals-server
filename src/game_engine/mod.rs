pub mod sampleobject;
pub mod events;
mod config;

use self::sampleobject::*;
use self::events::*;
use self::config::GameConfig;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::collections::{HashMap, VecDeque};

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerInfo {
    name: String,
    status: String,
    tps: u16,
}

pub struct GameEngine {
    pub info: ServerInfo,
    pub objects: HashMap<String, Arc<RwLock<SampleObject>>>,
    pub world_size_x: f64,
    pub world_size_y: f64,
    pub events: VecDeque<Event>,
    pub config: GameConfig,
}

impl GameEngine {
    pub fn new(width: f64, height: f64) -> Self {
        let config = GameConfig::new("config/engine.json");
        GameEngine {
            objects: HashMap::new(),
            info: ServerInfo {
                name: config.servername.clone(),
                status: "Ok".to_string(),
                tps: 0u16,
            },
            world_size_x: width,
            world_size_y: height,
            events: VecDeque::new(),
            config: config,
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
        self.objects.insert(object_name.clone(),
                            Arc::new(RwLock::new(SampleObject::new(owner,
                                                                   object_name,
                                                                   otype,
                                                                   coord_x,
                                                                   coord_y))));
    }

    pub fn get_object(&self, name: String) -> Option<Arc<RwLock<SampleObject>>> {
        self.objects.get(&name).map(|x| x.clone())
    }

    pub fn get_object_with_owner(&self,
                                 name: String,
                                 owner: String)
                                 -> Option<Arc<RwLock<SampleObject>>> {
        let o_object = match self.objects.get(&name) {
            Some(e) => e,
            None => return None, 
        };
        {
            let object = o_object.read().unwrap();
            if object.owner != owner {
                return None;
            }
        }
        Some(o_object.clone())
    }

    pub fn interact_with_object<F>(&mut self, name: String, owner: String, closure: F)
        where F: Fn(&mut GameEngine, RwLockWriteGuard<SampleObject>)
    {
        let object = match self.objects.get(&name) {
            Some(e) => e.clone(),
            None => return,
        };
        let object = object.write().unwrap();
        if object.owner != owner {
            return;
        }
        closure(self, object);
    }

    pub fn interact<F>(&mut self, name: String, closure: F)
        where F: Fn(&mut GameEngine, RwLockWriteGuard<SampleObject>)
    {
        let object = match self.objects.get(&name) {
            Some(e) => e.clone(),
            None => return,
        };
        let object = object.write().unwrap();
        closure(self, object);
    }

    pub fn game_loop(&mut self, elapsed: f64) {
        self.event(elapsed);
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push_front(event);
    }
    fn event(&mut self, elapsed: f64) {
        let event = match self.events.pop_back() {
            Some(e) => e,
            None => return,
        };
        match event {
            Event::MoveRequest(m_e) => {
                self.interact_with_object(m_e.name.clone(),
                                          m_e.owner.clone(),
                                          |engine, mut object| {
                    object.drive_dest_x = m_e.dest_x;
                    object.drive_dest_y = m_e.dest_y;
                    engine.add_event(Event::Move(MoveEvent {
                        name: m_e.name.clone(),
                        dest_x: m_e.dest_x,
                        dest_y: m_e.dest_y,
                    }));
                });
            }
            Event::FireRequest(f_e) => {
                self.interact_with_object(f_e.name.clone(),
                                          f_e.owner.clone(),
                                          move |engine, mut object| {
                    if object.cargo_remove(1.0) {
                        engine.add_event(Event::Damage(DamageEvent {
                            x: object.weapon_target_x,
                            y: object.weapon_target_y,
                            size: object.weapon_radius,
                            d_type: object.weapon_type.clone(),
                            damage: 1.0,
                        }));
                    }
                });
            }
            Event::BuildRequest(b_e) => {
                self.interact_with_object(b_e.name.clone(), b_e.owner.clone(), |engine, _| {
                    engine.add_event(Event::Build(BuildEvent {
                        name: b_e.name.clone(),
                        b_name: b_e.b_name.clone(),
                        b_type: b_e.b_type.clone(),
                        speed: 0.1,
                        progress: 0.0,
                        max_progress: 100.0,
                    }));
                });
            }
            Event::Move(m_e) => {
                self.interact(m_e.name.clone(), |engine, mut object| {
                    if distance(object.x, object.y, object.drive_dest_x, object.drive_dest_y) <
                       object.drive_speed * elapsed {
                        object.x = object.drive_dest_x;
                        object.y = object.drive_dest_y;
                    } else {
                        if !((object.x - m_e.dest_x).abs() < ::FLOAT_ERR) {
                            if object.x < m_e.dest_x {
                                object.x += object.drive_speed * elapsed;
                            } else if object.x > m_e.dest_x {
                                object.x -= object.drive_speed * elapsed;
                            }
                        }
                        if !((object.y - m_e.dest_y).abs() < ::FLOAT_ERR) {
                            if object.y < m_e.dest_y {
                                object.y += object.drive_speed * elapsed;
                            } else if object.y > m_e.dest_y {
                                object.y -= object.drive_speed * elapsed;
                            }
                        }
                        engine.add_event(Event::Move(m_e.clone()));
                    }
                });
            }
            Event::Destroy(d_e) => {
                self.objects.remove(&d_e.name);
            }
            Event::Damage(d_e) => {
                for i in self.objects.clone().iter() {
                    let (_, v) = i;
                    let mut object = v.write().unwrap();

                    if sampleobject::distance(object.x, object.y, d_e.x, d_e.y) <= d_e.size {
                        object.shell_damage(d_e.d_type.clone(), d_e.damage);
                        if object.shell_health <= 0.0 {
                            self.add_event(Event::Destroy(DestroyEvent {
                                name: object.name.clone(),
                            }));
                        }
                    }
                }
            }
            Event::Build(b_e) => {
                if b_e.progress >= b_e.max_progress {
                    let object = match self.get_object(b_e.name) {
                        Some(e) => e,
                        None => return,
                    };
                    let object = object.read().unwrap();
                    self.add_object(b_e.b_name.clone(),
                                    object.x,
                                    object.y,
                                    b_e.b_type.clone(),
                                    object.owner.clone());
                } else {
                    let mut new_event = b_e.clone();
                    new_event.progress += b_e.speed * elapsed;
                    self.add_event(Event::Build(new_event));
                }
            }
        }
    }
}
