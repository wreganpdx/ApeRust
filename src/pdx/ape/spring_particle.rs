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
    secondary_color:[f32; 4]
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

impl Particle for SpringParticle 
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
			return 17976931348623157.0;
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
/*
	fn get_parent(&self)-> Box<ParticleCollection>
	{
		return &self.collection;
	}

	This is probably never going to happen because Rust says you can't have an object reference in more than one place.

	fn set_parent(&mut self, pc:&ParticleCollection)
	{
		self.collection = Box::new(pc);
	}
	*/



	fn get_axes_len(&mut self)->usize
	{
		return self.axes.len();
	}
	fn get_axes(&mut self)->&Vec<Vector>
	{
			return &self.axes;
	}
	
	
	fn set_axes(&mut self)
	{
        println!("RADIAN {}", self.radian);
		let s = self.radian.sin();
		let c = self.radian.cos();
        println!("s: {},c: {}", s, c);
		self.axes[0].set_to(c.clone(), s.clone());
		self.axes[1].set_to(-s, c);
        println!("{:?}, {:?}", self.axes[0], self.axes[1]);
	}



	fn get_projection(&mut self, axis:&Vector)->&Interval 
	{
		let rad = self.extents[0].clone() * axis.dot(&self.axes[0]).abs() + self.extents[1].clone() * axis.dot(&self.axes[1]).abs();
        let mut projected = self.get_position();
        if !self.fixed
        {
            projected.plus_equals(&self.curr.minus(&self.prev));
        }
		//let next = self.curr.plus(&self.curr.minus(&self.prev));
		//let c2 = projected.dot(axis);
        let c = self.samp.dot(axis);
		//c.plus_equals(self.curr.minus(&self.prev));
		self.interval.min = c - rad;
		self.interval.max = c + rad;
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
        return self.collidable.clone();
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
        return self.velocity.clone();
    }
	fn set_velocity(&mut self, i:&Vector)
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
			
            self.velocity.mult_equals(ap.damping);
			
			// global forces
			self.add_force(ap.force.clone());
			self.add_massless_force(ap.massless_force.clone());
	
			// integrate
			self.set_temp(&self.get_position());
			
			self.velocity.plus_equals(&self.forces.mult(ap.time_step));
			self.curr.plus_equals(&self.velocity.mult(ap.time_step));
			self.set_prev(&self.get_temp());

			// clear the forces
			self.forces.set_to(0.0,0.0);
    }

	fn get_components(&mut self, cn:&Vector)->Collision
    {
		let vel:Vector = self.get_velocity();
		let vdotn:f64 = cn.dot(&vel);
		self.coll.vn = cn.mult(vdotn);
		self.coll.vt = vel.minus(&self.coll.vn);	
		return self.coll.clone();
    }
	fn resolve_collision(&mut self, mtd:&Vector, vel:&Vector, _n:&Vector, _d:f64, _o:i32)
	{
		if !self.fixed
		{
            self.curr.plus_equals(mtd);
            self.velocity.copy(vel);
            /*
			self.curr.plus_equals(mtd);
            //println!("{:?}", self.velocity);
            let mag = self.velocity.mag_or_one();
            let mut newVel = mtd.clone().normalize().mult(mag);
            self.velocity.times_equals(&newVel);
            newVel.normalize_self();
            self.velocity.normalize_self();
            self.velocity.plus_equals(&newVel);
            self.velocity.normalize_self();
            self.velocity.mult_equals(mag);
           // self.velocity.mult_equals(0.33);
			//self.velocity.plus_equals(&newVel);
            */
            /*
            self.velocity.normalize();
            self.velocity.plus_equals(&new_vel);
            self.velocity.div_equals(2.0);
            self.velocity.mult_equals(mag * self.elasticity);
			let mag = vel.magnitude();
			let newVel = self.curr.clone().minus(&self.prev).mult(mag);
			
			self.prev = self.curr.clone();
        
			self.curr.plus_equals(mtd);
            let mut normalMTD = mtd.clone().normalize();
            normalMTD.mult_equals(mag);
			self.set_velocity(&vel);
            println!("vel: {:?}, mtd: {:?}",vel, mtd);
            */
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
	fn resolve_velocities(&mut self, dv:Vector, _dw:f64, _normal:Vector)
    {
		if !self.fixed
		{
			self.velocity = self.velocity.plus(&dv);
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

	fn set_ang_velocity(&mut self, _a:f64)
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
        println!("getting rotation");
		return (180.0/f64::consts::PI) * self.get_radian();
	}

    pub fn set_primary_color(&mut self, c:[f32;4])
    {
        self.primary_color = c;
    }
	pub fn set_secondary_color(&mut self, c:[f32;4])
    {
        self.secondary_color = c;
    }
}

