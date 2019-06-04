extern crate ape_rust;
use ape_rust::rectangle_particle::RectangleParticle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::poly_poly_constraint::PolyPolyConstraint;
use ape_rust::vector::Vector;
use ape_rust::particle::Particle;

pub fn create_rectangle(part:&mut ParticleCollection, tuple:(i64,i64,i64,i64,i64,i64,i64,i64))
{
    let ctr = &part.get_center();
    let rw:f64 = 75.0;
    let rh:f64 = 4.0;
    let rad:f64 = 4.0;

    let mut cp_a = RectangleParticle::new(tuple.0);
    let mut cp_b = RectangleParticle::new(tuple.1);
    let mut cp_c = RectangleParticle::new(tuple.2);
    let mut cp_d = RectangleParticle::new(tuple.3);

    cp_a.set_collidable(true);
    cp_b.set_collidable(true);

    cp_c.set_collidable(true);

    cp_d.set_collidable(true);

    cp_a.create_rectangle(rad,rad);
    cp_b.create_rectangle(rad,rad);
    cp_c.create_rectangle(rad,rad);
    cp_d.create_rectangle(rad,rad);

    
    cp_a.set_position(&Vector::new(ctr.x-rw/2.0, ctr.y-rh/2.0));
    cp_b.set_position(&Vector::new(ctr.x+rw/2.0, ctr.y-rh/2.0));
    cp_c.set_position(&Vector::new(ctr.x+rw/2.0, ctr.y+rh/2.0));
    cp_d.set_position(&Vector::new(ctr.x-rw/2.0, ctr.y+rh/2.0));

    let mut spr_a = PolyPolyConstraint::new(tuple.4);
    let spr_b = PolyPolyConstraint::new(tuple.5);
    let spr_c = PolyPolyConstraint::new(tuple.6);
    let spr_d = PolyPolyConstraint::new(tuple.7);

    spr_a.init_spring((tuple.0,tuple.1), cp_a.get_position().distance(&cp_b.get_position()), 0.5);
    spr_a.init_spring((tuple.1,tuple.2), cp_b.get_position().distance(&cp_c.get_position()), 0.5);
    spr_a.init_spring((tuple.2,tuple.3), cp_c.get_position().distance(&cp_d.get_position()), 0.5);
    spr_a.init_spring((tuple.3,tuple.0), cp_d.get_position().distance(&cp_a.get_position()), 0.5);

    part.add_rectangle_particle(cp_a);
    part.add_rectangle_particle(cp_b);
    part.add_rectangle_particle(cp_c);
    part.add_rectangle_particle(cp_d);
    
    part.add_poly_poly_constraint(spr_a);
    part.add_poly_poly_constraint(spr_b);
    part.add_poly_poly_constraint(spr_c);
    part.add_poly_poly_constraint(spr_d);
}