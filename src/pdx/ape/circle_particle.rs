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

use crate::rim_particle::RimParticle;
use crate::vector::Vector;
use crate::interval::Interval;
use crate::collision::Collision;
use crate::particle::Particle;
use crate::ap_engine::APValues;
use crate::ap_engine::Paint;
use std::any::Any;
use std::f64;

#[allow(unused_variables)]
#[derive(Default)]
pub struct CircleParticle 
{
	pub id:i64,
    radian:f64,
    density:f64,
    original_vertices:Vec<Vector>,
    vertices:Vec<Vector>,
    num_vertices:usize,
    axes:Vec<Vector>,
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
	radius:f64,
	is_wheel:bool,
	rim:RimParticle,
	traction:f64,
	norm_slip:Vector,
    orientation:Vector,
    primary_color:[f32; 4],
    secondary_color:[f32; 4],
    last_delta:f64
}

impl CircleParticle
{
	pub fn init_wheel(&mut self, mt:f64)
	{
		self.rim = RimParticle::new();
		self.rim.init(self.radius, mt);
		//self.rim.max_torque = mt;
		self.friction = 0.0;
		self.set_traction(1.0);
		self.is_wheel = true;
        self.orientation = Vector::new(0.0,0.0);
	}
    pub fn new(id:i64)->CircleParticle
    {
		let mut p = CircleParticle::default();
		p.set_id(id);
        return p;
    }

	pub fn init_circle(&mut self, radius:f64)
	{
        self.primary_color = [0.2, 0.2, 0.8, 1.0];
		self.secondary_color =  [0.1, 0.1, 0.5, 1.0];
		self.width = radius*2.0;
		self.height = radius*2.0;

		self.set_radian(0.0);
		self.mass = 1.0;
		self.inv_mass = self.mass/1.0;
		self.radius = radius;
         self.primary_color =  [39.0/255.0, 36.0/255.0, 39.0/255.0, 1.0];
        self.secondary_color =  [139.0/255.0, 136.0/255.0, 139.0/255.0, 1.0];
	}

	pub fn get_interval_x(&mut self)->&Interval
	{
		self.interval.min = self.curr.x - self.radius;
		self.interval.max = self.curr.x + self.radius;
		return &self.interval;
	}

	pub fn get_interval_y(&mut self)->&Interval
	{
		self.interval.min = self.curr.y - self.radius;
		self.interval.max = self.curr.y + self.radius;
		return &self.interval;
	}

	pub fn get_radius(& mut self)->&f64
	{
		return &self.radius;
	}

	pub fn resolve_wheel(&mut self, n:Vector)
	{

		let mut tan = self.rim.get_curr().swap_with_neg_y();
		tan.normalize_self();
		let wheel_surf_vel = tan.mult(self.rim.get_speed()).mult(1.0-self.last_delta);
		let combined_vel = self.get_velocity().plus(&wheel_surf_vel);
		//let combined_vel = self.get_velocity().plus(&wheel_surf_vel);
		let cp = combined_vel.cross(&n);
		tan.mult_equals(cp);
		let prev = &self.rim.get_curr().minus(&tan);
	 	self.rim.set_prev(prev);
		let slip_speed = (1.0-self.traction) * self.rim.get_speed();
		
		self.norm_slip.set_to(slip_speed * n.y , slip_speed * n.x);
		//println!("normslip {:?}", self.norm_slip);
		self.curr.plus_equals(&self.norm_slip);
        
		self.rim.damp_speed(self.traction);
	}

	pub fn set_speed(&mut self, s:f64)
	{
		self.rim.speed = s;
	}

    
    pub fn get_traction(& self)->f64
    {
        return 1.0 - self.traction;
    }
    pub fn set_traction(&mut self, t:f64)
    {
        self.traction = 1.0-t;
    }
}

impl Paint for CircleParticle
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		use graphics::*;

		let rect = rectangle::rectangle_by_corners(0.0, 0.0, 1.0, 1.0);
		if self.is_wheel
		{
			gl.draw(args.viewport(), |c, gl| 
			{
				let transform = c.transform.trans(self.get_curr_x(), self.get_curr_y()).rot_rad(self.get_radian().clone());
				//rectangle(BLUE, rect, transform, gl);
				circle_arc(self.primary_color, self.get_radius().clone(), 0.0,f64::consts::PI  /2.0, rect, transform, gl);
				circle_arc(self.secondary_color, self.get_radius().clone(), f64::consts::PI  /2.0,f64::consts::PI , rect, transform, gl);
				circle_arc(self.primary_color, self.get_radius().clone(),f64::consts::PI, f64::consts::PI  * 1.5, rect, transform, gl);
				circle_arc(self.secondary_color, self.get_radius().clone(), f64::consts::PI  * 1.5,f64::consts::PI  * 2.0, rect, transform, gl);
			});
		}
		else
		{
			gl.draw(args.viewport(), |c, gl| 
			{
				let transform = c.transform.trans(self.get_curr_x(), self.get_curr_y()).rot_rad(self.get_radian().clone());
				//rectangle(BLUE, rect, transform, gl);
				circle_arc(self.primary_color, self.get_radius().clone(), 0.0,f64::consts::PI  * 1.99, rect, transform, gl);
			});
		}
	}
}

impl PartialEq for CircleParticle 
{
    fn eq(&self, other: &CircleParticle) -> bool {
        self.id == other.id
    }
}

impl Particle for CircleParticle 
{
    fn set_primary_color(&mut self, c:[f32;4])
    {
        self.primary_color = c;
    }
	fn set_secondary_color(&mut self, c:[f32;4])
    {
        self.secondary_color = c;
    }
	fn set_axes(&mut self)
	{
		
	}
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
        return self.mass.clone();
    }
    fn set_mass(&mut self, m:f64)
    {
        self.mass = m;
        self.inv_mass = 1.0/self.mass;
    }

	fn get_elasticity(&self)-> f64
    {
        return self.elasticity.clone();
    }
	fn set_elasticity(&mut self, e:f64)
    {
        self.elasticity = e;
    }

	fn get_curr(&self)-> &Vector
    {
        return &self.curr;
    }
	fn set_curr(&mut self, c:&Vector)
    {
        self.curr.copy(c);
    }

	fn get_position(&self)-> Vector
    {
        return self.curr.clone();
    }
	fn set_position(&mut self, c:&Vector)
    {
        self.curr.copy(c);
        self.prev.copy(c);
    }

	fn get_prev(&self)-> &Vector
    {
        return &self.prev;
    }
	fn set_prev(&mut self, p:&Vector)
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
    

	fn get_samp(&self)-> Vector
    {
        return self.samp.clone();
    }
	fn set_samp(&mut self, s:Vector)
    {
        self.samp.copy(&s);
    }

	fn get_interval(&self)-> &Interval
    {
        return &self.interval;
    }
	fn set_interval(&mut self, i:Interval)
    {
        self.interval.max = i.max;
        self.interval.min = i.min;
    }

	fn get_temp(&self)-> Vector
    {
        return self.temp.clone();
    }
	fn set_temp(&mut self, t:&Vector)
    {
        self.temp.copy(t);
    }

	fn get_forces(&self)-> Vector
    {
        return self.forces.clone();
    }
	fn set_forces(&mut self, f:&Vector)
    {
        self.forces.copy(f);
    }

	fn get_collision(&self)-> &Collision
    {
        return &self.coll;
    }
	fn set_collision(&mut self, f:&Collision)
    {
        self.coll = f.clone();
    }


	fn get_axes_len(&mut self)->usize
	{
		return self.axes.len();
	}
	fn get_axes(&mut self)->&Vec<Vector>
	{
			return &self.axes;
	}
	


	fn get_projection(&mut self, axis:&Vector)->&Interval 
	{
		let c:f64 = self.samp.dot(axis);
		self.interval.min = c -  self.radius;
		self.interval.max = c + self.radius;
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

	fn get_pinned_to(&self)-> &Particle
    {
        return self;
    }

	fn set_pinned_to(&mut self, _p:&Particle, v:&Vector)
    {
        self.pinned = true;
        self.pin = v.clone();
        //self.pinned_to = p;
    }

	fn set_pin(&mut self, v:Vector)
    {
        self.pin = v;
    }

	fn get_pin(&self)-> Vector
    {
        return self.pin.clone();
    }

	fn get_center(&self)-> Vector
    {
        return self.center.clone();
    }
	fn set_center(&mut self, c:Vector)
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

	fn get_velocity(&self)-> Vector
    {   
        return self.curr.minus(&self.prev);
    }
	fn set_velocity(&mut self, i:&Vector)
    {
        self.prev.copy(&self.curr.minus(i));
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

	fn add_force(&mut self, f:Vector)
    {
        self.forces.plus_equals(&f.mult(self.inv_mass));
    }

	fn add_massless_force(&mut self, f:Vector)
    {
        self.forces.plus_equals(&f);
    }

	fn update(&mut self, ap:&APValues)
    {
            if self.fixed
            {
                return;
            }
			
            self.last_delta = ap.time_step;
			// global forces
			self.add_force(ap.force.clone());
			self.add_massless_force(ap.massless_force.clone());
	
			// integrate
			self.set_temp(&self.get_position());
            let nv = self.get_velocity().plus(&self.get_forces().mult(ap.time_step));
            self.curr.plus_equals(&nv);
			
			
			self.set_prev(&self.get_temp());

			// clear the forces
			self.forces.set_to(0.0,0.0);

			if self.is_wheel
			{
				self.rim.update(ap);
                //self.orientation.set_to(self.rim.curr.get_x(), self.rim.curr.get_y());
				self.radian = f64::atan2(self.rim.get_curr_y(), self.rim.get_curr_x()) + f64::consts::PI//Math.atan2(orientation.y, orientation.x) + Math.PI;
			}
    }
    fn get_spring_contact(&self, center:&Vector, vec1:&Vector, vec2:&Vector)->f64
    {
        let ab = vec1.minus(vec2);
        let t = ab.dot(&center.minus(vec1)) / ab.dot(&ab);
        if t > 1.0
        {
            return 1.0;
        }
        else if t < 0.0
        {
            return 0.0;
        }
        return t;
    }
	fn get_components(&mut self, cn:&Vector)->Collision
    {
		let vel:Vector = self.get_velocity();
		let vdotn:f64 = cn.dot(&vel);
		self.coll.vn = cn.mult(vdotn);
		self.coll.vt = vel.minus(&self.coll.vn);	
		return self.coll.clone();
    }
	fn resolve_collision(&mut self, mtd:&Vector, vel:&Vector, n:&Vector, d:f64, o:i32, _p:i64)
	{
		if !self.fixed
		{
			self.curr.plus_equals(mtd);
            self.set_velocity(vel);
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

		if self.is_wheel
		{
            let mut val = 1.0;
            if d * (o as f64) < 0.0
            {
                val = -1.0;
            }
            let vec = n.mult(val);
			self.resolve_wheel(vec);
		}
			
	}
    
	fn resolve_velocities(&mut self, dv:Vector, _dw:f64, _normal:Vector)
    {
		if !self.fixed
		{
			self.set_velocity(&self.get_velocity().plus(&dv));
		}
    }

	fn get_inv_inertia(&self)->f64
    {	
		return 0.0;
    }

	fn get_ang_velocity(&self)->f64
    {
		if self.is_wheel
        {
            return self.rim.get_angular_velocity().clone();
        }
        return 0.0;
    }

	fn set_ang_velocity(&mut self, _a:f64)
    {
		if self.is_wheel
        {
            self.rim.set_angular_velocity(_a);
        }
    }

	fn get_radian(&self)->&f64
    {
		return &self.radian;
    }

	fn set_radian(&mut self, r:f64)
    {
		let _r = r % ((f64::consts::PI)*2.0);
		self.radian = _r;
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

}
