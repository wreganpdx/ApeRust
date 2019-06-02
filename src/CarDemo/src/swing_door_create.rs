extern crate ape_rust;
use ape_rust::circle_particle::CircleParticle;
use ape_rust::particle::Particle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::poly_poly_constraint::PolyPolyConstraint;
use ape_rust::rectangle_particle::RectangleParticle;
use ape_rust::vector::Vector;

pub fn swing_door_create(
    part: &mut ParticleCollection,
    tuple: (i64, i64, i64, i64, i64, i64, i64, i64),
    col_c: [f32; 4],
    col_e: [f32; 4],
) {
    let mut swing_door_p1 = CircleParticle::new(tuple.0);
    swing_door_p1.init_circle(7.0);
    swing_door_p1.set_position(&Vector::new(543.0, 55.0));
    swing_door_p1.set_mass(0.001);
    swing_door_p1.set_primary_color(col_e);

    let mut swing_door_p2 = CircleParticle::new(tuple.1);
    swing_door_p2.init_circle(7.0);
    swing_door_p2.set_position(&Vector::new(620.0, 55.0));
    swing_door_p2.set_primary_color(col_e);
    swing_door_p2.set_fixed(true);

    let mut swingDoor = PolyPolyConstraint::new(tuple.2);
    swingDoor.init_spring(
        (tuple.0, tuple.1),
        swing_door_p1
            .get_position()
            .distance(&swing_door_p2.get_position()),
        1.0,
    );
    swingDoor.circ_circ = true;
    swingDoor.set_collidable(true);
    swingDoor.set_primary_color(col_e);
    swingDoor.set_height(15.0);

    let mut swingDoor_r = RectangleParticle::new(tuple.3);
    swingDoor_r.set_constraint(
        swingDoor.id.clone(),
        swing_door_p1.id.clone(),
        swing_door_p2.id.clone(),
        &swing_door_p1.get_position(),
        &swing_door_p2.get_position(),
        swingDoor.get_height(),
    );
    swingDoor_r.set_collidable(true);
    swingDoor_r.set_primary_color(col_e);
    swingDoor.set_rect_id(swingDoor_r.get_id().clone());
    swingDoor_r.set_fixed(false);

    let mut swing_door_anchor = CircleParticle::new(tuple.4);
    swing_door_anchor.init_circle(2.0);
    swing_door_anchor.set_mass(2.0);
    swing_door_anchor.set_position(&Vector::new(543.0, 5.0));
    swing_door_anchor.set_primary_color(col_e);
    swing_door_anchor.set_fixed(true);
    swing_door_anchor.set_visible(false);

    let mut swingDoorSpring = PolyPolyConstraint::new(tuple.5);
    swingDoorSpring.init_spring(
        (
            swing_door_p1.get_id().clone(),
            swing_door_anchor.get_id().clone(),
        ),
        swing_door_p1
            .get_position()
            .distance(&swing_door_anchor.get_position()),
        0.004,
    );
    swingDoorSpring.circ_circ = true;
    swingDoorSpring.set_collidable(false);
    swingDoorSpring.set_visible(false);
    swingDoorSpring.set_rest_length(40.0);

    let mut stopper_a = CircleParticle::new(tuple.6);
    stopper_a.init_circle(70.0);
    stopper_a.set_position(&Vector::new(550.0, -60.0));
    stopper_a.set_visible(false);
    stopper_a.set_fixed(true);

    let mut stopper_b = RectangleParticle::new(tuple.7);
    stopper_b.create_rectangle(42.0, 70.0);
    stopper_b.set_position(&Vector::new(160.0, 130.0));
    stopper_b.set_visible(false);
    stopper_b.set_fixed(true);

    part.add_circle_particle(swing_door_p1);
    part.add_circle_particle(swing_door_p2);
    part.add_circle_particle(swing_door_anchor);
    part.add_circle_particle(stopper_a);
    part.add_rectangle_particle(swingDoor_r);
    part.add_rectangle_particle(stopper_b);
    part.add_poly_poly_constraint(swingDoor);
    part.add_poly_poly_constraint(swingDoorSpring);
}
