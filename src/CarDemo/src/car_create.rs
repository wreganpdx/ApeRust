extern crate ApeRust;
use ApeRust::polygon_particle::polygon_particle;
use ApeRust::circle_particle::circle_particle;
use ApeRust::particle_collection::particle_collection;
use ApeRust::poly_poly_constraint::poly_poly_constraint;
use ApeRust::vector::vector;
use ApeRust::particle::particle;

pub fn car_create(part:&mut particle_collection, tuple:(i64,i64,i64))
{
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

    cpA.init_wheel(2.0);
    cpB.init_wheel(2.0);
    

    
    cpA.set_position(&vector::new(140.0, 10.0));
    cpB.set_position(&vector::new(200.0, 10.0));

    let mut sprA = poly_poly_constraint::new(tuple.2);

    sprA.init_spring((tuple.0,tuple.1), cpA.get_position().distance(&cpB.get_position()), 0.5);

    sprA.circ_circ = true;
    sprA.set_height(2.0);

    part.add_circle_particle(cpA);
    part.add_circle_particle(cpB);
    
    part.add_poly_poly_constraint(sprA);
}