
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

use crate::vector::vector;
use crate::interval::interval;
use crate::collision::collision;
use crate::particle::particle;
use crate::polygon_particle::polygon_particle;
use crate::circle_particle::circle_particle;
use crate::poly_poly_constraint::poly_poly_constraint;
use crate::APEngine::APEngine;
use crate::APEngine::APValues;
use crate::collision_detector;
use crate::APEngine::Paint;
use std::default::Default;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };

#[allow(unused_variables)]
#[derive(Default)]
#[allow(dead_code)]
pub struct particle_collection
{
	pub collide_internal:bool,
    poly_particles:Vec<polygon_particle>,
	circle_particles:Vec<circle_particle>,
	poly_poly_constraints:Vec<poly_poly_constraint>,
	is_composite:bool,
	center:vector, 
	delta:vector
}

impl Paint for particle_collection
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
	}
}

pub fn getRelativeAngle(delta:&mut vector, center:&mut vector, p:&mut vector) ->f64
{
	delta.set_to(&p.x - &center.x, &p.y - &center.y);
	return delta.y.atan2(delta.x.clone());
} 

#[allow(dead_code)]
impl particle_collection
{
	pub fn init_composite(&mut self, v:vector)
	{
		self.center = v;
		self.is_composite = true;
	}
	pub fn get_center(&mut self)->&vector
	{
		return &self.center;
	}
	pub fn rotate_by_radian(&mut self, angleRadians:f64, center:vector) 
	{
		for p in self.poly_particles.iter_mut()
		{
			let mut c = &mut self.center;
			let mut d = &mut self.delta;
			let radius:f64 = p.get_center().distance(c);
			let angle:f64 = getRelativeAngle(&mut d, &mut c, &mut p.get_center()) + angleRadians;
			p.set_px((angle.cos() * radius) + c.x);
			p.set_py((angle.sin() * radius) + c.y);
		}
	}  


	pub fn set_collide_internal(&mut self, c:bool)
	{
		self.collide_internal = c;
	}
	pub fn new() -> particle_collection 
    {
        particle_collection::default()
    }

	fn get_poly_particles(&self)->&Vec<polygon_particle>
	{
		return &self.poly_particles;
	}

	pub fn add_poly_particle(&mut self, p:polygon_particle)
	{
		self.poly_particles.push(p);
	}



	fn get_poly_poly_constraint(&self)->&Vec<poly_poly_constraint>
	{
		return &self.poly_poly_constraints;
	}

	pub fn add_poly_poly_constraint(&mut self, p:poly_poly_constraint)
	{
		self.poly_poly_constraints.push(p);
	}

	fn get_circle_particles(&self)->&Vec<circle_particle>
	{
		return &self.circle_particles;
	}

	pub fn add_circle_particle(&mut self, p:circle_particle)
	{
		self.circle_particles.push(p);
	}

	pub fn integrate(&mut self, ap:&APValues) 
	{
		for poly in self.poly_particles.iter_mut()
		{
			poly.update(ap);	
		}
/*
		for circ in self.circle_particles 
		{
			circ.update(ap.delta, ap);	
		}
		*/
	}
	pub fn satisfy_constraints(&mut self, ap:&APValues)
	{
		let length:usize = self.poly_poly_constraints.len();
		//println!("Check LIST - internal - ");
		for i in 0..length
		{
			let mut c = self.poly_poly_constraints.remove(i);
			self.satisfy_constraint(&mut c, ap);
			self.poly_poly_constraints.insert(i, c);
		}
	}	
	pub fn satisfy_constraint(&mut self,constraint: &mut poly_poly_constraint, ap:&APValues)
	{
		let tuple = constraint.get_particles();
		let mut length:usize = self.poly_particles.len();
		//println!("Check LIST - internal - ");
		let mut i:usize = 0;
		let mut p1 = loop
		{
			if self.poly_particles[i].get_id() == &tuple.0
			{
				//let &mut p:&mut polygon_particle = &mut self.poly_particles.remove(i);
				break self.poly_particles.remove(i);
			}
			i+= 1;
		};
		length = self.poly_particles.len();
		i = 0;
		let mut p2 = loop
		{
			if self.poly_particles[i].get_id() == &tuple.1
			{
				//let &mut p:&mut polygon_particle = &mut self.poly_particles.remove(i);
				break self.poly_particles.remove(i);
			}
			i+= 1;
		};
		//contraint.resolve(&mut p1, &mut p2);
		if constraint.is_spring()
		{
			constraint.resolve_spring(&mut p1, &mut p2);
		}
		self.poly_particles.push(p1);
		self.poly_particles.push(p2);
	}	
	pub fn check_collisions(&mut self, ap:&APValues)
	{
		//println!("Check collisions ?");
		if self.collide_internal
		{
			//println!("Check collisions Internal");
			self.check_internal_collisions(ap);
		} 
	}
	pub fn check_internal_collisions(&mut self, ap:&APValues)
	{
		
		let length:usize = self.poly_particles.len();
		//println!("Check LIST - internal - ");
		for i in 0..length
		{
			//println!("Check LIST - internal - {}", i);
			let mut p = self.poly_particles.remove(i);
			if !p.get_collidable()
			{
				//println!("Check LIST -no collision 1");
				self.poly_particles.insert(i, p);
				continue;
			}
			for j in 0..length-1
			{
				let mut p2 = self.poly_particles.remove(j);
				if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed())
				{
					//println!("Check LIST -no collision 2");
					self.poly_particles.insert(j, p2);
					continue;
				}
				
				p.set_samp(p.get_position());
				p2.set_samp(p2.get_position());
				let p_size = p.get_axes_len();
				let p2_size = p.get_axes_len();
				//collision_detector::test_polygon_vs_polygon(&mut p,&mut p2, p_size, p2_size);
				collision_detector::test_rigid_polygon_vs_rigid_polygon(&mut p,&mut p2, p_size, p2_size);
				//collision_detector::test_polygon_vs_polygon2(&mut p,&mut p2, p_size, p2_size);
				self.poly_particles.insert(j, p2);
			}
			self.poly_particles.insert(i, p);
		}
	}

	pub fn check_collisions_vs_collection(&mut self, col:&mut particle_collection, ap:&APValues)
	{
		let length:usize = self.poly_particles.len();
		let length2:usize = col.poly_particles.len();
		//println!("Check LIST - internal - ");
		for i in 0..length
		{
			//println!("Check LIST - internal - {}", i);
			let mut p = self.poly_particles.remove(i);
			if !p.get_collidable()
			{
				//println!("Check LIST -no collision 1");
				self.poly_particles.insert(i, p);
				continue;
			}
			for j in 0..length2
			{
				let mut p2 = col.poly_particles.remove(j);
				if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed())
				{
					//println!("Check LIST -no collision 2");
					col.poly_particles.insert(j, p2);
					continue;
				}
				
				p.set_samp(p.get_position());
				p2.set_samp(p2.get_position());
				let p_size = p.get_axes_len();
				let p2_size = p.get_axes_len();
				//collision_detector::test_polygon_vs_polygon(&mut p,&mut p2, p_size, p2_size);
				collision_detector::test_rigid_polygon_vs_rigid_polygon(&mut p,&mut p2, p_size, p2_size);
				//collision_detector::test_polygon_vs_polygon2(&mut p,&mut p2, p_size, p2_size);
				col.poly_particles.insert(j, p2);
			}
			self.poly_particles.insert(i, p);
		}

			/*
			internal function checkInternalCollisions():void {
		 
			// every particle in this AbstractCollection
			//var plen:int = _particles.length;
			for (var j:int = 0; j < _particles.length; j++) {
				
				var pa:AbstractParticle = _particles[j];
				if (pa == null || ! pa.collidable) continue;
				
				// ...vs every other particle in this AbstractCollection
				for (var i:int = j + 1; i < _particles.length; i++) {
					var pb:AbstractParticle = _particles[i];
					if (pb.collidable) CollisionDetector.test(pa, pb);
				}
				
				// ...vs every other constraint in this AbstractCollection
				//var clen:int = _constraints.length;
				for (var n:int = 0; n < _constraints.length; n++) {
					if(_constraints[n] is SpringConstraint){
						var c:SpringConstraint = _constraints[n];
						if (c.collidable && ! c.isConnectedTo(pa)) {
							c.scp.updatePosition();
							CollisionDetector.test(pa, c.scp);
						}
					}
				}
			}*/
	}
}
