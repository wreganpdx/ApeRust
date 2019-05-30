extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{ GlGraphics};


use crate::vector::Vector;
use crate::particle::Particle;
use crate::circle_particle::CircleParticle;
use crate::rectangle_particle::RectangleParticle;
use crate::ap_engine::Paint;
use std::f64;
use std::default::Default;

#[derive(Default)]
pub struct SpringParticle
{
	particles:(i64,i64),
	delta:Vector,
	min_ang:f64,
	max_ang:f64,	
	low_mid:f64,
	high_mid:f64,
	stiffness:f64,
	is_angular:bool,
	is_spring:bool,
	curr_length:f64,
	rest_length:f64,
	pub rect_rect:bool,
	pub circ_circ:bool,
	pub rect_circ:bool,
	radian:f64,

	pub id:i64,
    density:f64,
    axes:Vec<Vector>,
	extents:Vec<f64>,
    curr:Vector,
    prev:Vector,
    temp:Vector,
    samp:Vector,
    forces:Vector,
    velocity:Vector,
    mass:f64,
    friction:f64,
    elasticity:f64,
    rest_loops:i64,
    rest_count:i64,
    center:Vector,
    pinned:bool,
    pinned_to: Vec<Box<Particle>>,//to make this work, it had to be a vector, but only one should be stored here.
    pin:Vector,
    smashable:bool,
    max_exit_velocity:f64,
    at_rest:bool,
    left_max:f64,
    right_max:f64,
    multi_sampe:i64,
    coll:Collision,
    interval:Interval, 
    kfr:f64,
    inv_mass:f64,
    fixed:bool,
    collidable:bool,
	width:f64,
	height:f64,
    created:bool,
    primary_color:[f32; 4],
    secondary_color:[f32; 4],
	rectId:i32
}

impl Paint for SpringParticle
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

impl SpringParticle
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
	fn set_primary_color(&mut self, c:[f32;4])
    {
        self.primary_color = c;
    }
	fn set_secondary_color(&mut self, c:[f32;4])
    {
        self.secondary_color = c;
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

	pub fn resolve_spring_rect_rect(&mut self, middle:&mut RectangleParticle, p1:&mut RectangleParticle,p2:&mut RectangleParticle) 
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

	pub fn resolve_spring_circ_circ(&mut self,middle:&mut RectangleParticle, p1:&mut CircleParticle,p2:&mut CircleParticle) 
	{
		if p1.get_fixed() && p2.get_fixed()
		{ return;}
		self.curr_length = p1.get_position().distance(&p2.get_position());
		self.delta = p1.get_position().minus(&p2.get_position());
		
			
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

	pub fn resolve_spring_circ_rect(&mut self,middle:&mut RectangleParticle, p1:&mut CircleParticle,p2:&mut RectangleParticle) 
	{
		if p1.get_fixed() && p2.get_fixed()
		{ return;}
		self.curr_length = p1.get_position().distance(&p2.get_position());
		self.delta = p1.get_position().minus(&p2.get_position());
		
			
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


