use crate::Charge;

pub struct SimValues {
    pub n_x: u32,
    pub n_y: u32,
    pub n_z: u32,
    pub m: f32,
    pub kg: f32,
    pub s: f32,
    pub c: f32,
    pub walls: Vec<bool>,
    pub charges: Vec<Charge>,
}
