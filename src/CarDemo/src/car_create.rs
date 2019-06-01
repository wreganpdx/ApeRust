extern crate ape_rust;
use ape_rust::rectangle_particle::RectangleParticle;
use ape_rust::circle_particle::CircleParticle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::poly_poly_constraint::PolyPolyConstraint;
use ape_rust::vector::Vector;
use ape_rust::particle::Particle;

pub fn car_create(part:&mut ParticleCollection, tuple:(i64,i64,i64, i64), col_c:[f32; 4], col_e:[f32; 4])
{
    let _ctr = &part.get_center();
    let rad:f64 = 14.0;
    let height = 8.0;
    let mut cp_a = CircleParticle::new(tuple.0);


    let mut cp_b = CircleParticle::new(tuple.1);

    let mut rp = RectangleParticle::new(tuple.3);

    rp.set_collidable(true);


   
    cp_a.set_collidable(true);
    cp_b.set_collidable(true);

    cp_a.init_circle(rad);
    cp_b.init_circle(rad);

    cp_a.init_wheel(2.0);
    cp_b.init_wheel(2.0);
    

    
    cp_a.set_position(&Vector::new(200.0, 120.0));
    cp_b.set_position(&Vector::new(260.0, 120.0));

    let mut spr_a = PolyPolyConstraint::new(tuple.2);

    spr_a.init_spring((tuple.0,tuple.1), cp_a.get_position().distance(&cp_b.get_position()), 0.5);
    rp.set_constraint(spr_a.id, cp_a.id.clone(), cp_b.id.clone(), &cp_a.get_position(), &cp_b.get_position(), height.clone());
    spr_a.circ_circ = true;
    spr_a.set_height(height/2.0);
    spr_a.set_collidable(true);
    spr_a.set_rect_id(rp.get_id().clone());

    cp_a.set_primary_color(col_c.clone());
    cp_b.set_primary_color(col_c.clone());
    spr_a.set_primary_color(col_c.clone());
    rp.set_primary_color(col_e);

    cp_a.set_secondary_color(col_e.clone());
    cp_b.set_secondary_color(col_e.clone());
    spr_a.set_secondary_color(col_e.clone());
    rp.set_secondary_color(col_c);


    part.add_circle_particle(cp_a);
    part.add_circle_particle(cp_b);
    part.add_rectangle_particle(rp);
    part.add_poly_poly_constraint(spr_a);
    part.set_collide_internal(false);
}