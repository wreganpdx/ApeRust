
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
use crate::circle_particle::CircleParticle;
use crate::rectangle_particle::RectangleParticle;
use crate::poly_poly_constraint::PolyPolyConstraint;
use crate::ap_engine::APValues;
use crate::collision_detector;
use crate::ap_engine::Paint;
use crate::owner_collision::OwnerCollision;
use crate::pending_translation::PendingTranslation;
use std::default::Default;

extern crate piston;
extern crate graphics;
extern crate glutin_window;

use piston::input::*;
use opengl_graphics::GlGraphics;

#[allow(unused_variables)]
#[derive(Default)]
#[allow(dead_code)]
pub struct ParticleCollection
{
	pub collide_internal:bool,
	pub circle_particles:Vec<CircleParticle>,
	rectangle_particles:Vec<RectangleParticle>,
	poly_poly_constraints:Vec<PolyPolyConstraint>,
	is_composite:bool,
	center:Vector, 
	delta:Vector,
	pub id:i64,
}

impl Paint for ParticleCollection
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{

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

	pub fn collide_pending_spring(&mut self, collider:&mut Option<&mut Particle>, pending:OwnerCollision)->bool
	{
		let totalLen = self.rectangle_particles.len() + self.circle_particles.len() + self.poly_poly_constraints.len();
		let mut dist = 0.0;
		let mut ret = false;
		let mut rectIndex = 0;
		let mut rect = loop
		{
			if rectIndex >= self.rectangle_particles.len()
			{
				panic!("line 97, collide_pending_spring");
			}
			if self.rectangle_particles[rectIndex].get_id() == &pending.ownerRect
			{
				break self.rectangle_particles.remove(rectIndex);
			}
			rectIndex+= 1;
		};
		
		
		let mut constraintIndex:usize = 0;
		let mut constraint = loop
		{
			if constraintIndex >= self.poly_poly_constraints.len()
			{
				panic!("line 109, collide_pending_spring, constraintIndex");
			}
			if self.poly_poly_constraints[constraintIndex].id == pending.ownerConstraint
			{
				break self.poly_poly_constraints.remove(constraintIndex);
			}
			constraintIndex+= 1;
			
		};
		if constraint.circ_circ
		{
			
			let c1 = 1.0 - dist;
			let c2 = dist.clone();
			let mut circ1Index = 0;
			let mut circ1 = loop
			{
				if circ1Index >= self.circle_particles.len()
				{
					panic!("line 130, collide_pending_spring, circ1Index {}, sibling1 {}", circ1Index, pending.sibling1);
				}
				if self.circle_particles[circ1Index].get_id() == &pending.sibling1
				{
					break self.circle_particles.remove(circ1Index);
				}
				circ1Index+= 1;
			};
			let mut circ_2_index:usize = 0;
			let mut circ2 = loop
			{
				if circ_2_index >= self.circle_particles.len()
				{
					panic!("line 141, collide_pending_spring, circ2Index {}, sibling2 {}", circ_2_index, pending.sibling2);
				}
				if self.circle_particles[circ_2_index].get_id() == &pending.sibling2
				{
					break self.circle_particles.remove(circ_2_index);
				}
				circ_2_index+= 1;
			};
			match collider
			{
				Some(t) => 
				{
					dist = t.get_spring_contact(rect.get_curr(), circ1.get_curr(), circ2.get_curr());
				}
				None=>
				{

				}
			}
			if circ1.get_fixed()
			{
				if c2 <= constraint.get_fixed_end_limit() 
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / c2, pending.mtd.y / c2);
					circ2.set_curr(&circ2.get_position().plus(&lambda));
					circ2.set_velocity(&pending.vel);
				}
			}
			else if circ2.get_fixed()
			{
				if c1 <= constraint.get_fixed_end_limit() 
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / c1, pending.mtd.y / c1);
					circ1.set_curr(&circ1.get_position().plus(&lambda));
					circ1.set_velocity(&pending.vel);
				}
				
			}
			else
			{
				let denom = c1 * c1 + c2 * c2;
				if denom == 0.0
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / denom, pending.mtd.y / denom);
			
					circ1.set_curr(&circ1.get_curr().plus(&lambda.mult(c1)));
					circ2.set_curr(&circ2.get_curr().plus(&lambda.mult(c2)));
				
					// if collision is in the middle of SCP set the velocity of both end particles
					if dist == 0.5 
					{
						circ1.set_velocity(&pending.vel);
						circ2.set_velocity(&pending.vel);
					
					// otherwise change the velocity of the particle closest to contact
					} else {
						if dist < 0.5
						{
							circ1.set_velocity(&pending.vel);
						}
						else
						{
							circ2.set_velocity(&pending.vel);
						} 
						
					}
					ret = true;
				}
				
			}
			self.circle_particles.push(circ1);
			self.circle_particles.push(circ2);

		}
		else if constraint.rect_circ
		{
			let c1 = 1.0 - dist;
			let c2 = dist.clone();
			let mut circ1Index = 0;
			let mut circ1 = loop
			{
				if circ1Index >= self.circle_particles.len()
				{
					panic!("line 236, collide_pending_spring, circ1Index");
				}
				if self.circle_particles[circ1Index].get_id() == &pending.sibling1 || self.circle_particles[circ1Index].get_id() == &pending.sibling2
				{
					break self.circle_particles.remove(circ1Index);
				}
				circ1Index+= 1;
			};
			let mut rect1Index:usize = 0;
			let mut rect1 = loop
			{
				if circ1Index >= self.rectangle_particles.len()
				{
					panic!("line 249, collide_pending_spring, rect1Index");
				}
				if self.rectangle_particles[rect1Index].get_id() == &pending.sibling2 || self.rectangle_particles[rect1Index].get_id() == &pending.sibling1
				{
					break self.rectangle_particles.remove(rect1Index);
				}
				rect1Index+= 1;
			};
			match collider
			{
				Some(t) => 
				{
					dist = t.get_spring_contact(rect.get_curr(), circ1.get_curr(), rect1.get_curr());
				}
				None=>
				{

				}
			}
			if circ1.get_fixed()
			{
				if c2 <= constraint.get_fixed_end_limit() 
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / c2, pending.mtd.y / c2);
					rect1.set_curr(&rect1.get_position().plus(&lambda));
					rect1.set_velocity(&pending.vel);
				}
			}
			else if rect1.get_fixed()
			{
				if c1 <= constraint.get_fixed_end_limit() 
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / c1, pending.mtd.y / c1);
					circ1.set_curr(&circ1.get_position().plus(&lambda));
					circ1.set_velocity(&pending.vel);
				}
				
			}
			else
			{
				let denom = c1 * c1 + c2 * c2;
				if denom == 0.0
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / denom, pending.mtd.y / denom);
			
					circ1.set_curr(&circ1.get_curr().plus(&lambda.mult(c1)));
					rect1.set_curr(&rect1.get_curr().plus(&lambda.mult(c2)));
				
					// if collision is in the middle of SCP set the velocity of both end particles
					if dist == 0.5 
					{
						circ1.set_velocity(&pending.vel);
						rect1.set_velocity(&pending.vel);
					
					// otherwise change the velocity of the particle closest to contact
					} else {
						if dist < 0.5
						{
							circ1.set_velocity(&pending.vel);
						}
						else
						{
							rect1.set_velocity(&pending.vel);
						} 
						
					}
					ret = true;
				}
				
			}
			self.circle_particles.push(circ1);
			self.rectangle_particles.push(rect1);
		}
		else if constraint.rect_rect
		{
			let c1 = 1.0 - dist;
			let c2 = dist.clone();
			let mut rect1Index = 0;
			let mut rect1 = loop
			{
				if rect1Index >= self.rectangle_particles.len()
				{
					panic!("line 249, collide_pending_spring, rect1Index");
				}
				if self.rectangle_particles[rect1Index].get_id() == &pending.sibling1
				{
					break self.rectangle_particles.remove(rect1Index);
				}
				rect1Index+= 1;
			};
			let mut rect2Index:usize = 0;
			let mut rect2 = loop
			{
				if self.rectangle_particles[rect2Index].get_id() == &pending.sibling2
				{
					break self.rectangle_particles.remove(rect2Index);
				}
				rect2Index+= 1;
			};
			match collider
			{
				Some(t) => 
				{
					dist = t.get_spring_contact(rect.get_curr(), rect1.get_curr(), rect2.get_curr());
				}
				None=>
				{

				}
			}
			if rect1.get_fixed()
			{
				if c2 <= constraint.get_fixed_end_limit() 
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / c2, pending.mtd.y / c2);
					rect2.set_curr(&rect2.get_position().plus(&lambda));
					rect2.set_velocity(&pending.vel);
				}
			}
			else if rect2.get_fixed()
			{
				if c1 <= constraint.get_fixed_end_limit() 
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / c1, pending.mtd.y / c1);
					rect1.set_curr(&rect1.get_position().plus(&lambda));
					rect1.set_velocity(&pending.vel);
				}
				
			}
			else
			{
				let denom = c1 * c1 + c2 * c2;
				if denom == 0.0
				{
					ret = true;
				}
				else
				{
					let lambda = Vector::new(pending.mtd.x / denom, pending.mtd.y / denom);
			
					rect1.set_curr(&rect1.get_curr().plus(&lambda.mult(c1)));
					rect2.set_curr(&rect2.get_curr().plus(&lambda.mult(c2)));
				
					// if collision is in the middle of SCP set the velocity of both end particles
					if dist == 0.5 
					{
						rect1.set_velocity(&pending.vel);
						rect2.set_velocity(&pending.vel);
					
					// otherwise change the velocity of the particle closest to contact
					} else {
						if dist < 0.5
						{
							rect1.set_velocity(&pending.vel);
						}
						else
						{
							rect2.set_velocity(&pending.vel);
						} 
						
					}
					ret = true;
				}
				
			}
			self.rectangle_particles.push(rect1);
			self.rectangle_particles.push(rect2);
		}
		self.poly_poly_constraints.insert(constraintIndex, constraint);
		self.rectangle_particles.insert(rectIndex, rect);
		
		if totalLen != self.rectangle_particles.len() + self.circle_particles.len() + self.poly_poly_constraints.len()
		{
			panic!(" Something happened... a particle or constraint has gone missing in the pending_spring function");
		}
		return ret;
	}
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

	pub fn get_rect_by_id(&mut self, i:&i64)->Option<&mut RectangleParticle>
	{
		for p in self.rectangle_particles.iter_mut()
		{
			if p.id == *i
			{
				return Option::from(p);
			}
		}
		return Option::None;
	}

	pub fn get_particle_by_id(&mut self, i:&i64)->Option<&mut Particle>
	{
		for p in self.rectangle_particles.iter_mut()
		{
			if p.id == *i
			{
				return Option::from(p as &mut Particle);
			}
		}
		for p in self.circle_particles.iter_mut()
		{
			if p.id == *i
			{
				return Option::from(p as &mut Particle);
			}
		}
		return Option::None;
	}

	
	pub fn get_constraint_by_id(&mut self, i:&i64)->Option<&mut PolyPolyConstraint>
	{
		for p in self.poly_poly_constraints.iter_mut()
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
	pub fn get_center(&mut self)->Vector
	{
		return self.center.clone();
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
		for p in self.circle_particles.iter_mut()
		{
			if !p.get_move_with_composite()
			{
				continue;
			}
			let mut c = &mut self.center;
			let mut d = &mut self.delta;
			let radius:f64 = p.get_center().distance(c);
			let angle:f64 = get_relative_angle(&mut d, &mut c, &mut p.get_center()) + angle_radians;
			p.set_px((angle.cos() * radius) + c.x);
			p.set_py((angle.sin() * radius) + c.y);
		}

		for p in self.rectangle_particles.iter_mut()
		{
			if !p.get_move_with_composite()
			{
				continue;
			}
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
		for poly in self.rectangle_particles.iter_mut()
		{
			poly.update(ap);	
		}
		for circ in self.circle_particles.iter_mut()
		{
			circ.update(ap);	
		}
	}

	pub fn find_pending_collision(&mut self)->Option<OwnerCollision>
	{
		for rect in self.rectangle_particles.iter_mut()
		{
			if rect.collision_pending
			{
				rect.collision_pending = false;
				return Some(rect.owner_col.clone());
			}
		}
		
		return None
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
			if c.pending
			{
				self.satisfy_pending_translations(c.translation.clone());
			}
			self.poly_poly_constraints.insert(i, c);
		}
	}	
	pub fn satisfy_pending_translations(&mut self, p:PendingTranslation)
	{
		for rect in self.rectangle_particles.iter_mut()
		{
			if rect.id == p.id
			{
				rect.set_position(&p.loc);
				rect.set_radian(p.radian);
				rect.set_velocity(&p.vel);
				break;
			}
		}
	}
	pub fn satisfy_constraint_rect_rect(&mut self,constraint: &mut PolyPolyConstraint, _ap:&APValues)
	{
		let tuple = constraint.get_particles();
		let mut _length:usize = self.rectangle_particles.len();
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
			if self.circle_particles[i].get_id() == &tuple.0 || self.circle_particles[i].get_id() == &tuple.1
			{
				break self.circle_particles.remove(i);
			}
			i+= 1;
				if i >= self.circle_particles.len()
			{
				panic!("Couldn't find circle!");
			}
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
			if i >= self.rectangle_particles.len()
			{
				panic!("Couldn't find rectangle!");
			}
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
			if !p.get_collidable() || !p.get_collide_internal()
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
			if !p.get_collidable() || !p.get_collide_internal()
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
			if !p.get_collidable() || !p.get_collide_internal()
			{
				self.circle_particles.insert(i, p);
				continue;
			}
			let vec = &mut self.circle_particles;
			collision_detector::check_circ_vs_circ(&mut p, vec, ap);
			self.circle_particles.insert(i, p);
		}
	}

	pub fn check_circ_rect_internal_collisions(&mut self, ap:&APValues)
	{
		
		let length:usize = self.circle_particles.len();
		
		for i in 0..length
		{
			let mut p = self.circle_particles.remove(i);
			if !p.get_collidable() || !p.get_collide_internal()
			{
				self.circle_particles.insert(i, p);
				continue;
			}
			let vec = &mut self.rectangle_particles;
			collision_detector::check_circ_vs_rects(&mut p, vec, ap);
			self.circle_particles.insert(i, p);
		}
	}

	
	pub fn check_internal_collisions(&mut self, ap:&APValues)
	{
		self.check_rect_rect_internal_collisions(ap);
		self.check_circ_circ_internal_collisions(ap);
		self.check_rect_circ_internal_collisions(ap);
		self.check_circ_rect_internal_collisions(ap);
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
