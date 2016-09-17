use game_engine::SampleObject;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct DriveModule {
    pub speed: f64,
    pub dest_x: f64,
    pub dest_y: f64,
}

impl DriveModule {
    pub fn set_dest(&mut self, x: f64, y: f64) {
        self.dest_x = x;
        self.dest_y = y;
    }

    pub fn update(&mut self, object: &mut SampleObject) {
        if !((object.x - self.dest_x).abs() < ::FLOAT_ERR) {
            if object.x < self.dest_x {
                object.x += self.speed;
            } else if object.x > self.dest_x {
                object.x -= self.speed;
            }
        }
        if !((object.y - self.dest_y).abs() < ::FLOAT_ERR) {
            if object.y < self.dest_y {
                object.y += self.speed;
            } else if object.y > self.dest_y {
                object.y -= self.speed;
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