use crate::Charge;

pub struct SimValues {
    pub l: u32,
    pub w: u32,
    pub h: u32,
    pub m: f32,
    pub kg: f32,
    pub s: f32,
    pub c: f32,
    pub walls: Vec<bool>,
    pub charges: Vec<Charge>,
}
