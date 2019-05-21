
/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * AbstractCollection.rs
 * 
 * Summary: Should allow grouping of objects for collision detection
 * ore information, see  https://github.com/arctwelve/ape-js-port/tree/master/org/cove/ape
 */

use crate::vector::Vector;
use crate::particle::Particle;
use crate::polygon_particle::PolygonParticle;
use crate::circle_particle::CircleParticle;
use crate::rectangle_particle::RectangleParticle;
use crate::poly_poly_constraint::PolyPolyConstraint;
use crate::ap_engine::APValues;
use crate::collision_detector;
use crate::ap_engine::Paint;
use std::default::Default;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
//extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::GlGraphics;
//use opengl_graphics::OpenGL;

#[allow(unused_variables)]
#[derive(Default)]
#[allow(dead_code)]
pub struct ParticleCollection
{
	pub collide_internal:bool,
    poly_particles:Vec<PolygonParticle>,
	circle_particles:Vec<CircleParticle>,
	rectangle_particles:Vec<RectangleParticle>,
	poly_poly_constraints:Vec<PolyPolyConstraint>,
	is_composite:bool,
	center:Vector, 
	delta:Vector,
	id:i64,
}

impl Paint for ParticleCollection
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		for poly in self.poly_particles.iter_mut()
		{
			poly.paint(args, gl);
		}

		for poly in self.circle_particles.iter_mut()
		{
			poly.paint(args, gl);
		}

		for poly in self.rectangle_particles.iter_mut()
		{
			poly.paint(args, gl);
		}

		for poly in self.poly_poly_constraints.iter_mut()
		{
			poly.paint(args, gl);
		}
	}
}

pub fn get_relative_angle(delta:&mut Vector, center:&mut Vector, p:&mut Vector) ->f64
{
	delta.set_to(&p.x - &center.x, &p.y - &center.y);
	return delta.y.atan2(delta.x.clone());
} 


#[allow(dead_code)]
impl ParticleCollection
{
	pub fn get_circle_by_id(&mut self, i:&i64)->Option<&mut CircleParticle>
	{
		for p in self.circle_particles.iter_mut()
		{
			if p.id == *i
			{
				return Option::from(p);
			}
		}
		return Option::None;
	}

	pub fn check_collisions_vs_collection(&mut self, rem2:&mut ParticleCollection, ap:&APValues)
	{
		self.check_rectangles_vs_collection(rem2, ap);
		self.check_circs_vs_collection(rem2, ap);
	}
	pub fn init_composite(&mut self, v:Vector)
	{
		self.center = v;
		self.is_composite = true;
	}
	pub fn get_center(&mut self)->&Vector
	{
		return &self.center;
	}

	pub fn set_speed(&mut self, s:f64)
	{
		for p in self.circle_particles.iter_mut()
		{
			p.set_speed(s);
		}
	}

	pub fn rotate_by_radian(&mut self, angle_radians:f64, _center:Vector) 
	{
		for p in self.poly_particles.iter_mut()
		{
			let mut c = &mut self.center;
			let mut d = &mut self.delta;
			let radius:f64 = p.get_center().distance(c);
			let angle:f64 = get_relative_angle(&mut d, &mut c, &mut p.get_center()) + angle_radians;
			p.set_px((angle.cos() * radius) + c.x);
			p.set_py((angle.sin() * radius) + c.y);
		}
	}  


	pub fn set_collide_internal(&mut self, c:bool)
	{
		self.collide_internal = c;
	}
	pub fn new(i:i64) -> ParticleCollection 
    {
        let mut p = ParticleCollection::default();
		p.id = i;
		return p;
    }

	fn get_poly_particles(&self)->&Vec<PolygonParticle>
	{
		return &self.poly_particles;
	}

	pub fn add_poly_particle(&mut self, p:PolygonParticle)
	{
		self.poly_particles.push(p);
	}



	fn get_poly_poly_constraint(&self)->&Vec<PolyPolyConstraint>
	{
		return &self.poly_poly_constraints;
	}

	pub fn add_poly_poly_constraint(&mut self, p:PolyPolyConstraint)
	{
		self.poly_poly_constraints.push(p);
	}

	fn get_circle_particles(&self)->&Vec<CircleParticle>
	{
		return &self.circle_particles;
	}

	pub fn add_circle_particle(&mut self, p:CircleParticle)
	{
		self.circle_particles.push(p);
	}

	fn get_rectangle_particles(&self)->&Vec<RectangleParticle>
	{
		return &self.rectangle_particles;
	}

	pub fn add_rectangle_particle(&mut self, p:RectangleParticle)
	{
		self.rectangle_particles.push(p);
	}

	pub fn integrate(&mut self, ap:&APValues) 
	{
		for poly in self.poly_particles.iter_mut()
		{
			poly.update(ap);	
		}
		for poly in self.rectangle_particles.iter_mut()
		{
			poly.update(ap);	
		}
		for circ in self.circle_particles.iter_mut()
		{
			circ.update(ap);	
		}
	}
	pub fn satisfy_constraints(&mut self, ap:&APValues)
	{
		let length:usize = self.poly_poly_constraints.len();
		for i in 0..length
		{
			let mut c = self.poly_poly_constraints.remove(i);
			if c.rect_circ
			{
				self.satisfy_constraint_circ_rect(&mut c, ap);
			}
			else if c.rect_rect
			{
				self.satisfy_constraint_rect_rect(&mut c, ap);
			}
			else if c.circ_circ
			{
				self.satisfy_constraint_circ_circ(&mut c, ap);
			}
			
			self.poly_poly_constraints.insert(i, c);
		}
	}	
	pub fn satisfy_constraint_rect_rect(&mut self,constraint: &mut PolyPolyConstraint, _ap:&APValues)
	{
		let tuple = constraint.get_particles();
		let mut _length:usize = self.poly_particles.len();
		let mut i:usize = 0;
		let mut p1 = loop
		{
			if self.rectangle_particles[i].get_id() == &tuple.0
			{
				break self.rectangle_particles.remove(i);
			}
			i+= 1;
		};
		_length = self.rectangle_particles.len();
		i = 0;
		let mut p2 = loop
		{
			if self.rectangle_particles[i].get_id() == &tuple.1
			{
				break self.rectangle_particles.remove(i);
			}
			i+= 1;
		};
		if constraint.is_spring()
		{
			constraint.resolve_spring_rect_rect(&mut p1, &mut p2);
		}
		self.rectangle_particles.push(p1);
		self.rectangle_particles.push(p2);
	}	
	pub fn satisfy_constraint_circ_rect(&mut self,constraint: &mut PolyPolyConstraint, _ap:&APValues)
	{
		let tuple = constraint.get_particles();
		let mut _length:usize = self.circle_particles.len();
		let mut i:usize = 0;
		let mut p1 = loop
		{
			if self.circle_particles[i].get_id() == &tuple.0 || self.circle_particles[i].get_id() == &tuple.0
			{
				break self.circle_particles.remove(i);
			}
			i+= 1;
		};
		_length = self.rectangle_particles.len();
		i = 0;
		let mut p2 = loop
		{
			if self.rectangle_particles[i].get_id() == &tuple.1 || self.rectangle_particles[i].get_id() == &tuple.0
			{
				break self.rectangle_particles.remove(i);
			}
			i+= 1;
		};
		if constraint.is_spring()
		{
			constraint.resolve_spring_circ_rect(&mut p1, &mut p2);
		}
		self.circle_particles.push(p1);
		self.rectangle_particles.push(p2);
	}	
	pub fn satisfy_constraint_circ_circ(&mut self,constraint: &mut PolyPolyConstraint, _ap:&APValues)
	{
		let tuple = constraint.get_particles();
		let mut _length:usize = self.circle_particles.len();
		let mut i:usize = 0;
		let mut p1 = loop
		{
			if self.circle_particles[i].get_id() == &tuple.0
			{
				break self.circle_particles.remove(i);
			}
			i+= 1;
		};
		_length = self.circle_particles.len();
		i = 0;
		let mut p2 = loop
		{
			if self.circle_particles[i].get_id() == &tuple.1
			{
				break self.circle_particles.remove(i);
			}
			i+= 1;
		};
		if constraint.is_spring()
		{
			constraint.resolve_spring_circ_circ(&mut p1, &mut p2);
		}
		self.circle_particles.push(p1);
		self.circle_particles.push(p2);
	}	
	pub fn check_collisions(&mut self, ap:&APValues)
	{
		if self.collide_internal
		{
			self.check_internal_collisions(ap);
		} 
	}
	pub fn check_rect_rect_internal_collisions(&mut self, ap:&APValues)
	{
		
		let length:usize = self.rectangle_particles.len();
		
		for i in 0..length
		{
			let mut p = self.rectangle_particles.remove(i);
			if !p.get_collidable()
			{
				self.rectangle_particles.insert(i, p);
				continue;
			}
			let vec = &mut self.rectangle_particles;
			collision_detector::check_rectangle_vs_rects(&mut p, vec, ap);
			self.rectangle_particles.insert(i, p);
		}
	}

	pub fn check_rect_circ_internal_collisions(&mut self, ap:&APValues)
	{
		
		let length:usize = self.rectangle_particles.len();
		
		for i in 0..length
		{
			let mut p = self.rectangle_particles.remove(i);
			if !p.get_collidable()
			{
				self.rectangle_particles.insert(i, p);
				continue;
			}
			let vec = &mut self.circle_particles;
			collision_detector::check_rectangle_vs_circs(&mut p, vec, ap);
			self.rectangle_particles.insert(i, p);
		}
	}

	pub fn check_circ_circ_internal_collisions(&mut self, ap:&APValues)
	{
		
		let length:usize = self.circle_particles.len();
		
		for i in 0..length
		{
			let mut p = self.circle_particles.remove(i);
			if !p.get_collidable()
			{
				self.circle_particles.insert(i, p);
				continue;
			}
			let vec = &mut self.circle_particles;
			collision_detector::check_circ_vs_circ(&mut p, vec, ap);
			self.circle_particles.insert(i, p);
		}
	}

	
	pub fn check_internal_collisions(&mut self, ap:&APValues)
	{
		self.check_rect_rect_internal_collisions(ap);
		self.check_circ_circ_internal_collisions(ap);
		self.check_rect_circ_internal_collisions(ap);
	}
	pub fn check_rectangles_vs_collection(&mut self, col:&mut ParticleCollection, ap:&APValues)
	{
		let length:usize = self.rectangle_particles.len();
		for i in 0..length
		{
			let mut p = self.rectangle_particles.remove(i);
			if !p.get_collidable()
			{
				self.rectangle_particles.insert(i, p);
				continue;
			}

			collision_detector::check_rectangle_vs_rects(&mut p, &mut col.rectangle_particles, ap);
			collision_detector::check_rectangle_vs_circs(&mut p, &mut col.circle_particles, ap);
			self.rectangle_particles.insert(i, p);
		}
	}

	pub fn check_circs_vs_collection(&mut self, col:&mut ParticleCollection, ap:&APValues)
	{
		let length:usize = self.circle_particles.len();
		for i in 0..length
		{
			let mut p = self.circle_particles.remove(i);
			if !p.get_collidable()
			{
				self.circle_particles.insert(i, p);
				continue;
			}

			collision_detector::check_circ_vs_circ(&mut p, &mut col.circle_particles, ap);
			collision_detector::check_circ_vs_rects(&mut p, &mut col.rectangle_particles, ap);
			self.circle_particles.insert(i, p);
		}
	}
	

}
