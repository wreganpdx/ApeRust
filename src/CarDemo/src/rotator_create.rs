extern crate ape_rust;
use ape_rust::rectangle_particle::RectangleParticle;
use ape_rust::circle_particle::CircleParticle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::poly_poly_constraint::PolyPolyConstraint;
use ape_rust::vector::Vector;
use ape_rust::particle::Particle;

pub fn rotator_create(rect:&mut ParticleCollection, various:&mut ParticleCollection, tuple:(i64,i64,i64, i64,i64),
                                                                                    tuple2:(i64,i64,i64, i64,
                                                                                    i64,i64,i64, i64,
                                                                                    i64,i64,i64, i64,
),col_a:[f32; 4], col_b:[f32; 4])
{

    println!("{:?}, {:?}", tuple, tuple2);
    let center = Vector::new(555.0, 175.0);
    let mut circle_tuple = rect_composite_create(rect, tuple2, col_a, col_b, center.clone());
    //addComposite(rectComposite);
			

    let mut cp_b = CircleParticle::new(tuple.0);
    cp_b.set_position(&Vector::new(center.x , center.y));
    cp_b.init_circle(5.0);
    cp_b.set_collidable(true);
    cp_b.set_primary_color(col_a);


    let mut rect1 = RectangleParticle::new(tuple.1);
    rect1.set_position(&Vector::new(555.0, 160.0));
    rect1.set_mass(3.0);
    rect1.create_rectangle(10.0, 10.0);
    rect1.set_collidable(true);
    rect1.set_collide_internal(true);
    rect1.set_primary_color(col_b);
    rect1.set_move_with_composite(false);

    let mut spr_b = PolyPolyConstraint::new(tuple.2);
    spr_b.init_spring((rect1.id.clone(), circle_tuple.0.id.clone()), circle_tuple.0.get_position().distance(&rect1.get_position()), 1.0);
    spr_b.set_collidable(false);
    spr_b.set_primary_color(col_b);
    spr_b.set_height(1.0);
    spr_b.rect_circ = true;

    let mut rect2 = RectangleParticle::new(tuple.3);
    rect2.set_position(&Vector::new(555.0, 160.0));
    rect2.set_mass(3.0);
    rect2.create_rectangle(10.0, 10.0);
    rect2.set_collide_internal(true);
    rect2.set_collidable(true);
    rect2.set_primary_color(col_b);
    rect2.set_move_with_composite(false);
   // circle_tuple = rect_composite_create::rect_composite_create(rect, tuple2, col_a, col_b, center.clone());
    let mut spr_c = PolyPolyConstraint::new(tuple.4);
    spr_c.init_spring( (rect2.id.clone(), circle_tuple.1.id.clone()),circle_tuple.1.get_position().distance(&rect2.get_position()), 1.0);
    spr_c.set_collidable(false);
    spr_c.set_primary_color(col_b);
    spr_c.set_height(1.0);
    spr_c.rect_circ = true;
    
  

    various.add_circle_particle(cp_b);

    rect.add_rectangle_particle(rect1);
    rect.add_rectangle_particle(rect2);

    rect.add_poly_poly_constraint(spr_c);
    rect.add_poly_poly_constraint(spr_b);

    rect.add_circle_particle(circle_tuple.0);
    rect.add_circle_particle(circle_tuple.1);

}

pub fn rect_composite_create(part:&mut ParticleCollection, tuple:(i64,i64,i64, i64,
                                                                i64,i64,i64, i64,
                                                                i64,i64,i64, i64,
    ), col_a:[f32; 4], col_b:[f32; 4], cntr:Vector)->(CircleParticle, CircleParticle)
{
    let mut rw = 75.0;
    let mut rh = 18.0;
    let rad = 4.0;
    
    part.init_composite(cntr);
    let center = part.get_center();

    let mut cp_a = CircleParticle::new(tuple.0);
    cp_a.set_position(&Vector::new(center.x - rw/2.0, center.y - rh/2.0));
    cp_a.set_collide_internal(false);
    cp_a.init_circle(rad);
    cp_a.set_collidable(true);
    cp_a.set_primary_color(col_a);
    cp_a.set_fixed(true);
    cp_a.set_move_with_composite(true);

    let mut cp_b = CircleParticle::new(tuple.1);
    cp_b.set_position(&Vector::new(center.x + rw/2.0, center.y - rh/2.0));
    cp_b.set_collide_internal(false);
    cp_b.init_circle(rad);
    cp_b.set_collidable(true);
    cp_b.set_primary_color(col_a);
    cp_b.set_fixed(true);
    cp_b.set_move_with_composite(true);

    let mut cp_c = CircleParticle::new(tuple.2);
    cp_c.set_position(&Vector::new(center.x + rw/2.0, center.y + rh/2.0));
    cp_c.set_collide_internal(false);
    cp_c.init_circle(rad);
    cp_c.set_collidable(true);
    cp_c.set_primary_color(col_a);
    cp_c.set_fixed(true);
    cp_c.set_move_with_composite(true);

    let mut cp_d = CircleParticle::new(tuple.3);
    cp_d.set_position(&Vector::new(center.x - rw/2.0, center.y + rh/2.0));
    cp_d.set_collide_internal(false);
    cp_d.init_circle(rad);
    cp_d.set_collidable(true);
    cp_d.set_primary_color(col_a);
    cp_d.set_fixed(true);
    cp_d.set_move_with_composite(true);

    let mut spr_a = PolyPolyConstraint::new(tuple.4);
    spr_a.init_spring((cp_a.id.clone(), cp_b.id.clone()), cp_a.get_position().distance(&cp_b.get_position()), 0.5);
    spr_a.set_collidable(true);
    spr_a.set_primary_color(col_b);
    spr_a.set_height(rad*2.0);
    spr_a.circ_circ = true;

    let mut spr_a_r = RectangleParticle::new(tuple.5);
    spr_a_r.set_constraint(spr_a.id.clone(), cp_a.id.clone(), cp_b.id.clone(), &cp_a.get_position(), &cp_b.get_position(), spr_a.get_height());
    spr_a_r.set_collide_internal(false);
    spr_a_r.set_collidable(true);
    spr_a_r.set_primary_color(col_b);
    spr_a.set_rect_id(spr_a_r.get_id().clone());
    spr_a_r.set_fixed(true);
    	

    let mut spr_b = PolyPolyConstraint::new(tuple.6);
    spr_b.init_spring((cp_a.id.clone(), cp_b.id.clone()), cp_a.get_position().distance(&cp_b.get_position()), 0.5);
    spr_b.set_collidable(true);
    spr_b.set_primary_color(col_b);
    spr_b.set_height(rad*2.0);
    spr_b.circ_circ = true;

    let mut spr_b_r = RectangleParticle::new(tuple.7);
    spr_b_r.set_constraint(spr_b.id.clone(), cp_b.id.clone(), cp_c.id.clone(), &cp_b.get_position(), &cp_c.get_position(), spr_b.get_height());
    spr_b_r.set_collide_internal(false);
    spr_b_r.set_collidable(true);
    spr_b_r.set_primary_color(col_b);
    spr_b.set_rect_id(spr_b_r.get_id().clone());
    spr_b_r.set_fixed(true);

    let mut spr_c = PolyPolyConstraint::new(tuple.8);
    spr_c.init_spring((cp_a.id.clone(), cp_b.id.clone()), cp_a.get_position().distance(&cp_b.get_position()), 0.5);
    spr_c.set_collidable(true);
    spr_c.set_primary_color(col_b);
    spr_c.set_height(rad*2.0);
    spr_c.circ_circ = true;
    

    let mut spr_c_r = RectangleParticle::new(tuple.9);
    spr_c_r.set_constraint(spr_c.id.clone(), cp_c.id.clone(), cp_d.id.clone(), &cp_c.get_position(), &cp_d.get_position(), spr_c.get_height());
    spr_c_r.set_collidable(true);
    spr_c_r.set_collide_internal(false);
    spr_c_r.set_primary_color(col_b);
    spr_c.set_rect_id(spr_c_r.get_id().clone());
    spr_c_r.set_fixed(true);

    let mut spr_d = PolyPolyConstraint::new(tuple.10);
    spr_d.init_spring((cp_a.id.clone(), cp_b.id.clone()), cp_a.get_position().distance(&cp_b.get_position()), 0.5);
    spr_d.set_collidable(true);
    
    spr_d.set_primary_color(col_b);
    spr_d.set_height(rad*2.0);
    spr_d.circ_circ = true;

    let mut spr_d_r = RectangleParticle::new(tuple.11);
    spr_d_r.set_constraint(spr_d.id.clone(), cp_d.id.clone(), cp_a.id.clone(), &cp_d.get_position(), &cp_a.get_position(), spr_d.get_height());
    spr_d_r.set_collidable(true);
    spr_d_r.set_collide_internal(false);
    spr_d_r.set_primary_color(col_b);
    spr_d.set_rect_id(spr_d_r.get_id().clone());
    spr_d_r.set_fixed(true);
  
    part.add_circle_particle(cp_c);
    part.add_circle_particle(cp_d);

    part.add_rectangle_particle(spr_a_r);
    

    part.add_rectangle_particle(spr_b_r);
  
    part.add_rectangle_particle(spr_c_r);
    part.add_rectangle_particle(spr_d_r);

    part.add_poly_poly_constraint(spr_a);
    part.add_poly_poly_constraint(spr_b);
    part.add_poly_poly_constraint(spr_c);
    part.add_poly_poly_constraint(spr_d);

    part.set_collide_internal(false);

    return (cp_a, cp_b);
}