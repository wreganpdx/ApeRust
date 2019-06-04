/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510
Final Project
*/

/**
 * collision.rs
 *
 * Summary: Simple collision for deciding the minimum and maximum for an object. Basically used to decide which objects may be about to collide.
 *
 * For more information, see  https://github.com/arctwelve/ape-js-port/tree/master/org/cove/ape
 */
//use std::fmt;
use crate::vector::Vector;

#[allow(unused_variables)]
#[derive(Default)]
pub struct Collision {
    pub vn: Vector,
    pub vt: Vector,
}

impl Collision {
    pub fn new(vn: Vector, vt: Vector) -> Collision {
        Collision { vn: vn, vt: vt }
    }

    pub fn clone(&self) -> Collision {
        return Collision::new(self.vn.clone(), self.vt.clone());
    }
}
