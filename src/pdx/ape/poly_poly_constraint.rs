extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{ GlGraphics};

use crate::pending_translation::PendingTranslation;
use crate::vector::Vector;
use crate::particle::Particle;
use crate::circle_particle::CircleParticle;
use crate::rectangle_particle::RectangleParticle;
use crate::ap_engine::Paint;
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
	pub id:i64,
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
	fixed_end_limit:f64,

	pub pending:bool,
	pub translation:PendingTranslation,
	collidable:bool,
	rect_id:i64
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
	pub fn get_fixed_end_limit(&self)->f64
	{
		return self.fixed_end_limit.clone();
	}
	pub fn get_rect_id(&self)->i64
	{
		return self.rect_id.clone();
	}

	pub fn set_collidable(&mut self, c:bool)
	{
		self.collidable = c;
	}

	pub fn set_rect_id(&mut self, id:i64)
	{
		self.rect_id = id;
	}

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
	


	pub fn resolve_spring_rect_rect(&mut self,p1:&mut RectangleParticle,p2:&mut RectangleParticle) 
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
		if self.collidable
		{
			println!("Creatiln!ng pending");
			self.translation = PendingTranslation::new(&self.curr, &self.radian, self.get_rect_id());
			self.pending = true;
		}
	}

	pub fn resolve_spring_circ_circ(&mut self,p1:&mut CircleParticle,p2:&mut CircleParticle) 
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
		if self.collidable
		{
			self.translation = PendingTranslation::new(&self.curr, &self.radian, self.get_rect_id());
			self.pending = true;
		}
	}

	pub fn resolve_spring_circ_rect(&mut self,p1:&mut CircleParticle,p2:&mut RectangleParticle) 
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
