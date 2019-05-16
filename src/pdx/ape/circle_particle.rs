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

use crate::rim_particle::rim_particle;
use crate::vector::vector;
use crate::interval::interval;
use crate::collision::collision;
use crate::particle::particle;
use crate::APEngine::APValues;
use crate::APEngine::Paint;
use std::any::Any;
use std::f64;

#[allow(unused_variables)]
#[derive(Default)]
pub struct circle_particle 
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
	radius:f64,
	is_wheel:bool,
	rim:rim_particle,
	traction:f64,
	norm_slip:vector,
}

impl circle_particle
{
	pub fn init_wheel(&mut self)
	{
		self.rim = rim_particle::new();
		self.rim.init(self.radius, 5.0);
		self.is_wheel = true;
	}
    pub fn new(id:i64)->circle_particle
    {
		let mut p = circle_particle::default();
		p.set_id(id);
        return p;
    }

	pub fn init_circle(&mut self, radius:f64)
	{
		self.width = radius*2.0;
		self.height = radius*2.0;

		self.set_radian(0.0);
		self.mass = 1.0;
		self.set_friction(0.5);
		self.inv_mass = self.mass/1.0;
		self.radius = radius;

	}

	pub fn get_interval_x(&mut self)->&interval
	{
		self.interval.min = self.curr.x - self.radius;
		self.interval.max = self.curr.x + self.radius;
		return &self.interval;
	}

	pub fn get_interval_y(&mut self)->&interval
	{
		self.interval.min = self.curr.y - self.radius;
		self.interval.max = self.curr.y + self.radius;
		return &self.interval;
	}

	pub fn get_radius(& mut self)->&f64
	{
		return &self.radius;
	}

	pub fn resolve_wheel(&mut self, mtd:&vector, vel:&vector, n:&vector, d:f64, o:i32)
	{
		
		let mut tan = self.rim.get_curr().swap_with_neg_y();
		tan.normalize();
		let wheel_surf_vel = tan.mult(self.rim.get_speed());
		
		let combined_vel = self.get_velocity().plus(&wheel_surf_vel);
		let cp = combined_vel.cross(n);
		tan.mult_equals(cp);
		let prev = &self.rim.get_curr().minus(&tan);
	 	self.rim.set_prev(prev);
		let slip_speed = (1.0-self.traction) * self.rim.get_speed();
		self.norm_slip.set_to(slip_speed * n.y , slip_speed * n.x);
		self.curr.plus_equals(&self.norm_slip);
		self.rim.damp_speed(self.traction);
	}
}

impl Paint for circle_particle
{
	fn paint(&mut self, args: &RenderArgs, gl:&mut GlGraphics)
	{
		use graphics::*;
		const BLUE:   [f32; 4] = [0.2, 0.2, 0.8, 1.0];
		const OFFBLUE:   [f32; 4] = [0.1, 0.1, 0.5, 1.0];
		let rect = rectangle::rectangle_by_corners(0.0, 0.0, 1.0, 1.0);
		if self.is_wheel
		{
			gl.draw(args.viewport(), |c, gl| 
			{
				let transform = c.transform.trans(self.get_curr_x(), self.get_curr_y()).rot_rad(self.get_radian().clone());
				//rectangle(BLUE, rect, transform, gl);
				circle_arc(BLUE, self.get_radius().clone(), 0.0,f64::consts::PI  /2.0, rect, transform, gl);
				circle_arc(OFFBLUE, self.get_radius().clone(), f64::consts::PI  /2.0,f64::consts::PI , rect, transform, gl);
				circle_arc(BLUE, self.get_radius().clone(),f64::consts::PI, f64::consts::PI  * 1.5, rect, transform, gl);
				circle_arc(OFFBLUE, self.get_radius().clone(), f64::consts::PI  * 1.5,f64::consts::PI  * 2.0, rect, transform, gl);
			});
		}
		else
		{
			gl.draw(args.viewport(), |c, gl| 
			{
				let transform = c.transform.trans(self.get_curr_x(), self.get_curr_y()).rot_rad(self.get_radian().clone());
				//rectangle(BLUE, rect, transform, gl);
				circle_arc(BLUE, self.get_radius().clone(), 0.0,f64::consts::PI  * 1.99, rect, transform, gl);
			});
		}
	}
}

impl PartialEq for circle_particle 
{
    fn eq(&self, other: &circle_particle) -> bool {
        self.id == other.id
    }
}

impl particle for circle_particle 
{
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
	


	fn get_projection(&mut self, axis:&vector)->&interval 
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
        return self.velocity.clone();
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
			//println!("velocity {:?}", self.velocity);
			self.velocity.mult_equals(ap.damping);
			self.velocity.plus_equals(&self.forces.mult(ap.time_step));
			//let mut nv:vector = self.velocity.plus(&self.forces.mult(ap.time_step));
			//println!("velocity {:?}, adding: {:?}", self.velocity,self.velocity.mult(ap.time_step));
			self.curr.plus_equals(&self.velocity.mult(ap.time_step));
			
			
			self.set_prev(&self.get_temp());

			// clear the forces
			self.forces.set_to(0.0,0.0);

			if self.is_wheel
			{
				self.rim.update(ap);
				self.radian = f64::atan2(self.rim.get_curr_x(), self.rim.get_curr_y()) + f64::consts::PI//Math.atan2(orientation.y, orientation.x) + Math.PI;
			}
    }

	fn get_components(&mut self, cn:&vector)->collision
    {
		let mut vel:vector = self.velocity.clone();
		let vdotn:f64 = cn.dot(&vel);
		self.coll.vn = cn.mult(vdotn);
		self.coll.vt = vel.minus(&self.coll.vn);	
		return self.coll.clone();
    }
	fn resolve_collision(&mut self, mtd:&vector, vel:&vector, n:&vector, d:f64, o:i32)
	{
		if !self.fixed
		{

			self.curr.plus_equals(mtd);
           // println!("{:?}", self.velocity);
            let mag = self.velocity.mag_or_one();
            let newVel = mtd.clone().normalize().mult(mag).mult(0.5);
            self.velocity.mult_equals(0.5);
			self.velocity.plus_equals(&newVel);
			/*
			self.curr.plus_equals(mtd);
            let mag = self.velocity.magnitude();
            let newVel = mtd.clone().normalize().mult(mag).mult(self.elasticity);
			self.velocity.copy(&newVel);
			*/
			//self.curr.plus_equals(mtd);
			//self.velocity.plus_equals(vel);
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
			self.resolve_wheel(mtd, vel, n, d, o);
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
/*
package org.cove.ape {
	
	import flash.display.Graphics;
	import flash.geom.Matrix;
	
	/**
	 * An n-sided polygon shaped particle. 
	 */ 
	public class PolygonParticle extends AbstractParticle {

		
		public function PolygonParticle(x:Number, 
				y:Number,
				width:Number, 
				height:Number,
				numVertices:int,
				rotation:Number = 0,
				fixedPosition:Boolean = false,
				mass:Number = 1, 
				elasticity:Number = 0.15,
				friction:Number = 0.1) {
				
				super(x, y, fixedPosition, mass, elasticity, friction);
				
				_numVertices = numVertices;
				createVertices(width, height);
				radian = rotation;
				
				//this.density = density;
		}
		
		internal function createVertices(width:Number, height:Number):void{
			_vertices = new Array();
			_originalVertices = new Array();
			
			var a:Number = Math.PI/numVertices;
			var da:Number = MathUtil.TWO_PI/numVertices;
			
			for(var i:int = 0; i < numVertices; i++){
				a+= da;
				_originalVertices.push(new Vector(Math.cos(a) * width, Math.sin(a) * height));
			}			
		}
		
		
		public override function get radian():Number {
			return _radian;
		}
		
		/**
		 * @private
		 */		
		public function set radian(t:Number):void {
			t = t % (MathUtil.TWO_PI);
			_radian = t;
			orientVertices(t);
			setAxes();
		}
		
		public function get angle():Number {
			return radian * MathUtil.ONE_EIGHTY_OVER_PI;
		}

		/**
		 * @private
		 */		
		public function set angle(a:Number):void {
			radian = a * MathUtil.PI_OVER_ONE_EIGHTY;
		}
		
		/**
		 * Sets up the visual representation of this PolygonParticle. This method is called 
		 * automatically when an instance of this PolygonParticle's parent Group is added to 
		 * the APEngine, when  this PolygonParticle's Composite is added to a Group, or the 
		 * PolygonParticle is added to a Composite or Group.
		 */				
		public override function init():void {
			cleanup();
			if (displayObject != null) {
				initDisplay();
			} else {
			
				sprite.graphics.clear();
				sprite.graphics.lineStyle(lineThickness, lineColor, lineAlpha);
				sprite.graphics.beginFill(fillColor, fillAlpha);
				sprite.graphics.moveTo(_originalVertices[0].x, _originalVertices[0].y);
				for(var i:int = 1; i < _originalVertices.length; i++){
					sprite.graphics.lineTo(_originalVertices[i].x, _originalVertices[i].y);
				}
				sprite.graphics.lineTo(_originalVertices[0].x, _originalVertices[0].y);
				sprite.graphics.endFill();
			}
			paint();
		}
		
		public override function paint():void {
			sprite.x = curr.x;
			sprite.y = curr.y;
			sprite.rotation = angle;
		}
		
		public function clearSprite():void{
			sprite.parent.removeChild(sprite);
		}
		
		internal function get vertices():Array{
			return _vertices;
		}
		
		internal function get numVertices():int{
			return _numVertices;
		}
		
		internal function set density(d:Number):void{
			_density = d;
			mass = calculateMass();
		}
		
		internal function get density():Number{
			return _density;
		}
		
		internal function calculateMass():Number{
			if(numVertices < 2){
				return 5 * density;
			}
			
			var m:Number = 0;
			var j:int = numVertices - 1;
			for (var i:int = 0; i < numVertices; i++){
				var P0:Vector = vertices[j];
				var P1:Vector = vertices[i];
				m += Math.abs(P0.cross(P1));
				j = i;
			}
			if(numVertices <= 2){
				m = 10;
			}
			m *= density * .5;
			return m;
		}
		
		internal function orientVertices(r:Number){
			for(var i:int = 0; i < _originalVertices.length; i++){
				_vertices[i] = _originalVertices[i].rotate(r);
			}
		}
		
		/**
		 * @private
		 */	
		internal function getProjection(axis:Vector):Interval {
			
			var c:Number = curr.dot(axis);
			
			var rad:Number = _vertices[0].dot(axis);
			var negRad:Number = rad;
			var posRad:Number = rad;
			
			for (var i:int = 1; i < _vertices.length; i++){
				rad = _vertices[i].dot(axis);
				if(rad < negRad){
					negRad = rad;
				}else if(rad > posRad){
					posRad = rad;
				}
			}
			
			interval.min = c + negRad;
			interval.max = c + posRad;
			
			return interval;
		}
		
		internal function getAxes():Array{
			return _axes;
		}
		
		internal function setAxes():void{
			_axes = new Array();
			var j:int = _numVertices - 1;
			for(var i:int = 0; i < _numVertices; i++){
				var e0:Vector = _vertices[j];
				var e1:Vector = _vertices[i];
				var e:Vector = e1.minus(e0);
				var currAxis:Vector = (new Vector(-e.y, e.x)).normalize();
				_axes.push(currAxis);
				j=i;
			}
		}
		
		internal function getClosestVertex(v:Vector):Vector{
			var d:Vector = v.minus(curr);
			var maxDist:Number = 0;
			var index:int = -1;
			
			for(var i:int = 0; i<_vertices.length; i++){
				var dist:Number = d.dot(_vertices[i]);
				if(dist > maxDist){
					maxDist = dist;
					index = i;
				}
			}
			return _vertices[index].plus(curr);
		}
		
		public override function leftMostXValue():Number{
			if(!isNaN(lmx) && fixedPosition) return lmx;
			
			var vx:Number = _vertices[0].x;
			lmx = vx;
			for(var i:int = 1; i < _vertices.length; i++){
				vx = _vertices[i].x;
				if( vx < lmx){
					lmx = vx;
				}
			}
			lmx += curr.x;
			return lmx;
		}
		
		public override function rightMostXValue():Number{
			if(!isNaN(rmx) && fixedPosition) return rmx;
			
			var vx:Number = _vertices[0].x;
			rmx = vx;
			for(var i:int = 1; i < _vertices.length; i++){
				vx = _vertices[i].x;
				if( vx > rmx){
					rmx = vx;
				}
			}
			rmx += curr.x;
			return rmx;
		}
	}
}

*/