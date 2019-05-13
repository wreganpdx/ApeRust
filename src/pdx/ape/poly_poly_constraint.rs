use crate::vector::vector;
use crate::interval::interval;
use crate::collision::collision;
use crate::particle::particle;
use crate::polygon_particle::polygon_particle;
use crate::circle_particle::circle_particle;
use crate::APEngine::APEngine;
use crate::APEngine::APValues;
use crate::collision_detector;
use crate::APEngine::Paint;
use std::any::Any;
use std::f64::consts;
use std::default::Default;

#[derive(Default)]
pub struct poly_poly_constraint
{
	particles:(i64,i64),
	delta:vector,
	min_ang:f64,
	max_ang:f64,	
	low_mid:f64,
	high_mid:f64,
	stiffness:f64,
	id:i64,
	is_angular:bool,
	is_spring:bool,
	curr_length:f64,
	rest_length:f64,
}

impl poly_poly_constraint
{
	pub fn init_angular(&mut self, p:(i64,i64), _min:f64, _max:f64, _stiff:f64)
	{
		self.particles = p;
		self.min_ang = _min;
		self.max_ang = _max;
		self.stiffness = _stiff;
		self.is_angular = true;
	}

	pub fn init_spring(&mut self, p:(i64,i64), rest_length:f64, _stiff:f64)
	{
		self.particles = p;
		self.stiffness = _stiff;
		self.rest_length = rest_length;
		self.is_spring = true;
	}
	pub fn is_spring(&mut self)->bool
	{
		return self.is_spring;
	}
	pub fn is_angular(&mut self)->bool
	{
		return self.is_angular;
	}
	pub fn new(id:i64)->poly_poly_constraint
    {
		let mut p = poly_poly_constraint::default();
		p.is_angular = false;
		p.is_spring = false;
		p.set_id(id);
        return p;
    }
	pub fn set_id(&mut self, id:i64)
	{
		self.id = id;
	}
	pub fn get_particles(&mut self)->&(i64,i64)
	{
		return &self.particles;
	}
	pub fn set_particles(&mut self, p:(i64,i64))
	{
		self.particles = p;
	}
	pub fn get_min_angle(&mut self)->f64
	{
		return self.min_ang;
	}
	pub fn set_min_angle(&mut self, p:f64)
	{
		self.min_ang = p;
	}
	pub fn get_max_angle(&mut self)->f64
	{
		return self.max_ang;
	}
	pub fn set_max_angle(&mut self, p:f64)
	{
		self.max_ang = p;
	}

	pub fn calcMidAngles(&mut self)
	{
		self.low_mid = (self.max_ang - self.min_ang) / 2.0;
		self.high_mid = (self.max_ang +self.min_ang) / 2.0;
	}
	pub fn get_curr_angle(&self, p1:&mut polygon_particle,p2:&mut polygon_particle)->f64
	{
		let ang1:f64 = p1.get_radian().clone();
		let ang2:f64 = p2.get_radian().clone();
		
		let mut ang:f64 = ang1 - ang2;
		while ang > consts::PI {ang -= consts::PI * 2.0};
		while ang < consts::PI {ang += consts::PI * 2.0};
		
		return ang;
	}

	pub fn get_curr_length(&mut self)->&f64 
	{
		return &self.curr_length; 
	}

	pub fn resolve_angular(&mut self,p1:&mut polygon_particle,p2:&mut polygon_particle) 
	{
		let ca:f64 = self.get_curr_angle(p1,p2);
		let mut delta:f64 = 0.0;
		
		let mut diff:f64 = self.high_mid - ca;
		while diff > consts::PI * 2.0 {diff -= consts::PI * 2.0};
		while diff < -consts::PI * 2.0 {diff += consts::PI * 2.0};
		
		if diff > self.low_mid{
			delta = diff - self.low_mid;
		}else if diff < - self.low_mid{
			delta = diff + self.low_mid;
		}else{
			return;
		}
		
		let invInertiaTotal:f64 = p1.get_inv_inertia() + p2.get_inv_inertia() ;
		let deltaAng1:f64 = delta * p1.get_inv_inertia() /invInertiaTotal;
		let deltaAng2:f64 = delta * -p2.get_inv_inertia() /invInertiaTotal;
		p1.set_ang_velocity(p1.get_ang_velocity() -deltaAng1 * self.stiffness); 
		p2.set_ang_velocity(p2.get_ang_velocity() -deltaAng2 * self.stiffness); 			
	}
	/**
		 * @private
		 */			
	pub fn resolve_spring(&mut self,p1:&mut polygon_particle,p2:&mut polygon_particle) 
	{
		if p1.get_fixed() && p2.get_fixed()
		{ return;}
		self.curr_length = p1.get_position().distance(&p2.get_position());
		self.delta = p1.get_position().minus(&p2.get_position());
		
		
		//let deltaLength:f64 = self.curr_length;			
		let diff:f64 = (&self.curr_length - self.rest_length) / (&self.curr_length * (p1.get_inv_mass() + p2.get_inv_mass()));
		let dmds:vector = self.delta.mult(diff * self.stiffness);
	
		p1.set_curr(&p1.get_position().minus(&dmds.mult(p1.get_inv_mass())));
		p2.set_curr(&p2.get_position().plus(&dmds.mult(p2.get_inv_mass())));
	}
}
