use game_engine::SampleObject;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct DriveModule {
    pub speed: f64,
    pub dest_x: f64,
    pub dest_y: f64,
}

impl DriveModule {
    pub fn new(speed: f64, dest_x: f64, dest_y: f64) -> Self {
        DriveModule {
            speed: speed,
            dest_x: dest_x,
            dest_y: dest_y,
        }
    }

    pub fn set_dest(&mut self, x: f64, y: f64) {
        self.dest_x = x;
        self.dest_y = y;
    }

    pub fn update(&mut self, object: &mut SampleObject, elapsed: f64) {
        if !((object.x - self.dest_x).abs() < ::FLOAT_ERR) {
            if object.x < self.dest_x {
                object.x += self.speed * elapsed;
            } else if object.x > self.dest_x {
                object.x -= self.speed * elapsed;
            }
        }
        if !((object.y - self.dest_y).abs() < ::FLOAT_ERR) {
            if object.y < self.dest_y {
                object.y += self.speed * elapsed;
            } else if object.y > self.dest_y {
                object.y -= self.speed * elapsed;
            }
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum RadarTypes {
    Simple,
    Middle,
    Military,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct RadarModule {
    pub radius: f64,
    pub rtype: RadarTypes,
}

impl RadarModule {
    pub fn new(radius: f64, rtype: RadarTypes) -> Self {
        RadarModule {
            radius: radius,
            rtype: rtype,
        }
    }

    pub fn get_nearby_objects(&self,
                              obj_x: f64,
                              obj_y: f64,
                              objects: &Vec<SampleObject>)
                              -> Option<Vec<Box<SampleObject>>> {
        let mut result = vec![];
        for object in objects.clone() {
            if distance(obj_x, obj_y, object.x, object.y) <= self.radius {
                result.push(Box::new(object));
            }
        }
        Some(result)
    }
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum WeaponType {
    Mining,
    Laser,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum CargoType {
    Mining,
    Battery,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct WeaponModule {
    pub active: bool,
    pub wtype: WeaponType,
    pub radius: f64,
    pub target_x: f64,
    pub target_y: f64,
}

impl WeaponModule {
    pub fn new(wtype: WeaponType, radius: f64) -> Self {
        WeaponModule {
            active: false,
            wtype: wtype,
            radius: radius,
            target_x: 0.0,
            target_y: 0.0,
        }
    }

    pub fn fire(&mut self, x: f64, y: f64) {
        self.target_x = x;
        self.target_y = y;
        self.active = true;
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn update(&mut self, object: &mut SampleObject, objects: &mut Vec<SampleObject>) {
        if self.active == true &&
           distance(object.x, object.y, self.target_x, self.target_y) <= self.radius {
            for obj in objects {
                if obj.x == self.target_x && obj.y == self.target_y {
                    match self.wtype {
                        WeaponType::Mining => {
                            if !object.cargo.add_cargo(0.1) {
                                self.active = false;
                            }
                        }
                        WeaponType::Laser => {
                            if !object.cargo.remove_cargo(0.1) {
                                self.active = false;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct CargoModule {
    pub ctype: CargoType,
    pub max_capacity: f64,
    pub current_capacity: f64,
}

impl CargoModule {
    pub fn new(ctype: CargoType, max_capacity: f64, current_capacity: f64) -> Self {
        CargoModule {
            ctype: ctype,
            max_capacity: max_capacity,
            current_capacity: current_capacity,
        }
    }

    pub fn add_cargo(&mut self, size: f64) -> bool {
        if self.current_capacity + size > self.max_capacity {
            return false;
        }
        self.current_capacity += size;
        true
    }
    pub fn remove_cargo(&mut self, size: f64) -> bool {
        if self.current_capacity + size < 0.0 {
            return false;
        }
        self.current_capacity -= size;
        true
    }
}