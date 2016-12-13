pub mod sampleobject;
pub mod events;

use self::sampleobject::*;
use self::events::*;
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
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            objects: HashMap::new(),
            info: ServerInfo {
                name: "ServerName".to_string(),
                status: "Ok".to_string(),
                tps: 0u16,
            },
            world_size_x: 800.0,
            world_size_y: 600.0,
            events: VecDeque::new(),
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
        let mut o_object = match self.objects.get(&name) {
            Some(e) => e.clone(),
            None => return,
        };
        let mut object = o_object.write().unwrap();
        if object.owner != owner {
            return;
        }
        closure(self, object);
    }

    pub fn game_loop(&mut self, elapsed: f64) {
        self.event();

        for i in self.objects.clone().iter() {
            // Объявление штук
            let (k, v) = i;
            let mut object = v.write().unwrap();

            // Уничтожение объекта в случае закончившихся HP
            if object.shell_health <= 0.0 {
                self.objects.remove(k);
            }

            // Двигатели
            object.engine_update(elapsed);

            // Боевая система
            // Проверка, находится ли цель в радиусе стрельбы
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
                            object.weapon_active = false; // Отключаем оружие при полном заполнении трюма
                        }
                    }
                    WeaponType::Laser => {
                        if !object.cargo_remove(0.1) {
                            object.weapon_active = false; // Отключаем оружие при отсутствии патронов
                        }
                    }
                }

                // Нанесение повреждений
                for y in self.objects.clone().iter() {
                    let (t_k, t_v) = y;
                    let mut t_object = t_v.write().unwrap();

                    if t_object.x == object.weapon_target_x &&
                       t_object.y == object.weapon_target_y {
                        t_object.shell_damage(object.weapon_type.clone(), 1.0);
                    }
                }
            }
        }
    }
    pub fn add_event(&mut self, event: Event) {
        self.events.push_front(event);
    }
    fn event(&mut self) {
        let event = match self.events.pop_back() {
            Some(e) => e,
            None => return,
        };
        match event {
            Event::Move(m_e) => {
                self.interact_with_object(m_e.name.clone(),
                                          m_e.owner.clone(),
                                          move |_, mut object| {
                                              object.drive_move_to(m_e.dest_x, m_e.dest_y)
                                          });
            }
            Event::Fire(f_e) => {
                self.interact_with_object(f_e.name.clone(),
                                          f_e.owner.clone(),
                                          move |_, mut object| {
                                              object.weapon_fire(f_e.dest_x, f_e.dest_y);
                                          });
            }
            Event::Build(b_e) => {
                self.interact_with_object(b_e.name.clone(),
                                          b_e.owner.clone(),
                                          move |engine, mut object| {
                    engine.add_object(b_e.b_name.clone(),
                                      object.x,
                                      object.y,
                                      b_e.b_type.clone(),
                                      object.owner.clone());
                });
            }
        }
    }
}
