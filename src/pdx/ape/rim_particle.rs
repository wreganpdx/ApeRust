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
    pub fn set_prev(&mut self, vec:&vector)
    {
        self.prev = vec.clone();
    }
    pub fn get_curr(&mut self)->vector
    {
        return self.curr.clone();
    }
    pub fn get_speed(&mut self)->f64
    {
        return self.speed.clone();
    }

    pub fn damp_speed(&mut self, d:f64)
    {
        self.speed = self.speed * d;
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
        let mut dx = -self.curr.get_y();
        let mut dy = self.curr.get_x();
        let len = f64::sqrt(dx * dx + dy * dy);
        dx = dx/len;
        dy = dy/len;

        self.curr.plus_equals(&vector::new(self.sp*dx, self.sp*dy));

        let ox = self.prev.get_x();
        let oy = self.prev.get_y();
        self.prev.x = self.curr.get_x().clone();
        self.prev.y = self.curr.get_y().clone();
        let px = self.prev.get_x();
        let py = self.prev.get_y();
        self.curr.plus_equals(&vector::new(ap.damping * (px - ox), ap.damping * (py-oy)));
        let clen:f64 = self.curr.length();
        let diff = (clen - self.wr) / clen;
        self.curr.minus_equals(&self.curr.mult(diff));

    }
}
