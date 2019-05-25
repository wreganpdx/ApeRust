extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{ GlGraphics};


use crate::vector::Vector;
//use crate::interval::Interval;
//use crate::collision::Collision;
use crate::particle::Particle;
use crate::polygon_particle::PolygonParticle;
use crate::circle_particle::CircleParticle;
use crate::rectangle_particle::RectangleParticle;
//use crate::ap_engine::ApEngine;
//use crate::ap_engine::APValues;
//use crate::collision_detector;
use crate::ap_engine::Paint;
//use std::any::Any;
use std::f64;
use std::default::Default;

#[derive(Default)]
pub struct PolyPolyConstraint
{
	particles:(i64,i64),
	delta:Vector,
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
	pub rect_rect:bool,
	pub circ_circ:bool,
	pub rect_circ:bool,
	width:f64,
	height:f64,
	radian:f64,
	curr:Vector,
    primary_color:[f32; 4],
    secondary_color:[f32; 4],
}

impl Paint for PolyPolyConstraint
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		use graphics::*;
		let rect = rectangle::rectangle_by_corners(0.0, 0.0, self.width.clone(), self.height.clone());

		gl.draw(args.viewport(), |c, gl| 
		{
            let transform = c.transform.trans(self.curr.x.clone(), self.curr.y.clone()).rot_rad(self.radian.clone()).trans(-self.width.clone()/2.0, -self.height.clone()/2.0);
            rectangle(self.primary_color, rect, transform, gl);
        });
	}
}

impl PolyPolyConstraint
{
	pub fn set_height(&mut self, h:f64)
	{
		self.height = h;
	}

	pub fn get_height(&mut self)->f64
	{
		return self.height.clone();
	}
	fn get_width(&mut self)->f64
	{
		return self.width.clone();
	}

	fn get_radian(&mut self)->f64
	{
		return self.radian.clone();
	}
	pub fn set_primary_color(&mut self, c:[f32;4])
    {
        self.primary_color = c;
    }
	pub fn set_secondary_color(&mut self, c:[f32;4])
    {
        self.secondary_color = c;
    }

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
		self.width = rest_length;
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
	pub fn new(id:i64)->PolyPolyConstraint
    {
		let mut p = PolyPolyConstraint::default();
		p.primary_color= [1.0, 0.0, 0.0, 1.0];
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

	pub fn calc_mid_angles(&mut self)
	{
		self.low_mid = (self.max_ang - self.min_ang) / 2.0;
		self.high_mid = (self.max_ang +self.min_ang) / 2.0;
	}
	pub fn get_curr_angle(&self, p1:&mut PolygonParticle,p2:&mut PolygonParticle)->f64
	{
		let ang1:f64 = p1.get_radian().clone();
		let ang2:f64 = p2.get_radian().clone();
		
		let mut ang:f64 = ang1 - ang2;
		while ang > (f64::consts::PI) {ang -= (f64::consts::PI) * 2.0};
		while ang < (f64::consts::PI) {ang += (f64::consts::PI) * 2.0};
		
		return ang;
	}
	pub fn set_angle(&mut self, p1:&Vector,p2:&Vector)
	{
		let angle = p2.minus(p1);
		self.radian = f64::atan2(angle.x, -angle.y) + (f64::consts::PI) * 0.5;

	}
	pub fn set_position(&mut self, p1:&Vector,p2:&Vector)
	{
		self.curr = p2.plus(p1);
		self.curr.mult_equals(0.5);
	}

	pub fn get_curr_length(&mut self)->&f64 
	{
		return &self.curr_length; 
	}

	pub fn resolve_angular(&mut self,p1:&mut PolygonParticle,p2:&mut PolygonParticle) 
	{
		let ca:f64 = self.get_curr_angle(p1,p2);
		let mut _delta:f64 = 0.0;
		
		let mut diff:f64 = self.high_mid - ca;
		while diff > (f64::consts::PI) * 2.0 {diff -= (f64::consts::PI) * 2.0};
		while diff < -(f64::consts::PI) * 2.0 {diff += (f64::consts::PI) * 2.0};
		
		if diff > self.low_mid{
			_delta = diff - self.low_mid;
		}else if diff < - self.low_mid{
			_delta = diff + self.low_mid;
		}else{
			return;
		}
		
		let inv_inertia_total:f64 = p1.get_inv_inertia() + p2.get_inv_inertia() ;
		let delta_ang1:f64 = _delta * p1.get_inv_inertia() /inv_inertia_total;
		let delta_ang2:f64 = _delta * -p2.get_inv_inertia() /inv_inertia_total;
		p1.set_ang_velocity(p1.get_ang_velocity() -delta_ang1 * self.stiffness); 
		p2.set_ang_velocity(p2.get_ang_velocity() -delta_ang2 * self.stiffness); 			
	}
	/**
		 * @private
		 */			
	pub fn resolve_spring(&mut self,p1:&mut PolygonParticle,p2:&mut PolygonParticle) 
	{
		if p1.get_fixed() && p2.get_fixed()
		{ return;}
		self.curr_length = p1.get_position().distance(&p2.get_position());
		self.delta = p1.get_position().minus(&p2.get_position());
		
		
		//let deltaLength:f64 = self.curr_length;			
		let diff:f64 = (&self.curr_length - self.rest_length) / (&self.curr_length * (p1.get_inv_mass() + p2.get_inv_mass()));
		let dmds:Vector = self.delta.mult(diff * self.stiffness);
	
		p1.set_curr(&p1.get_position().minus(&dmds.mult(p1.get_inv_mass())));
		p2.set_curr(&p2.get_position().plus(&dmds.mult(p2.get_inv_mass())));
	}

	pub fn resolve_spring_rect_rect(&mut self,p1:&mut RectangleParticle,p2:&mut RectangleParticle) 
	{
		if p1.get_fixed() && p2.get_fixed()
		{ return;}
		self.curr_length = p1.get_position().distance(&p2.get_position());
		self.delta = p1.get_position().minus(&p2.get_position());
		
		
		//let deltaLength:f64 = self.curr_length;			
		let diff:f64 = (&self.curr_length - self.rest_length) / (&self.curr_length * (p1.get_inv_mass() + p2.get_inv_mass()));
		let dmds:Vector = self.delta.mult(diff * self.stiffness);
	
		if !p1.get_fixed()
		{
			p1.set_curr(&p1.get_position().minus(&dmds.mult(p1.get_inv_mass())));
		}
		if !p2.get_fixed()
		{
			p2.set_curr(&p2.get_position().plus(&dmds.mult(p2.get_inv_mass())));
		}
		self.set_angle(&p1.get_position(),&p2.get_position());
		self.set_position(&p1.get_position(),&p2.get_position());
	}

	pub fn resolve_spring_circ_circ(&mut self,p1:&mut CircleParticle,p2:&mut CircleParticle) 
	{
		if p1.get_fixed() && p2.get_fixed()
		{ return;}
		self.curr_length = p1.get_position().distance(&p2.get_position());
		self.delta = p1.get_position().minus(&p2.get_position());
		
		
		//let deltaLength:f64 = self.curr_length;			
		let diff:f64 = (&self.curr_length - self.rest_length) / (&self.curr_length * (p1.get_inv_mass() + p2.get_inv_mass()));
		let dmds:Vector = self.delta.mult(diff * self.stiffness);
	
		if !p1.get_fixed()
		{
			p1.set_curr(&p1.get_position().minus(&dmds.mult(p1.get_inv_mass())));
		}
		if !p2.get_fixed()
		{
			p2.set_curr(&p2.get_position().plus(&dmds.mult(p2.get_inv_mass())));
		}
		self.set_angle(&p1.get_position(),&p2.get_position());
		
		self.set_position(&p1.get_position(),&p2.get_position());
	}

	pub fn resolve_spring_circ_rect(&mut self,p1:&mut CircleParticle,p2:&mut RectangleParticle) 
	{
		if p1.get_fixed() && p2.get_fixed()
		{ return;}
		self.curr_length = p1.get_position().distance(&p2.get_position());
		self.delta = p1.get_position().minus(&p2.get_position());
		
		
		//let deltaLength:f64 = self.curr_length;			
		let diff:f64 = (&self.curr_length - self.rest_length) / (&self.curr_length * (p1.get_inv_mass() + p2.get_inv_mass()));
		let dmds:Vector = self.delta.mult(diff * self.stiffness);


		
		if !p1.get_fixed()
		{
			p1.set_curr(&p1.get_position().minus(&dmds.mult(p1.get_inv_mass())));
		}
		if !p2.get_fixed()
		{
			p2.set_curr(&p2.get_position().plus(&dmds.mult(p2.get_inv_mass())));
		}
		self.set_angle(&p1.get_position(),&p2.get_position());
		
		self.set_position(&p1.get_position(),&p2.get_position());
	}
}
