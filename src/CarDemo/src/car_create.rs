extern crate ape_rust;
//use ape_rust::polygon_particle::PolygonParticle;
use ape_rust::circle_particle::CircleParticle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::poly_poly_constraint::PolyPolyConstraint;
use ape_rust::vector::Vector;
use ape_rust::particle::Particle;

pub fn car_create(part:&mut ParticleCollection, tuple:(i64,i64,i64))
{
    let _ctr = &part.get_center();
    let _rw:f64 = 14.0;
    let _rh:f64 = 14.0;
    let rad:f64 = 14.0;

    let mut cp_a = CircleParticle::new(tuple.0);


    let mut cp_b = CircleParticle::new(tuple.1);

    cp_a.set_collidable(true);
    cp_b.set_collidable(true);

    cp_a.init_circle(rad);
    cp_b.init_circle(rad);

    cp_a.init_wheel(2.0);
    cp_b.init_wheel(2.0);
    

    
    cp_a.set_position(&Vector::new(140.0, 10.0));
    cp_b.set_position(&Vector::new(200.0, 10.0));

    let mut spr_a = PolyPolyConstraint::new(tuple.2);

    spr_a.init_spring((tuple.0,tuple.1), cp_a.get_position().distance(&cp_b.get_position()), 0.5);

    spr_a.circ_circ = true;
    spr_a.set_height(2.0);

    part.add_circle_particle(cp_a);
    part.add_circle_particle(cp_b);
    
    part.add_poly_poly_constraint(spr_a);
}