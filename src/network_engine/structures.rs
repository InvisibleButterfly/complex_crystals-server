#[derive(RustcEncodable)]
pub struct SampleObjectResponse {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MoveObjectRequest {
    pub name: String,
    pub x: f64,
    pub y: f64,
}