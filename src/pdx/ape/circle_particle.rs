/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * CirclyParticle.rs
 * 
 * Summary: Circle Particle
 * 
 * 
 * For more information, see  https://github.com/arctwelve/ape-js-port/tree/master/org/cove/ape
 */

use crate::vector::vector;
use crate::interval::interval;
use crate::collision::collision;
use crate::particle::particle;
use crate::particle::shapes;
use crate::APEngine::APEngine;
use crate::particle_collection::particle_collection;
use std::any::Any;

#[allow(unused_variables)]
#[derive(Default)]
pub struct circle_particle 
{
    id:i64,
    radian:f64,
    density:f64,
    original_vertices:Vec<vector>,
    vertices:Vec<vector>,
    num_vertices:i64,
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
}

