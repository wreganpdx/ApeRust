use crate::circle_particle::CircleParticle;
use crate::collision::Collision;
use crate::collision_resolver;
use crate::particle::Particle;
use crate::poly_poly_constraint::PolyPolyConstraint;
use crate::rectangle_particle::RectangleParticle;
use crate::vector::Vector;
use std::f64;

pub fn resolve_circle_circle(
    pa: &mut CircleParticle,
    pb: &mut CircleParticle,
    normal: &Vector,
    depth: f64,
) {
    let im_pb_inv_mass: f64 = pb.get_inv_mass();
    let im_pa_inv_mass: f64 = pa.get_inv_mass();

    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());

    let mtd: Vector = normal.mult(depth);

    let mut te: f64 = pa.get_elasticity() + pb.get_elasticity();
    // te = collision_resolver::clamp(te, 0.0, 1.0);
    let mut tf: f64 = 1.0 - (pa.get_friction() + pb.get_friction());
    tf = collision_resolver::clamp(tf, 0.0, 1.0);

    let sum_inv_mass: f64 = im_pa_inv_mass + im_pb_inv_mass;

    let mut ca: Collision = pa.get_components(normal);
    let mut cb: Collision = pb.get_components(normal);

    let mult_b: &mut Vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2: &mut Vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b: &mut Vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b: Vector = plus_b.divided_by(sum_inv_mass);

    let mult_a: &mut Vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2: &mut Vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a: &mut Vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a: Vector = plus_a.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);

    let mtd_a: Vector = mtd.mult(im_pa_inv_mass / sum_inv_mass);
    let mtd_b: Vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);

    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, -1, pb.id.clone());
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth, 1, pa.id.clone());
}

pub fn resolve_collision_rect_rect(
    pa: &mut RectangleParticle,
    pb: &mut RectangleParticle,
    normal: Vector,
    depth: f64,
) {
    let im_pb_inv_mass: f64 = pb.get_inv_mass();
    let im_pa_inv_mass: f64 = pa.get_inv_mass();

    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());

    let mtd: Vector = normal.mult(depth);

    let mut te: f64 = pa.get_elasticity() + pb.get_elasticity();

    let mut tf: f64 = 1.0 - (pa.get_friction() + pb.get_friction());
    tf = collision_resolver::clamp(tf, 0.0, 1.0);

    let sum_inv_mass: f64 = im_pa_inv_mass + im_pb_inv_mass;

    let mut ca: Collision = pa.get_components(&normal);
    let mut cb: Collision = pb.get_components(&normal);

    let mult_a: &mut Vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2: &mut Vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a: &mut Vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a: Vector = plus_a.divided_by(sum_inv_mass);

    let mult_b: &mut Vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2: &mut Vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b: &mut Vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b: Vector = plus_b.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);

    let mtd_a: Vector = mtd.mult(im_pa_inv_mass / sum_inv_mass);
    let mtd_b: Vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);

    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, -1, pb.id.clone());
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth, 1, pa.id.clone());
}

pub fn resolve_collision_rect_circ(
    pa: &mut CircleParticle,
    pb: &mut RectangleParticle,
    normal: Vector,
    depth: f64,
) {
    let im_pb_inv_mass: f64 = pb.get_inv_mass();
    let im_pa_inv_mass: f64 = pa.get_inv_mass();

    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());

    let mtd: Vector = normal.mult(depth);

    let mut te: f64 = pa.get_elasticity() + pb.get_elasticity();
    // te = collision_resolver::clamp(te, 0.0, 1.0);
    let mut tf: f64 = 1.0 - (pa.get_friction() + pb.get_friction());
    tf = collision_resolver::clamp(tf, 0.0, 1.0);

    let sum_inv_mass: f64 = im_pa_inv_mass + im_pb_inv_mass;

    let mut ca: Collision = pa.get_components(&normal);
    let mut cb: Collision = pb.get_components(&normal);

    let mult_b: &mut Vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2: &mut Vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b: &mut Vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b: Vector = plus_b.divided_by(sum_inv_mass);

    let mult_a: &mut Vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2: &mut Vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a: &mut Vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a: Vector = plus_a.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);

    let mtd_a: Vector = mtd.mult(im_pa_inv_mass / sum_inv_mass);
    let mtd_b: Vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);

    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, -1, pb.id.clone());
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth, 1, pa.id.clone());
}
pub fn resolve_spring_collision(
    mtd: &Vector,
    vel: &Vector,
    _n: &Vector,
    _d: f64,
    _o: i32,
    stuff: (
        &mut PolyPolyConstraint,
        Option<&mut RectangleParticle>,
        Vec<Option<&mut Particle>>,
    ),
) {
}

pub fn clamp(mut t: f64, min: f64, max: f64) -> f64 {
    if t > max {
        t = max
    }
    if t < min {
        t = min
    }
    return t;
}
