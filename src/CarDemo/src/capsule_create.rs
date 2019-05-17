extern crate ApeRust;
use ApeRust::polygon_particle::polygon_particle;
use ApeRust::circle_particle::circle_particle;
use ApeRust::particle_collection::particle_collection;
use ApeRust::poly_poly_constraint::poly_poly_constraint;
use ApeRust::vector::vector;
use ApeRust::particle::particle;

pub fn capsule_create(part:&mut particle_collection, tuple:(i64,i64,i64))
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
    let ctr = &part.get_center();
    let rw:f64 = 14.0;
    let rh:f64 = 14.0;
    let rad:f64 = 14.0;

    let mut cpA = circle_particle::new(tuple.0);


    let mut cpB = circle_particle::new(tuple.1);

    cpA.set_collidable(true);
    cpB.set_collidable(true);

    cpA.init_circle(rad);
    cpB.init_circle(rad);

    cpA.set_mass(1.3);
    cpB.set_mass(1.3);

    cpA.set_elasticity(0.4);
    cpB.set_elasticity(0.4);


    //300,10,14,false,1.3,0.4);
    cpA.set_position(&vector::new(300.0, 10.0));
    cpB.set_position(&vector::new(325.0, 10.0));

    let mut sprA = poly_poly_constraint::new(tuple.2);

    sprA.init_spring((tuple.0,tuple.1), cpA.get_position().distance(&cpB.get_position()), 1.0);

    sprA.circ_circ = true;
    sprA.set_height(28.0);

    part.add_circle_particle(cpA);
    part.add_circle_particle(cpB);
    
    part.add_poly_poly_constraint(sprA);
}