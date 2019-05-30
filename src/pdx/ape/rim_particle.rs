use crate::vector::Vector;
use crate::ap_engine::APValues;
use std::f64;

#[derive(Default)]
pub struct RimParticle
{
    pub speed:f64,
    curr:Vector,
    prev:Vector,
    wr:f64,
    av:f64,
    max_torque:f64,
}

impl RimParticle
{
    pub fn new()->RimParticle
    {
		let p = RimParticle::default();
        return p;
    }
    pub fn set_prev(&mut self, vec:&Vector)
    {
        self.prev = vec.clone();
    }
    pub fn get_curr(&mut self)->Vector
    {
        return self.curr.clone();
    }

    pub fn get_curr_x(&mut self)->f64
    {
        return self.curr.x.clone();
    }
    pub fn get_curr_y(&mut self)->f64
    {
        return self.curr.y.clone();
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
        self.wr = r.clone();
        self.speed = 0.0;
        self.av = 0.0;
        self.curr = Vector::new(r.clone(), 0.0);
        self.prev = Vector::new(0.0,0.0);
    }

    pub fn get_angular_velocity(&self)->&f64
    {
        return &self.av;
    }
    pub fn set_angular_velocity(&mut self, s:f64)
    {
        self.av = s;
    }

    pub fn update(&mut self,ap:&APValues)
    {
        self.speed = f64::max(-self.max_torque, f64::min(self.max_torque, self.speed + self.av));

        let mut dx = -self.curr.get_y();
        let mut dy = self.curr.get_x();
        let len = f64::sqrt(dx * dx + dy * dy);
        dx = dx/len;
        dy = dy/len;

        self.curr.plus_equals(&Vector::new(self.speed*dx, self.speed*dy));

        let ox = self.prev.get_x();
        let oy = self.prev.get_y();
        self.prev.x = self.curr.get_x();
        self.prev.y = self.curr.get_y();
        let px = self.prev.get_x();
        let py = self.prev.get_y();
        self.curr.plus_equals(&Vector::new(ap.damping * (px - ox), ap.damping * (py-oy)));
        let clen:f64 = self.curr.length();
        let diff = (clen - self.wr) / clen;
        self.curr.minus_equals(&self.curr.mult(diff));
    }
}
