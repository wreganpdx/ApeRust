extern crate ApeRust;
use ApeRust::polygon_particle::polygon_particle;
use ApeRust::circle_particle::circle_particle;
use ApeRust::particle_collection::particle_collection;
use ApeRust::poly_poly_constraint::poly_poly_constraint;
use ApeRust::vector::vector;
use ApeRust::particle::particle;

pub fn create_rectangle(part:&mut particle_collection, tuple:(i64,i64,i64,i64,i64,i64,i64,i64))
{
    let ctr = &part.get_center();
    let rw:f64 = 75.0;
    let rh:f64 = 75.0;
    let rad:f64 = 4.0;

    let mut cpA = circle_particle::new(tuple.0);
    let mut cpB = circle_particle::new(tuple.1);
    let mut cpC = circle_particle::new(tuple.2);
    let mut cpD = circle_particle::new(tuple.3);

    cpA.set_collidable(true);
    cpB.set_collidable(true);

    cpC.set_collidable(true);

    cpD.set_collidable(true);

    cpA.init_circle(rad);
    cpB.init_circle(rad);
    cpC.init_circle(rad);
    cpD.init_circle(rad);

    
    cpA.set_position(&vector::new(ctr.x-rw/2.0, ctr.y-rh/2.0));
    cpB.set_position(&vector::new(ctr.x+rw/2.0, ctr.y-rh/2.0));
    cpC.set_position(&vector::new(ctr.x+rw/2.0, ctr.y+rh/2.0));
    cpD.set_position(&vector::new(ctr.x-rw/2.0, ctr.y+rh/2.0));

    let mut sprA = poly_poly_constraint::new(tuple.4);
    let mut sprB = poly_poly_constraint::new(tuple.5);
    let mut sprC = poly_poly_constraint::new(tuple.6);
    let mut sprD = poly_poly_constraint::new(tuple.7);

    sprA.init_spring((tuple.0,tuple.1), cpA.get_position().distance(&cpB.get_position()), 0.5);
    sprB.init_spring((tuple.1,tuple.2), cpB.get_position().distance(&cpC.get_position()), 0.5);
    sprC.init_spring((tuple.2,tuple.3), cpC.get_position().distance(&cpD.get_position()), 0.5);
    sprD.init_spring((tuple.3,tuple.0), cpD.get_position().distance(&cpA.get_position()), 0.5);

    sprA.circ_circ = true;
    sprB.circ_circ = true;
    sprC.circ_circ = true;
    sprD.circ_circ = true;

    part.add_circle_particle(cpA);
    part.add_circle_particle(cpB);
    part.add_circle_particle(cpC);
    part.add_circle_particle(cpD);
    
    part.add_poly_poly_constraint(sprA);
    part.add_poly_poly_constraint(sprB);
    part.add_poly_poly_constraint(sprC);
    part.add_poly_poly_constraint(sprD);
}