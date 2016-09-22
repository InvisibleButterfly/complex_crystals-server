#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum ObjectType {
    Harvester,
    Battlecruiser,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum RadarType {
    Simple,
    Middle,
    Military,
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
                    weapon_radius: 50.0,
                    weapon_target_x: x,
                    weapon_target_y: y,
                    cargo_type: CargoType::Battery,
                    cargo_max: 100.0,
                    cargo_current: 100.0,
                    shell_health: 300.0,
                    shell_type: ArmorType::Light,
                }
            }
        }
    }

    pub fn drive_move_to(&mut self, x: f64, y: f64) {
        self.drive_dest_x = x;
        self.drive_dest_y = y;
    }

    pub fn radar_scan(&self, objects: &Vec<SampleObject>) -> Option<Vec<Box<SampleObject>>> {
        let mut result = vec![];
        for object in objects.clone() {
            if distance(self.x, self.y, object.x, object.y) <= self.radar_radius {
                result.push(Box::new(object));
            }
        }
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

    pub fn shell_check_health(&self) -> bool {
        if self.shell_health <= 0.0 {
            false
        } else {
            true
        }
    }

    pub fn shell_damage(&mut self, wtype: WeaponType, dmg: f64) {
        match self.shell_type {
            ArmorType::Asteroid => {
                match wtype {
                    WeaponType::Mining => self.shell_health -= dmg,
                    WeaponType::Laser => self.shell_health -= dmg,
                }
            }
            ArmorType::Building => {
                match wtype {
                    WeaponType::Mining => self.shell_health -= dmg * 0.0,
                    WeaponType::Laser => self.shell_health -= dmg * 0.001,
                }
            }
            ArmorType::Heavy => {
                match wtype {
                    WeaponType::Mining => self.shell_health -= dmg * 0.0,
                    WeaponType::Laser => self.shell_health -= dmg * 0.01,
                }
            }
            ArmorType::Middle => {
                match wtype {
                    WeaponType::Mining => self.shell_health -= dmg * 0.0,
                    WeaponType::Laser => self.shell_health -= dmg * 0.1,
                }
            }
            ArmorType::Light => {
                match wtype {
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

    fn weapon_update(&mut self, objects: &mut Vec<SampleObject>) {
        if self.weapon_active == true &&
           distance(self.x, self.y, self.weapon_target_x, self.weapon_target_y) <=
           self.weapon_radius {
            for obj in objects {
                if obj.x == self.weapon_target_x && obj.y == self.weapon_target_y {
                    match self.weapon_type {
                        WeaponType::Mining => {
                            if !self.cargo_add(0.1) {
                                self.weapon_active = false;
                            } else {
                                obj.shell_damage(WeaponType::Mining, 1.0);
                            }
                        }
                        WeaponType::Laser => {
                            if !self.cargo_remove(0.1) {
                                self.weapon_active = false;
                            } else {
                                obj.shell_damage(WeaponType::Laser, 1.0);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self, objects: &mut Vec<SampleObject>, elapsed: f64) {
        self.engine_update(elapsed);
        self.weapon_update(objects);
    }
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}
