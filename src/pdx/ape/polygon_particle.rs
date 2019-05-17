/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * ApeEngine.rs
 * 
 * Summary: This is the core of the Ape Engine
 * Functions to impliment include, adding objects, stepping through physics simulations
 * and painting.
 * ore information, see https://exercism.io/my/tracks/rust
 */

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::{ GlGraphics};


use crate::vector::vector;
use crate::interval::interval;
use crate::collision::collision;
use crate::particle::particle;
use crate::APEngine::APValues;
use crate::APEngine::Paint;
use std::any::Any;
use std::f64;
use std::num::FpCategory::Infinite;
use num_traits;
use std::num::FpCategory;
use num_traits::float::FloatCore;

#[allow(unused_variables)]
#[derive(Default)]
pub struct polygon_particle 
{
	pub id:i64,
    radian:f64,
    density:f64,
    original_vertices:Vec<vector>,
    vertices:Vec<vector>,
    num_vertices:usize,
    axes:Vec<vector>,
    curr:vector,
    prev:vector,
    temp:vector,
    samp:vector,
    forces:vector,
    velocity:vector,
    mass:f64,
    friction:f64,
    elasticity:f64,
    rest_loops:i64,
    rest_count:i64,
    center:vector,
    pinned:bool,
    pinned_to: Vec<Box<particle>>,//to make this work, it had to be a vector, but only one should be stored here.
    pin:vector,
    smashable:bool,
    max_exit_velocity:f64,
    at_rest:bool,
    left_max:f64,
    right_max:f64,
    multi_sampe:i64,
    coll:collision,
    interval:interval, 
    kfr:f64,
    inv_mass:f64,
    fixed:bool,
    collidable:bool,
	width:f64,
	height:f64,
    primary_color:[f32; 4],
    secondary_color:[f32; 4]
}

impl polygon_particle
{
    pub fn new(id:i64)->polygon_particle
    {
		let mut p = polygon_particle::default();
		p.set_id(id);
        return p;
    }

	pub fn create_vertices_from_rect(&mut self, width:f64, height:f64)
	{
		self.width = width;
		self.height = height;
		self.vertices = Vec::new();
		self.original_vertices = Vec::new();
		
		self.original_vertices.push(vector::new(width/2.0, height/2.0));
		self.original_vertices.push(vector::new(-width/2.0, height/2.0));
		self.original_vertices.push(vector::new(-width/2.0, -height/2.0));
		self.original_vertices.push(vector::new(width/2.0, -height/2.0));
		
		self.num_vertices = self.original_vertices.len();

		self.axes = Vec::new();
		self.axes.push(vector::new(0.0,0.0));
		self.axes.push(vector::new(0.0,0.0));
		println!("rect init");
		self.set_radian(0.0);
		println!("rect init complete");
		self.mass = 1.0;
		self.set_friction(0.5);
		self.inv_mass = self.mass/1.0;
	}

	
	pub fn get_interval_and_axe(&mut self, i:usize)->(vector, interval)
	{
		//println!("made it here: {:?}", self.axes );
		let vec:vector = self.axes[i].clone();
		//println!("made it here: {:?}", self.axes );
		let c:f64 =  self.get_curr().dot(&vec);
		//println!("made it here: {:?}", self.axes );
		let mut rad:f64 = self.vertices[0].dot(&vec);
		//println!("made it here:");
		let mut neg_rad:f64 = rad;
		let mut pos_rad:f64 = rad;
		
		for i in 0..self.num_vertices
		{
			rad = self.vertices[i].dot(&vec);
			if rad < neg_rad {
				neg_rad = rad;
			}else if rad > pos_rad {
				pos_rad = rad;
			}
		}
		let interv:interval = interval::new(c + neg_rad,c + pos_rad );
		return (vec, interv);
		//self.interval.min = c + negRad;
		//self.interval.max = c + posRad;
	}

	pub fn orient_vertices(&mut self, r: f64)
	{
		let n = self.original_vertices.len();
		self.vertices = Vec::new();
		for i in 0..n
		{
			self.vertices.push(self.original_vertices[i].rotate(&r));
		}
	}

	pub fn get_vertices_and_position(&mut self)->(&Vec<vector>, vector)
	{
		return (&self.vertices, self.curr.clone());
	}

	pub fn get_vertices(&mut self)->&Vec<vector>
	{
		return &self.vertices;
	}
}

impl Paint for polygon_particle
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		use graphics::*;
		const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		let rect = rectangle::rectangle_by_corners(0.0, 0.0, self.get_width(), self.get_height());

		gl.draw(args.viewport(), |c, gl| 
		{
            let transform = c.transform.trans(self.get_curr_x(), self.get_curr_y()).rot_rad(self.get_radian().clone()).trans(-self.get_width()/2.0, -self.get_height()/2.0);
            rectangle(RED, rect, transform, gl);
        });
	}
}

impl PartialEq for polygon_particle 
{
    fn eq(&self, other: &polygon_particle) -> bool {
        self.id == other.id
    }
}

impl particle for polygon_particle 
{
	fn set_id(&mut self, i:i64)
	{
		self.id = i;
	}
	fn get_id(&self)->&i64
	{
		return &self.id;
	}
	fn as_any(&self) -> &dyn Any 
	{
        self
    }
    
    fn get_mass(&self)-> f64
    {
		if self.fixed {
			return num_traits::float::FloatCore::infinity();
		}
		else
		{
        	return self.mass;
		}
    }
    fn set_mass(&mut self, m:f64)
    {
        self.mass = m;
        self.inv_mass = 1.0/self.mass;
    }

	fn get_elasticity(&self)-> f64
    {
        return self.elasticity;
    }
	fn set_elasticity(&mut self, e:f64)
    {
        self.elasticity = e;
    }

	fn get_curr(&self)-> &vector
    {
        return &self.curr;
    }
	fn set_curr(&mut self, c:&vector)
    {
        self.curr.copy(c);
    }

	fn get_position(&self)-> vector
    {
        return self.curr.clone();
    }
	fn set_position(&mut self, c:&vector)
    {
        self.curr.copy(c);
        self.prev.copy(c);
    }

	fn get_prev(&self)-> &vector
    {
        return &self.prev;
    }
	fn set_prev(&mut self, p:&vector)
    {
         self.prev.copy(p);
    }

	fn get_px(&self)-> f64
    {
        return self.curr.x;
    }
	fn set_px(&mut self, x:f64)
    {
        self.curr.x = x;
        self.prev.x = x;
    }

	fn get_py(&self)-> f64
    {
        return self.curr.y;
    }
	fn set_py(&mut self, y:f64)
    {
        self.curr.y = y;
        self.prev.y = y;
    }

	fn get_curr_x(&self)-> f64
    {
        return self.curr.x;
    }
	fn set_curr_x(&mut self, x:f64)
    {
        self.curr.x = x;
    }

	fn get_curr_y(&self)-> f64
    {
        return self.curr.y;
    }
	fn set_curr_y(&mut self, y:f64)
    {
        self.curr.y = y;
    }
    

	fn get_samp(&self)-> vector
    {
        return self.samp.clone();
    }
	fn set_samp(&mut self, s:vector)
    {
        self.samp.copy(&s);
    }

	fn get_interval(&self)-> &interval
    {
        return &self.interval;
    }
	fn set_interval(&mut self, i:interval)
    {
        self.interval.max = i.max;
        self.interval.min = i.min;
    }

	fn get_temp(&self)-> vector
    {
        return self.temp.clone();
    }
	fn set_temp(&mut self, t:&vector)
    {
        self.temp.copy(t);
    }

	fn get_forces(&self)-> vector
    {
        return self.forces.clone();
    }
	fn set_forces(&mut self, f:&vector)
    {
        self.forces.copy(f);
    }

	fn get_collision(&self)-> &collision
    {
        return &self.coll;
    }
	fn set_collision(&mut self, f:&collision)
    {
        self.coll = f.clone();
    }
/*
	fn get_parent(&self)-> Box<particle_collection>
	{
		return &self.collection;
	}

	This is probably never going to happen because Rust says you can't have an object reference in more than one place.

	fn set_parent(&mut self, pc:&particle_collection)
	{
		self.collection = Box::new(pc);
	}
	*/


	fn get_axes_len(&mut self)->usize
	{
		return self.axes.len();
	}
	fn get_axes(&mut self)->&Vec<vector>
	{
			return &self.axes;
	}
	
	
	fn set_axes(&mut self)
	{
		self.axes = Vec::new();
		//println!("self.vertices.len() {} ", self.vertices.len() );
		let mut j:usize = self.num_vertices - 1;
		for i in 0..self.num_vertices
		{
			let e0:vector = self.vertices[j].clone();
			let e1:vector = self.vertices[i].clone();
			let e:vector = e1.minus(&e0);
			let curr_axis:vector = (vector::new(-e.y, e.x)).normalize();
			self.axes.push(curr_axis);
			j=i;
		}
	}



	fn get_projection(&mut self, axis:&vector)->&interval 
	{
		let c:f64 = self.curr.dot(axis);
		
		let mut rad:f64 = self.vertices[0].dot(axis);
		let mut neg_rad:f64 = rad;
		let mut pos_rad:f64 = rad;
		
		for i in 0..self.num_vertices
		{
			rad = self.vertices[i].dot(axis);
			if rad < neg_rad {
				neg_rad = rad;
			}else if rad > pos_rad {
				pos_rad = rad;
			}
		}
		let i:interval = interval::new(c + neg_rad,c + pos_rad );
		self.set_interval(i);
		//self.interval.min = c + negRad;
		//self.interval.max = c + posRad;
		
		return &self.interval;
	}

	fn get_kfr(&self)-> f64
    {
        return self.kfr;
    }
	fn set_kfr(&mut self, kfr:f64)
    {
        self.kfr = kfr;
    }

	fn get_inv_mass(&self)-> f64
    {
        if self.fixed
        {
            return 0.0;
        }
        return self.inv_mass;
    }


	fn get_friction(&self)-> f64
    {
        return self.friction;
    }
	fn set_friction(&mut self, f:f64)
    {
        self.friction = f;
    }

	fn get_fixed(&self)-> bool
    {
        return self.fixed;
    }
	fn set_fixed(&mut self, f:bool)
    {
        self.fixed = f;
    }

	fn get_collidable(&self)-> bool
    {
        return self.collidable;
    }
	fn set_collidable(&mut self, c:bool)
    {
        self.collidable = c;
    }

	fn get_pinned(&self)-> bool
    {
        return self.pinned;
    }
	fn set_pinned(&mut self, f:bool)
    {
        if f 
        {
            return;
            
        }
        self.pinned = false;
        //possibly do more here?
        /*
        _pinned = false;
			_pinnedTo = null;
			_pin = null;*/
    }

	fn get_pinned_to(&self)-> &particle
    {
        return self;
    }

	fn set_pinned_to(&mut self, p:&particle, v:&vector)
    {
        self.pinned = true;
        self.pin = v.clone();
        //self.pinned_to = p;
    }

	fn set_pin(&mut self, v:vector)
    {
        self.pin = v;
    }

	fn get_pin(&self)-> vector
    {
        return self.pin.clone();
    }

	fn get_center(&self)-> vector
    {
        return self.center.clone();
    }
	fn set_center(&mut self, c:vector)
    {
        self.center = c.clone();
    }

	fn get_multi_sample(&self)-> i64
    {
        return self.multi_sampe;
    }
	fn set_multi_sample(&mut self, i:i64)
    {
        self.multi_sampe = i;
    }

	fn get_smashable(&self)-> bool
    {
        return self.smashable;
    }
	fn set_smashable(&mut self, i:bool)
    {
        self.smashable = i;
    }

	fn get_max_exit_velocity(&self)-> f64
    {
        return self.max_exit_velocity;
    }
	fn set_max_exit_velocity(&mut self, ev:f64)
    {
        self.max_exit_velocity = ev;
    }

	fn get_velocity(&self)-> vector
    {   
        return self.curr.minus(&self.prev);
    }
	fn set_velocity(&mut self, i:&vector)
    {
        self.velocity.copy(i);
    }

	fn get_at_rest(&self)-> bool
    {
        return self.at_rest;
    }
	fn set_at_rest(&mut self, i:bool)
    {
        self.at_rest = i;
    }

	fn get_rest_loops(&self)-> i64
    {
        return self.rest_loops;
    }
	fn set_rest_loops(&mut self, rl:i64)
    {
        self.rest_loops = rl;
    }

	fn get_rest_count(&self)-> i64
    {
        return self.rest_count;
    }
	fn set_rest_count(&mut self, rc:i64)
    {
        self.rest_count = rc;
    }

	fn get_left_max_x(&self)-> f64
    {
        return self.left_max;
    }
	fn set_left_max_x(&mut self, lm:f64)
    {
        self.left_max = lm;
    }

	fn get_right_max_x(&self)-> f64
    {
        return self.right_max;
    }
	fn set_right_max_x(&mut self, rm:f64)
    {
        self.right_max = rm;
    }

	fn add_force(&mut self, f:vector)
    {
        self.forces.plus_equals(&f.mult(self.inv_mass));
    }

	fn add_massless_force(&mut self, f:vector)
    {
        self.forces.plus_equals(&f);
    }

	fn update(&mut self, ap:&APValues)
    {
            if self.fixed
            {
                return;
            }
			
			
			// global forces
			self.add_force(ap.force.clone());
			self.add_massless_force(ap.massless_force.clone());
	
			// integrate
			self.set_temp(&self.get_position());
			
			let mut nv:vector = self.velocity.plus(&self.forces.mult(ap.time_step));
			self.curr.plus_equals(&nv.mult(ap.damping));
			self.set_prev(&self.get_temp());

			// clear the forces
			self.forces.set_to(0.0,0.0);
    }

	fn get_components(&mut self, cn:&vector)->collision
    {
		let mut vel:vector = self.get_velocity();
		let vdotn:f64 = cn.dot(&vel);
		self.coll.vn = cn.mult(vdotn);
		self.coll.vt = vel.minus(&self.coll.vn);	
		return self.coll.clone();
    }
	fn resolve_collision(&mut self, mtd:&vector, vel:&vector, n:&vector, d:f64, o:i32)
	{
		if !self.fixed
		{
			self.prev = self.curr.clone();
			self.curr.plus_equals(mtd);
			self.set_velocity(&self.curr.minus(&self.prev));
		}
		
		if self.smashable
		{
			let ev:f64 = vel.magnitude();
			if ev > self.max_exit_velocity
			{
				//note: These smash events are probably not necessary.
				//dispatchEvent(new SmashEvent(SmashEvent.COLLISION, ev));
			}
		}
			
	}
	fn resolve_velocities(&mut self, dv:vector, dw:f64, normal:vector)
    {
		if !self.fixed
		{
			println!("Velocity old {:?}, velocity delta {:?}", self.velocity, dv);
			self.velocity = self.velocity.plus(&dv);
			println!("Velocity new {:?}, velocity delta {:?}", self.velocity, dv);
		}
    }

	fn get_inv_inertia(&self)->f64
    {	
		return 0.0;
    }

	fn get_ang_velocity(&self)->f64
    {
		return 0.0;
    }

	fn set_ang_velocity(&self, a:f64)
    {
		//do nothing
    }

	fn get_radian(&self)->&f64
    {
		return &self.radian;
    }

	fn set_radian(&mut self, r:f64)
    {
		let _r = r % ((f64::consts::PI)*2.0);
		self.radian = _r;
		println!("orienting_vertices");
		self.orient_vertices(_r);
		println!("setting axes_vertices");
		self.set_axes();
    }

	fn get_left_most_x_value(&self)->f64
    {
		return self.curr.x;
    }

	fn get_right_most_x_value(&self)->f64
    {
		return self.curr.x;
    }

	fn get_width(&self)->f64
	{
		return self.width
	}

	fn get_height(&self)->f64
	{
		return self.height;
	}

	fn get_rotation(&self)->f64
	{
		return (180.0/f64::consts::PI) * self.get_radian();
	}
	 fn set_primary_color(&mut self, c:[f32;4])
    {
        self.primary_color = c;
    }
	fn set_secondary_color(&mut self, c:[f32;4])
    {
        self.secondary_color = c;
    }
}
