/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510 
Final Project
*/

/**
 * interval.rs
 * 
 * Summary: Simple interval for deciding the minimum and maximum for an object. Basically used to decide which objects may be about to collide.
 * 
 * For more information, see  https://github.com/arctwelve/ape-js-port/tree/master/org/cove/ape
 */
use std::fmt;
use crate::vector::vector;

#[allow(unused_variables)]
#[derive(Default)]
pub struct collision
{
    vn:vector,
    vt:vector,
}

impl collision
{
	pub fn new(vn:vector, vt:vector) -> collision 
    {
        collision { vn:vn, vt:vt}
    }
}
