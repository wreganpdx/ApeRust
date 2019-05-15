use crate::vector::vector;
use crate::APEngine::APValues;
use std::f64;

#[derive(Default)]
pub struct rim_particle
{
    speed:f64,
    curr:vector,
    prev:vector,
    wr:f64,
    av:f64,
    sp:f64,
    max_torque:f64,
}

impl rim_particle
{
    pub fn get_speed(&mut self)->&f64
    {
        return &self.speed;
    }

    pub fn init(&mut self, r:f64, mt:f64)
    {
        self.max_torque = mt;
        self.wr = r;
        self.sp = 0.0;
        self.av = 0.0;
        self.curr = vector::new(r.clone(), 0.0);
        self.prev = vector::new(0.0,0.0);
    }

    pub fn get_angular_velocity(&mut self)->&f64
    {
        return &self.av;
    }
    pub fn set_angular_velocity(&mut self, s:f64)
    {
        self.av = s;
    }

    pub fn update(&mut self, dt:&f64, ap:&APValues)
    {
       // self.sp = f64::MAX(-self.max_torque, f64::MIN(self.max_torque, self.sp + self.av));
        let dx = self.curr.get_x();
        let dy = self.curr.get_y();
        //let len = f64::
    }
}
