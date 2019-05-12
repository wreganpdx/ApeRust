/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * AbstractParticle.rs
 * 
 * Summary: This is the base class for Particles such as circle or rectangle
 * 
 * For more information, see  https://github.com/arctwelve/ape-js-port/tree/master/org/cove/ape
 */

use crate::vector::vector;
use crate::interval::interval;
use crate::collision::collision;
use crate::polygon_particle::polygon_particle;
use crate::particle_collection::particle_collection;
use crate::APEngine::APEngine;
use crate::APEngine::APValues;
use std::any::Any;

use std::default::Default;


pub enum shapes
{
	Polygon(polygon_particle),
}

#[allow(unused_variables)]
pub trait particle 
{

	fn get_axes_len(&mut self)->usize;
	fn get_interval_and_axe(&mut self, i:usize)->(vector, interval);

	fn as_any(&self) -> &dyn Any;
	
	fn get_axes(&mut self)->&Vec<vector>;
	fn set_axes(&mut self);

	fn orient_vertices(&mut self, r:f64);

	fn get_projection(&mut self, axis:&vector)->&interval;

    fn get_mass(&self)-> f64;
    fn set_mass(&mut self, m:f64);

	fn get_elasticity(&self)-> f64;
	fn set_elasticity(&mut self, e:f64);

	fn get_curr(&self)-> &vector;
	fn set_curr(&mut self, c:&vector);

	fn get_position(&self)-> vector;
	fn set_position(&mut self, c:&vector);

	fn get_prev(&self)-> &vector;
	fn set_prev(&mut self, p:&vector);

	fn get_px(&self)-> f64;
	fn set_px(&mut self, f:f64);

	fn get_py(&self)-> f64;
	fn set_py(&mut self, f:f64);

	fn get_curr_x(&self)-> f64;
	fn set_curr_x(&mut self, f:f64);

	fn get_curr_y(&self)-> f64;
	fn set_curr_y(&mut self, f:f64);

	fn get_samp(&self)-> vector;
	fn set_samp(&mut self, s:vector);

	fn get_interval(&self)-> &interval;
	fn set_interval(&mut self, i:interval);

	fn get_temp(&self)-> vector;
	fn set_temp(&mut self, t:&vector);

	fn get_forces(&self)-> vector;
	fn set_forces(&mut self, f:&vector);

	fn get_collision(&self)-> &collision;
	fn set_collision(&mut self, f:&collision);
/*
	fn get_parent(&self)-> Box<particle_collection>;
	
	fn set_parent(&mut self, pc:&particle_collection);
*/
	fn get_kfr(&self)-> f64;
	fn set_kfr(&mut self, kfr:f64);

	fn get_inv_mass(&self)-> f64;

	fn get_friction(&self)-> f64;
	fn set_friction(&mut self, f:f64);

	fn get_fixed(&self)-> bool;
	fn set_fixed(&mut self, f:bool);

	fn get_collidable(&self)-> bool;
	fn set_collidable(&mut self, f:bool);

	fn get_pinned(&self)-> bool;
	fn set_pinned(&mut self, f:bool);

	fn get_pinned_to(&self)-> &particle;
	fn set_pinned_to(&mut self, p:&particle, v:&vector);

	fn get_pin(&self)-> vector;
	fn set_pin(&mut self, p:vector);

	fn get_center(&self)-> vector;
	fn set_center(&mut self, c:vector);

	fn get_multi_sample(&self)-> i64;
	fn set_multi_sample(&mut self, i:i64);

	fn get_smashable(&self)-> bool;
	fn set_smashable(&mut self, i:bool);

	fn get_max_exit_velocity(&self)-> f64;
	fn set_max_exit_velocity(&mut self, ev:f64);

	fn get_velocity(&self)-> vector;
	fn set_velocity(&mut self, i:&vector);

	fn get_at_rest(&self)-> bool;
	fn set_at_rest(&mut self, i:bool);

	fn get_rest_loops(&self)-> i64;
	fn set_rest_loops(&mut self, rl:i64);

	fn get_rest_count(&self)-> i64;
	fn set_rest_count(&mut self, rc:i64);

	fn get_left_max_x(&self)-> f64;
	fn set_left_max_x(&mut self, lm:f64);

	fn get_right_max_x(&self)-> f64;
	fn set_right_max_x(&mut self, rm:f64);

	fn add_force(&mut self, f:vector);

	fn add_massless_force(&mut self, f:vector);

	fn update(&mut self,ap:&APValues);

	fn get_components(&mut self, cn:&vector)->collision;

	fn resolve_collision(&mut self, mtd:&vector, vel:&vector, n:&vector, d:f64, o:i32);
	//fn resolve_collision(mtd:vector, vel:vector, n:vector, d:f64, o:int, p:particle);

	fn resolve_velocities(&mut self, dv:vector, dw:f64, normal:vector);

	fn get_inv_inertia(&self)->f64;

	fn get_ang_velocity(&self)->f64;

	fn set_ang_velocity(&self, a:f64);

	fn get_radian(&self)->&f64;
	fn set_radian(&mut self, r:f64);

	fn get_left_most_x_value(&self)->f64;

	fn get_right_most_x_value(&self)->f64;

	fn get_width(&self)->f64;

	fn get_height(&self)->f64;

	fn get_rotation(&self)->f64;

	fn get_vertices(&mut self)->&Vec<vector>;

	fn get_vertices_and_position(&mut self)->(&Vec<vector>, vector);

	//fn resolve_collision(&mut self, mtd:vector, vel:vector, n:vector, d:f64, o:i64, p:&particle);

}


