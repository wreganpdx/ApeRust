extern crate ape_rust;
//use ape_rust::polygon_particle::PolygonParticle;
use ape_rust::circle_particle::CircleParticle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::poly_poly_constraint::PolyPolyConstraint;
use ape_rust::vector::Vector;
use ape_rust::particle::Particle;

pub fn capsule_create(part:&mut ParticleCollection, tuple:(i64,i64,i64))
{
    /*
    var capsuleP1:CircleParticle = new CircleParticle(300,10,14,false,1.3,0.4);
			capsuleP1.setStyle(0, colC, 1, colC);
			addParticle(capsuleP1);
			
			var capsuleP2:CircleParticle = new CircleParticle(325,35,14,false,1.3,0.4);
			capsuleP2.setStyle(0, colC, 1, colC);
			addParticle(capsuleP2);
			
			var capsule:SpringConstraint = new SpringConstraint(capsuleP1, capsuleP2, 1, true, 24);
			capsule.setStyle(5, colC, 1, colC, 1);
			addConstraint(capsule);
            */
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

    cp_a.set_mass(1.3);
    cp_b.set_mass(1.3);

    cp_a.set_elasticity(0.4);
    cp_b.set_elasticity(0.4);


    //300,10,14,false,1.3,0.4);
    cp_a.set_position(&Vector::new(300.0, 10.0));
    cp_b.set_position(&Vector::new(325.0, 10.0));

    let mut spr_a = PolyPolyConstraint::new(tuple.2);

    spr_a.init_spring((tuple.0,tuple.1), cp_a.get_position().distance(&cp_b.get_position()), 1.0);

    spr_a.circ_circ = true;
    spr_a.set_height(28.0);

    part.add_circle_particle(cp_a);
    part.add_circle_particle(cp_b);
    
    part.add_poly_poly_constraint(spr_a);
}