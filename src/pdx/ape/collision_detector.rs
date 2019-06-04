use crate::ap_engine::APValues;
use crate::circle_particle::CircleParticle;
use crate::collision_resolver;
use crate::interval::Interval;
use crate::particle::Particle;
use crate::rectangle_particle::RectangleParticle;
use crate::vector::Vector;
use std::f64;

pub fn test_rect_vs_rect(ra: &mut RectangleParticle, rb: &mut RectangleParticle) -> bool {
    ra.set_samp(ra.get_position());
    rb.set_samp(rb.get_position());
    let mut collision_normal: Vector = Vector::new(0.0, 0.0);
    let mut collision_depth: f64 = 1000000.0;
    for i in 0..2 {
        let axis_a = &ra.get_axe(i);
        let depth_a = test_intervals(ra.get_projection(axis_a), rb.get_projection(axis_a));
        let abs_a: f64 = depth_a.abs();
        if abs_a == 0.0 {
            return false;
        }
        let axis_b = &rb.get_axe(i);
        let depth_b = test_intervals(ra.get_projection(axis_b), rb.get_projection(axis_b));
        let abs_b: f64 = depth_b.abs();
        if abs_b == 0.0 {
            return false;
        }

        if abs_a < collision_depth.abs() || abs_b < collision_depth.abs() {
            let altb: bool = abs_a < abs_b;
            if altb {
                collision_normal.copy(axis_a);
            } else {
                collision_normal.copy(axis_b);
            }
            if altb {
                collision_depth = depth_a;
            } else {
                collision_depth = depth_b;
            }
        }
    }

    collision_resolver::resolve_collision_rect_rect(ra, rb, collision_normal, collision_depth);
    return true;
}

pub fn test_circ_vs_rect(circle: &mut CircleParticle, rect: &mut RectangleParticle) -> bool {
    circle.set_samp(circle.get_position());
    rect.set_samp(rect.get_position());
    let mut collision_normal: Vector = Vector::new(0.0, 0.0);
    let mut collision_depth: f64 = 1000000.0;
    let mut depths = Vec::new();
    for i in 0..2 {
        let axis_b = &rect.get_axe(i);
        let depth_b = test_intervals(circle.get_projection(axis_b), rect.get_projection(axis_b));
        let abs_b: f64 = depth_b.abs();
        if abs_b == 0.0 {
            return false;
        }

        if abs_b < collision_depth.abs() {
            collision_normal.copy(axis_b);
            collision_depth = depth_b;
        }
        depths.push(depth_b);
    }

    let r = circle.get_radius().clone();
    if depths[0].abs() < r && depths[1].abs() < r {
        let vert = closest_vertex_on_obb(circle.get_samp(), rect);
        collision_normal.copy(&vert.minus(&circle.get_samp()));
        let mag = collision_normal.magnitude();
        collision_depth = r - mag;
        if collision_depth > 0.0 {
            collision_normal.div_equals(mag);
        } else {
            return false;
        }
    }
    collision_resolver::resolve_collision_rect_circ(
        circle,
        rect,
        collision_normal,
        collision_depth,
    );
    return true;
}

pub fn closest_vertex_on_obb(p: Vector, r: &mut RectangleParticle) -> Vector {
    let d: Vector = p.minus(&r.get_samp());
    let mut q: Vector = r.get_samp().clone();

    for i in 0..2 {
        let mut dist: f64 = d.dot(&r.get_axe(i));

        if dist >= 0.0 {
            dist = r.get_extent(i);
        } else if dist < 0.0 {
            dist = -r.get_extent(i);
        }

        q.plus_equals(&r.get_axe(i).mult(dist));
    }
    return q;
}

pub fn test_circ_vs_circ(ra: &mut CircleParticle, rb: &mut CircleParticle) -> bool {
    ra.set_samp(ra.get_position());
    rb.set_samp(rb.get_position());
    let depth_a = test_intervals(ra.get_interval_x(), rb.get_interval_x());
    if depth_a == 0.0 {
        return false;
    }
    let depth_b = test_intervals(ra.get_interval_y(), rb.get_interval_y());
    if depth_b == 0.0 {
        return false;
    }
    let mut collision_normal: Vector = ra.get_position().minus(&rb.get_position());
    let mag = collision_normal.clone().magnitude();
    let collision_depth: f64 = ra.get_radius() + rb.get_radius() - mag;

    if collision_depth > 0.0 {
        collision_normal.div_equals(mag);
        collision_resolver::resolve_circle_circle(ra, rb, &collision_normal, collision_depth);
    }
    return true;
}

pub fn project_point_on_segment(v: &Vector, a: &Vector, b: &Vector) -> Vector {
    let a_v: Vector = v.minus(a);
    let a_b: Vector = b.minus(a);
    let mut t: f64 = (a_v.dot(&a_b)) / (a_b.dot(&a_b));

    if t < 0.0 {
        t = 0.0;
    } else if t > 1.0 {
        t = 1.0;
    }

    let point: Vector = a.plus(&a_b.mult(t));
    return point;
}

fn test_intervals(interval_a: &Interval, interval_b: &Interval) -> f64 {
    if interval_a.max < interval_b.min {
        return 0.0;
    }
    if interval_b.max < interval_a.min {
        return 0.0;
    }

    let len_a: f64 = interval_b.max - interval_a.min;
    let len_b: f64 = interval_b.min - interval_a.max;

    if len_a.abs() < len_b.abs() {
        return len_a;
    }
    return len_b;
}

pub fn check_rectangle_vs_rects(
    p: &mut RectangleParticle,
    col: &mut Vec<RectangleParticle>,
    _ap: &APValues,
) {
    let length2: usize = col.len();

    for j in 0..length2 {
        let mut p2 = col.remove(j);
        if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed()) {
            col.insert(j, p2);
            continue;
        }
        test_rect_vs_rect(p, &mut p2);
        col.insert(j, p2);
    }
}

pub fn check_rectangle_vs_circs(
    p: &mut RectangleParticle,
    col: &mut Vec<CircleParticle>,
    _ap: &APValues,
) {
    let length2: usize = col.len();

    for j in 0..length2 {
        let mut p2 = col.remove(j);
        if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed()) {
            col.insert(j, p2);
            continue;
        }
        test_circ_vs_rect(&mut p2, p);
        col.insert(j, p2);
    }
}

pub fn check_circ_vs_circ(p: &mut CircleParticle, col: &mut Vec<CircleParticle>, _ap: &APValues) {
    let length2: usize = col.len();

    for j in 0..length2 {
        let mut p2 = col.remove(j);
        if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed()) {
            col.insert(j, p2);
            continue;
        }
        test_circ_vs_circ(&mut p2, p);
        col.insert(j, p2);
    }
}

pub fn check_circ_vs_rects(
    p: &mut CircleParticle,
    col: &mut Vec<RectangleParticle>,
    _ap: &APValues,
) {
    let length2: usize = col.len();

    for j in 0..length2 {
        let mut p2 = col.remove(j);
        if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed()) {
            col.insert(j, p2);
            continue;
        }
        test_circ_vs_rect(p, &mut p2);
        col.insert(j, p2);
    }
}
