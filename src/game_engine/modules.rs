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
        if !(object.x == self.dest_x) {
            if object.x < self.dest_x {
                object.x += self.speed;
            } else if object.x > self.dest_x {
                object.x -= self.speed;
            }
        }
        if !(object.y == self.dest_y) {
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
