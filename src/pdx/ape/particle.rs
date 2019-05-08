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

pub trait particle
{

    fn get_mass(&self)-> f64;
    fn set_mass(&mut self, m:f64);

	fn get_elasticity(&self)-> f64;
	fn set_elasticity(&mut self, e:f64);

	fn get_curr(&self)-> vector;
	fn set_curr(&mut self, c:vector);

	fn get_position(&self)-> vector;
	fn set_position(&mut self, c:vector);

	fn get_prev(&self)-> vector;
	fn set_prev(&mut self, p:vector);

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

	fn get_interval()-> interval;
	fn set_interval(&mut self, i:interval);

	fn get_temp(&self)-> vector;
	fn set_temp(&mut self, t:vector);

	fn get_forces(&self)-> vector;
	fn set_forces(&mut self, f:vector);

	fn get_collision(&self)-> collision;
	fn set_collision(&mut self, f:collision);

	//fn get_parent()-> particle_collection;
	//fn set_parent(&mut self, pc:particle_collection);

	fn get_kfr(&self)-> f64;
	fn set_kfr(&mut self, kfr:f64);

	fn get_inv_mass(&self)-> f64;
	fn set_inv_mass(&mut self, im:f64);

	fn get_friction(&self)-> f64;
	fn set_friction(&mut self, f:f64);

	fn get_fixed(&self)-> bool;
	fn set_fixed(&mut self, f:bool);

	fn get_collidable(&self)-> bool;
	fn set_collidable(&mut self, f:bool);

	fn get_pinned(&self)-> bool;
	fn set_pinned(&mut self, f:bool);

	//fn get_pinned_to()-> particle;
	//fn set_pinned_to(&mut self, p:particle);

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
	fn set_velocity(&mut self, i:vector);

	fn get_at_rest(&self)-> bool;
	fn set_at_rest(&mut self, i:bool);

	fn get_rest_loops(&self)-> i64;
	fn set_rest_loops(&mut self, rl:i64);

	fn get_rest_count(&self)-> i64;
	fn set_rest_count(&mut self, rc:i64);

	fn get_left_max_x(&self)-> i64;
	fn set_left_max_x(&mut self, lm:i64);

	fn get_right_max_x(&self)-> i64;
	fn set_right_max_x(&mut self, rm:i64);

	fn add_force(&mut self, f:vector);

	fn add_massless_force(&mut self, f:vector);

	fn update(dt2:f64);

	fn get_components(cn:vector)->collision;

	//fn resolve_collision(mtd:vector, vel:vector, n:vector, d:f64, o:int, p:particle);

	fn resolve_velocities(dv:vector, dw:f64, normal:vector);

	fn get_inv_inertia(&self)->f64;

	fn get_ang_velocity(&self)->f64;

	fn set_ang_velocity(a:f64);

	fn get_radian(&self)->f64;

	fn get_left_most_x_value(&self)->f64;

	fn get_right_most_x_value(&self)->f64;

}


