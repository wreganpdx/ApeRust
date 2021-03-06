/*
William Regan
Tyler Pelham
Portland State University
Rust Programming 510
Final Project
*/

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
/**
 * ApeEngine.rs
 *
 * Summary: This is the core of the Ape Engine
 * Functions to impliment include, adding objects, stepping through physics simulations
 * and painting.
 * ore information, see https://exercism.io/my/tracks/rust
 */
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::ap_engine::APValues;
use crate::ap_engine::Paint;
use crate::collision::Collision;
use crate::interval::Interval;
use crate::owner_collision::OwnerCollision;
use crate::particle::Particle;
use crate::poly_poly_constraint::PolyPolyConstraint;
use crate::vector::Vector;
use std::any::Any;
use std::f64;

#[allow(unused_variables)]
#[derive(Default)]
pub struct RectangleParticle {
    pub id: i64,
    radian: f64,
    density: f64,
    axes: Vec<Vector>,
    extents: Vec<f64>,
    curr: Vector,
    prev: Vector,
    temp: Vector,
    samp: Vector,
    forces: Vector,
    velocity: Vector,
    mass: f64,
    friction: f64,
    elasticity: f64,
    rest_loops: i64,
    rest_count: i64,
    pinned: bool,
    pinned_to: Vec<Box<Particle>>, //to make this work, it had to be a vector, but only one should be stored here.
    pin: Vector,
    smashable: bool,
    max_exit_velocity: f64,
    at_rest: bool,
    left_max: f64,
    right_max: f64,
    multi_sampe: i64,
    coll: Collision,
    interval: Interval,
    kfr: f64,
    inv_mass: f64,
    fixed: bool,
    collidable: bool,
    width: f64,
    height: f64,
    created: bool,
    primary_color: [f32; 4],
    secondary_color: [f32; 4],
    owner: i64,
    owned: bool,
    sibling1: i64,
    sibling2: i64,
    pub owner_col: OwnerCollision,
    pub collision_pending: bool,
    width_scale: f64,
    collide_internal: bool,
    move_with_composite: bool,
    moved_flag: bool,
    visible: bool,
}

impl RectangleParticle {
    pub fn set_width_scale(&mut self, scale: f64) {
        if self.width_scale != scale {
            self.width_scale = scale;
            self.extents[0] = (scale * self.width) / 2.0;
        }
    }

    pub fn resolve_spring_collision(
        &mut self,
        mtd: &Vector,
        vel: &Vector,
        _n: &Vector,
        _d: f64,
        _o: i32,
        _p1: Option<&mut Particle>,
        _p2: Option<&mut Particle>,
        _collider: Option<&mut Particle>,
        _owner: &mut PolyPolyConstraint,
    ) {
        if !self.fixed && !self.owned {
            self.curr.plus_equals(mtd);
            self.get_velocity().copy(vel);
        }

        if self.smashable {
            let ev: f64 = vel.magnitude();
            if ev > self.max_exit_velocity {
                //note: These smash events are probably not necessary.
            }
        }
    }

    pub fn set_constraint(
        &mut self,
        owner: i64,
        sib1: i64,
        sib2: i64,
        pos1: &Vector,
        pos2: &Vector,
        height: f64,
    ) {
        self.owned = true;
        self.owner = owner;
        self.sibling1 = sib1;
        self.sibling2 = sib2;
        let length = pos1.distance(pos2);
        let angle = pos1.minus(pos2);
        self.radian = f64::atan2(angle.x, -angle.y) + (f64::consts::PI) * 0.5;
        self.create_rectangle(length, height);
        self.set_position(&pos1.plus(pos2).divided_by(2.0));
    }
    pub fn new(id: i64) -> RectangleParticle {
        let mut p = RectangleParticle::default();
        p.set_id(id);
        p.visible = true;
        return p;
    }

    pub fn get_extent(&mut self, i: usize) -> f64 {
        return self.extents[i].clone();
    }

    pub fn create_rectangle(&mut self, width: f64, height: f64) {
        if self.created {
            println!("Already created rectangle!!");
            return;
        }
        self.width = width;
        self.height = height;

        self.axes = Vec::new();
        self.axes.push(Vector::new(0.0, 0.0));
        self.axes.push(Vector::new(0.0, 0.0));
        self.collide_internal = true;
        self.extents.push(width / 2.0);
        self.extents.push(height / 2.0);
        //println!("rect init");
        self.set_radian(0.0);
        //println!("rect init complete");
        self.mass = 1.0;
        self.inv_mass = self.mass / 1.0;
        self.samp = Vector::new(0.0, 0.0);
        self.width_scale = 1.0;
        self.primary_color = [89.0 / 255.0, 86.0 / 255.0, 89.0 / 255.0, 1.0];
        self.secondary_color = [189.0 / 255.0, 186.0 / 255.0, 189.0 / 255.0, 1.0];
    }

    pub fn get_axe(&mut self, i: usize) -> Vector {
        return self.axes[i].clone();
    }
    fn closest_pt_segment_segment(
        &self,
        pp2: &Vector,
        pq2: &Vector,
        pp1: &Vector,
        pq1: &Vector,
    ) -> f64 {
        let d1 = pq1.minus(pp1);
        let d2 = pq2.minus(pp2);
        let r = pp1.minus(pp2);
        let a = d1.dot(&d1);
        let e = d2.dot(&d2);
        let f = d2.dot(&r);
        let c = d1.dot(&r);
        let b = d1.dot(&d2);
        let denom = a * e - b * b;

        let mut s = 0.5;
        if denom != 0.0 {
            s = (b * f - c * e) / denom;
            if s > 1.0 {
                s = 1.0;
            } else if s < 0.0 {
                s = 0.0;
            }
        }
        let mut t = (b * s + f) / e;

        if t < 0.0 {
            t = 0.0;
            s = -c / a;
            if s > 1.0 {
                s = 1.0
            } else if s < 0.0 {
                s = 0.0
            }
        } else if t > 0.0 {
            t = 1.0;
            s = (b - c) / a;
            if s > 1.0 {
                s = 1.0
            } else if s < 0.0 {
                s = 0.0
            }
        }
        let c1 = pp1.plus(&d1.mult(s));
        let c2 = pp2.plus(&d2.mult(t));
        let c1mc2: Vector = c1.minus(&c2);
        return c1mc2.dot(&c1mc2);
    }
    fn get_corners(&self, i: i32) -> (Vector, Vector) {
        let rx = self.curr.x.clone();
        let ry = self.curr.y.clone();
        let mut rca = Vector::new(0.0, 0.0);
        let mut rcb = Vector::new(0.0, 0.0);

        let ae0_x = self.axes[0].x * self.extents[0];
        let ae0_y = self.axes[0].y * self.extents[0];
        let ae1_x = self.axes[1].x * self.extents[1];
        let ae1_y = self.axes[1].y * self.extents[1];

        let emx = ae0_x - ae1_x;
        let emy = ae0_y - ae1_y;
        let epx = ae0_x + ae1_x;
        let epy = ae0_y + ae1_y;

        if i == 0 {
            // 0 and 1
            rca.x = rx - epx;
            rca.y = ry - epy;
            rcb.x = rx + emx;
            rcb.y = ry + emy;
        } else if i == 1 {
            // 1 and 2
            rca.x = rx + emx;
            rca.y = ry + emy;
            rcb.x = rx + epx;
            rcb.y = ry + epy;
        } else if i == 2 {
            // 2 and 3
            rca.x = rx + epx;
            rca.y = ry + epy;
            rcb.x = rx - emx;
            rcb.y = ry - emy;
        } else if i == 3 {
            // 3 and 0
            rca.x = rx - emx;
            rca.y = ry - emy;
            rcb.x = rx - epx;
            rcb.y = ry - epy;
        }
        return (rca, rcb);
    }
}

impl Paint for RectangleParticle {
    fn paint(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        if !self.visible {
            return;
        }
        use graphics::*;
        let width = self.width * self.width_scale;
        let rect = rectangle::rectangle_by_corners(0.0, 0.0, width, self.get_height());

        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(self.get_curr_x(), self.get_curr_y())
                .rot_rad(self.get_radian().clone())
                .trans(-width / 2.0, -self.get_height() / 2.0);
            rectangle(self.primary_color, rect, transform, gl);
        });
    }
}

impl PartialEq for RectangleParticle {
    fn eq(&self, other: &RectangleParticle) -> bool {
        self.id == other.id
    }
}

impl Particle for RectangleParticle {
    fn set_visible(&mut self, b: bool) {
        self.visible = b;
    }
    fn get_moved_flag(&self) -> bool {
        return self.moved_flag.clone();
    }
    fn get_move_with_composite(&self) -> bool {
        return self.move_with_composite.clone();
    }
    fn set_move_with_composite(&mut self, b: bool) {
        self.move_with_composite = b;
    }
    fn get_collide_internal(&self) -> bool {
        return self.collide_internal.clone();
    }
    fn set_collide_internal(&mut self, b: bool) {
        self.collide_internal = b;
    }
    fn get_spring_contact(&self, _center: &Vector, vec1: &Vector, vec2: &Vector) -> f64 {
        let mut shortest_distance = 10000000.0;
        for i in 0..4 {
            let tuple = self.get_corners(i);

            let d = self.closest_pt_segment_segment(&tuple.0, &tuple.1, vec1, vec2);
            if d < shortest_distance {
                shortest_distance = d;
            }
        }
        return shortest_distance;
    }
    fn set_id(&mut self, i: i64) {
        self.id = i;
    }
    fn get_id(&self) -> &i64 {
        return &self.id;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_mass(&self) -> f64 {
        if self.fixed {
            return 17976931348623157.0;
        } else {
            return self.mass;
        }
    }
    fn set_mass(&mut self, m: f64) {
        self.mass = m;
        self.inv_mass = 1.0 / self.mass;
    }

    fn get_elasticity(&self) -> f64 {
        return self.elasticity.clone();
    }
    fn set_elasticity(&mut self, e: f64) {
        self.elasticity = e;
    }

    fn get_curr(&self) -> &Vector {
        return &self.curr;
    }
    fn set_curr(&mut self, c: &Vector) {
        self.moved_flag = true;
        self.curr.copy(c);
    }

    fn get_position(&self) -> Vector {
        return self.curr.clone();
    }
    fn set_position(&mut self, c: &Vector) {
        self.moved_flag = true;
        self.curr.copy(c);
        self.prev.copy(c);
    }

    fn get_prev(&self) -> &Vector {
        return &self.prev;
    }
    fn set_prev(&mut self, p: &Vector) {
        self.prev.copy(p);
    }

    fn get_px(&self) -> f64 {
        return self.curr.x;
    }
    fn set_px(&mut self, x: f64) {
        self.curr.x = x;
        self.prev.x = x;
    }

    fn get_py(&self) -> f64 {
        return self.curr.y;
    }
    fn set_py(&mut self, y: f64) {
        self.curr.y = y;
        self.prev.y = y;
    }

    fn get_curr_x(&self) -> f64 {
        return self.curr.x;
    }
    fn set_curr_x(&mut self, x: f64) {
        self.curr.x = x;
    }

    fn get_curr_y(&self) -> f64 {
        return self.curr.y;
    }
    fn set_curr_y(&mut self, y: f64) {
        self.curr.y = y;
    }

    fn get_samp(&self) -> Vector {
        return self.samp.clone();
    }
    fn set_samp(&mut self, s: Vector) {
        self.samp.copy(&s);
    }

    fn get_interval(&self) -> &Interval {
        return &self.interval;
    }
    fn set_interval(&mut self, i: Interval) {
        self.interval.max = i.max;
        self.interval.min = i.min;
    }

    fn get_temp(&self) -> Vector {
        return self.temp.clone();
    }
    fn set_temp(&mut self, t: &Vector) {
        self.temp.copy(t);
    }

    fn get_forces(&self) -> Vector {
        return self.forces.clone();
    }
    fn set_forces(&mut self, f: &Vector) {
        self.forces.copy(f);
    }

    fn get_collision(&self) -> &Collision {
        return &self.coll;
    }
    fn set_collision(&mut self, f: &Collision) {
        self.coll = f.clone();
    }

    fn get_axes_len(&mut self) -> usize {
        return self.axes.len();
    }
    fn get_axes(&mut self) -> &Vec<Vector> {
        return &self.axes;
    }

    fn set_axes(&mut self) {
        // println!("RADIAN {}", self.radian);
        let s = self.radian.sin();
        let c = self.radian.cos();
        // println!("s: {},c: {}", s, c);
        self.axes[0].set_to(c.clone(), s.clone());
        self.axes[1].set_to(-s, c);
        // println!("{:?}, {:?}", self.axes[0], self.axes[1]);
    }

    fn get_projection(&mut self, axis: &Vector) -> &Interval {
        let rad = self.extents[0].clone() * axis.dot(&self.axes[0]).abs()
            + self.extents[1].clone() * axis.dot(&self.axes[1]).abs();
        let mut projected = self.get_position();
        if !self.fixed {
            projected.plus_equals(&self.curr.minus(&self.prev));
        }
        let c = self.samp.dot(axis);
        self.interval.min = c - rad;
        self.interval.max = c + rad;
        return &self.interval;
    }

    fn get_kfr(&self) -> f64 {
        return self.kfr;
    }
    fn set_kfr(&mut self, kfr: f64) {
        self.kfr = kfr;
    }

    fn get_inv_mass(&self) -> f64 {
        if self.fixed {
            return 0.0;
        }
        return self.inv_mass;
    }

    fn get_friction(&self) -> f64 {
        return self.friction;
    }
    fn set_friction(&mut self, f: f64) {
        self.friction = f;
    }

    fn get_fixed(&self) -> bool {
        return self.fixed;
    }
    fn set_fixed(&mut self, f: bool) {
        self.fixed = f;
    }

    fn get_collidable(&self) -> bool {
        return self.collidable.clone();
    }
    fn set_collidable(&mut self, c: bool) {
        self.collidable = c;
    }

    fn get_pinned(&self) -> bool {
        return self.pinned;
    }
    fn set_pinned(&mut self, f: bool) {
        if f {
            return;
        }
        self.pinned = false;
        //possibly do more here?
        /*
        _pinned = false;
            _pinnedTo = null;
            _pin = null;*/
    }

    fn get_pinned_to(&self) -> &Particle {
        return self;
    }

    fn set_pinned_to(&mut self, _p: &Particle, v: &Vector) {
        self.pinned = true;
        self.pin = v.clone();
        //self.pinned_to = p;
    }

    fn set_pin(&mut self, v: Vector) {
        self.pin = v;
    }

    fn get_pin(&self) -> Vector {
        return self.pin.clone();
    }

    fn get_center(&self) -> Vector {
        return self.curr.clone();
    }
    fn set_center(&mut self, c: Vector) {
        self.curr = c.clone();
    }

    fn get_multi_sample(&self) -> i64 {
        return self.multi_sampe;
    }
    fn set_multi_sample(&mut self, i: i64) {
        self.multi_sampe = i;
    }

    fn get_smashable(&self) -> bool {
        return self.smashable;
    }
    fn set_smashable(&mut self, i: bool) {
        self.smashable = i;
    }

    fn get_max_exit_velocity(&self) -> f64 {
        return self.max_exit_velocity;
    }
    fn set_max_exit_velocity(&mut self, ev: f64) {
        self.max_exit_velocity = ev;
    }

    fn get_velocity(&self) -> Vector {
        return self.curr.minus(&self.prev);
    }
    fn set_velocity(&mut self, i: &Vector) {
        self.prev.copy(&self.curr.minus(i));
    }

    fn get_at_rest(&self) -> bool {
        return self.at_rest;
    }
    fn set_at_rest(&mut self, i: bool) {
        self.at_rest = i;
    }

    fn get_rest_loops(&self) -> i64 {
        return self.rest_loops;
    }
    fn set_rest_loops(&mut self, rl: i64) {
        self.rest_loops = rl;
    }

    fn get_rest_count(&self) -> i64 {
        return self.rest_count;
    }
    fn set_rest_count(&mut self, rc: i64) {
        self.rest_count = rc;
    }

    fn get_left_max_x(&self) -> f64 {
        return self.left_max;
    }
    fn set_left_max_x(&mut self, lm: f64) {
        self.left_max = lm;
    }

    fn get_right_max_x(&self) -> f64 {
        return self.right_max;
    }
    fn set_right_max_x(&mut self, rm: f64) {
        self.right_max = rm;
    }

    fn add_force(&mut self, f: Vector) {
        self.forces.plus_equals(&f.mult(self.inv_mass));
    }

    fn add_massless_force(&mut self, f: Vector) {
        self.forces.plus_equals(&f);
    }

    fn update(&mut self, ap: &APValues) {
        if self.fixed {
            //self.moved_flag = false;
            return;
        }

        // global forces
        self.add_force(ap.force.clone());
        self.add_massless_force(ap.massless_force.clone());

        // integrate
        self.set_temp(&self.get_position());
        let nv = self
            .get_velocity()
            .plus(&self.get_forces().mult(ap.time_step));
        self.curr.plus_equals(&nv);

        self.set_prev(&self.get_temp());

        // clear the forces
        self.forces.set_to(0.0, 0.0);

        if self.owned {}
    }

    fn get_components(&mut self, cn: &Vector) -> Collision {
        let vel: Vector = self.get_velocity();
        let vdotn: f64 = cn.dot(&vel);
        self.coll.vn = cn.mult(vdotn);
        self.coll.vt = vel.minus(&self.coll.vn);
        return self.coll.clone();
    }
    fn resolve_collision(
        &mut self,
        mtd: &Vector,
        vel: &Vector,
        _n: &Vector,
        _d: f64,
        _o: i32,
        p: i64,
    ) {
        if !self.fixed && !self.owned {
            self.curr.plus_equals(mtd);
            self.set_velocity(vel);
        }

        if self.owned {
            self.owner_col = OwnerCollision::new(
                mtd,
                vel,
                _n,
                _d,
                _o,
                p,
                self.id,
                self.owner,
                self.sibling1,
                self.sibling2,
            );
            self.collision_pending = true;
        }

        if self.smashable {
            let ev: f64 = vel.magnitude();
            if ev > self.max_exit_velocity {
                //note: These smash events are probably not necessary.
                //dispatchEvent(new SmashEvent(SmashEvent.COLLISION, ev));
            }
        }
    }

    fn resolve_velocities(&mut self, dv: Vector, _dw: f64, _normal: Vector) {
        if !self.fixed {
            self.set_velocity(&self.get_velocity().plus(&dv));
        }
    }

    fn get_inv_inertia(&self) -> f64 {
        return 0.0;
    }

    fn get_ang_velocity(&self) -> f64 {
        return 0.0;
    }

    fn set_ang_velocity(&mut self, _a: f64) {
        //do nothing
    }

    fn get_radian(&self) -> &f64 {
        return &self.radian;
    }

    fn set_radian(&mut self, r: f64) {
        let _r = r % ((f64::consts::PI) * 2.0);
        self.radian = _r;
        self.set_axes();
    }

    fn get_left_most_x_value(&self) -> f64 {
        return self.curr.x;
    }

    fn get_right_most_x_value(&self) -> f64 {
        return self.curr.x;
    }

    fn get_width(&self) -> f64 {
        return self.width;
    }

    fn get_height(&self) -> f64 {
        return self.height;
    }

    fn get_rotation(&self) -> f64 {
        println!("getting rotation");
        return (180.0 / f64::consts::PI) * self.get_radian();
    }

    fn set_primary_color(&mut self, c: [f32; 4]) {
        self.primary_color = c;
    }
    fn set_secondary_color(&mut self, c: [f32; 4]) {
        self.secondary_color = c;
    }
}
