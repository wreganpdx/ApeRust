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

use crate::vector::Vector;
use crate::interval::Interval;
use crate::collision::Collision;
use crate::ap_engine::APValues;
use std::any::Any;

//use std::default::Default;




#[allow(unused_variables)]
pub trait Particle 
{
	fn get_moved_flag(&self)->bool;
	fn get_collide_internal(&self)->bool;
	fn set_collide_internal(&mut self, b:bool);
	fn get_move_with_composite(&self)->bool;
	fn set_move_with_composite(&mut self, b:bool);
	fn get_spring_contact(&self, center:&Vector, vec1:&Vector, vec2:&Vector)->f64;
	fn set_primary_color(&mut self, c:[f32;4]);
	fn set_secondary_color(&mut self, c:[f32;4]);
	fn set_id(&mut self, i:i64);
	fn get_id(&self)->&i64;
	fn get_axes_len(&mut self)->usize;

	fn as_any(&self) -> &dyn Any;
	
	fn get_axes(&mut self)->&Vec<Vector>;
	fn set_axes(&mut self);


	fn get_projection(&mut self, axis:&Vector)->&Interval;

    fn get_mass(&self)-> f64;
    fn set_mass(&mut self, m:f64);

	fn get_elasticity(&self)-> f64;
	fn set_elasticity(&mut self, e:f64);

	fn get_curr(&self)-> &Vector;
	fn set_curr(&mut self, c:&Vector);

	fn get_position(&self)-> Vector;
	fn set_position(&mut self, c:&Vector);

	fn get_prev(&self)-> &Vector;
	fn set_prev(&mut self, p:&Vector);

	fn get_px(&self)-> f64;
	fn set_px(&mut self, f:f64);

	fn get_py(&self)-> f64;
	fn set_py(&mut self, f:f64);

	fn get_curr_x(&self)-> f64;
	fn set_curr_x(&mut self, f:f64);

	fn get_curr_y(&self)-> f64;
	fn set_curr_y(&mut self, f:f64);

	fn get_samp(&self)-> Vector;
	fn set_samp(&mut self, s:Vector);

	fn get_interval(&self)-> &Interval;
	fn set_interval(&mut self, i:Interval);

	fn get_temp(&self)-> Vector;
	fn set_temp(&mut self, t:&Vector);

	fn get_forces(&self)-> Vector;
	fn set_forces(&mut self, f:&Vector);

	fn get_collision(&self)-> &Collision;
	fn set_collision(&mut self, f:&Collision);
/*
	fn get_parent(&self)-> Box<ParticleCollection>;
	
	fn set_parent(&mut self, pc:&ParticleCollection);
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

	fn get_pinned_to(&self)-> &Particle;
	fn set_pinned_to(&mut self, p:&Particle, v:&Vector);

	fn get_pin(&self)-> Vector;
	fn set_pin(&mut self, p:Vector);

	fn get_center(&self)-> Vector;
	fn set_center(&mut self, c:Vector);

	fn get_multi_sample(&self)-> i64;
	fn set_multi_sample(&mut self, i:i64);

	fn get_smashable(&self)-> bool;
	fn set_smashable(&mut self, i:bool);

	fn get_max_exit_velocity(&self)-> f64;
	fn set_max_exit_velocity(&mut self, ev:f64);

	fn get_velocity(&self)-> Vector;
	fn set_velocity(&mut self, i:&Vector);

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

	fn add_force(&mut self, f:Vector);

	fn add_massless_force(&mut self, f:Vector);

	fn update(&mut self,ap:&APValues);

	fn get_components(&mut self, cn:&Vector)->Collision;

	fn resolve_collision(&mut self, mtd:&Vector, vel:&Vector, n:&Vector, d:f64, o:i32, p:i64);
	//fn resolve_collision(mtd:Vector, vel:Vector, n:Vector, d:f64, o:int, p:particle);

	fn resolve_velocities(&mut self, dv:Vector, dw:f64, normal:Vector);

	fn get_inv_inertia(&self)->f64;

	fn get_ang_velocity(&self)->f64;

	fn set_ang_velocity(&mut self, a:f64);

	fn get_radian(&self)->&f64;
	fn set_radian(&mut self, r:f64);

	fn get_left_most_x_value(&self)->f64;

	fn get_right_most_x_value(&self)->f64;

	fn get_width(&self)->f64;

	fn get_height(&self)->f64;

	fn get_rotation(&self)->f64;

	//fn resolve_collision(&mut self, mtd:Vector, vel:Vector, n:Vector, d:f64, o:i64, p:&particle);

}


