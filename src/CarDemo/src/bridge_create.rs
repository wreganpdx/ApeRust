extern crate ape_rust;
use ape_rust::circle_particle::CircleParticle;
use ape_rust::particle::Particle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::poly_poly_constraint::PolyPolyConstraint;
use ape_rust::rectangle_particle::RectangleParticle;
use ape_rust::vector::Vector;
pub fn bridge_create(
    part: &mut ParticleCollection,
    tuple: (
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
    ),
    col_b: [f32; 4],
    col_c: [f32; 4],
    col_d: [f32; 4],
) {
    let mut bx = 170.0;
    let mut by = 40.0;
    let bsize = 51.5;
    let yslope = 2.4;
    let particle_size = 4.0;

    let mut bridge_paa = CircleParticle::new(tuple.0);
    bridge_paa.set_position(&Vector::new(bx.clone(), by.clone()));
    bridge_paa.init_circle(particle_size);
    bridge_paa.set_collidable(true);
    bridge_paa.set_primary_color(col_c);
    bridge_paa.set_secondary_color(col_b);
    bridge_paa.set_fixed(true);

    bx += bsize;
    by += yslope;

    let mut bridge_pa = CircleParticle::new(tuple.1);
    bridge_pa.set_position(&Vector::new(bx.clone(), by.clone()));
    bridge_pa.init_circle(particle_size);
    bridge_pa.set_collidable(true);
    bridge_pa.set_primary_color(col_c);
    bridge_pa.set_secondary_color(col_b);

    bx += bsize;
    by += yslope;

    let mut bridge_pb = CircleParticle::new(tuple.2);
    bridge_pb.set_position(&Vector::new(bx.clone(), by.clone()));
    bridge_pb.init_circle(particle_size);
    bridge_pb.set_collidable(true);
    bridge_pb.set_primary_color(col_c);
    bridge_pb.set_secondary_color(col_b);

    bx += bsize;
    by += yslope;

    let mut bridge_pc = CircleParticle::new(tuple.3);
    bridge_pc.set_position(&Vector::new(bx.clone(), by.clone()));
    bridge_pc.init_circle(particle_size);
    bridge_pc.set_collidable(true);
    bridge_pc.set_primary_color(col_c);
    bridge_pc.set_secondary_color(col_b);

    bx += bsize;
    by += yslope;

    let mut bridge_pd = CircleParticle::new(tuple.4);
    bridge_pd.set_position(&Vector::new(bx.clone(), by.clone()));
    bridge_pd.init_circle(particle_size);
    bridge_pd.set_collidable(true);
    bridge_pd.set_primary_color(col_c);
    bridge_pd.set_secondary_color(col_b);

    bx += bsize;
    by += yslope;

    let mut bridge_pdd = CircleParticle::new(tuple.5);
    bridge_pdd.set_position(&Vector::new(bx.clone(), by.clone()));
    bridge_pdd.init_circle(particle_size);
    bridge_pdd.set_collidable(true);
    bridge_pdd.set_primary_color(col_c);
    bridge_pdd.set_secondary_color(col_b);
    bridge_pdd.set_fixed(true);

    let mut bridge_conn_a = PolyPolyConstraint::new(tuple.6);
    bridge_conn_a.init_spring(
        (bridge_paa.id.clone(), bridge_pa.id.clone()),
        bridge_paa
            .get_position()
            .distance(&bridge_pa.get_position()),
        0.9,
    );
    bridge_conn_a.set_collidable(true);
    bridge_conn_a.set_primary_color(col_c);
    bridge_conn_a.set_secondary_color(col_b);
    bridge_conn_a.set_height(5.0);
    bridge_conn_a.set_fixed_end_limit(0.25);
    bridge_conn_a.set_width_scale(0.8);
    bridge_conn_a.circ_circ = true;

    let mut bridge_conn_r_a = RectangleParticle::new(tuple.7);
    bridge_conn_r_a.set_constraint(
        bridge_conn_a.id.clone(),
        bridge_paa.id.clone(),
        bridge_pa.id.clone(),
        &bridge_paa.get_position(),
        &bridge_pa.get_position(),
        bridge_conn_a.get_height() * 2.0,
    );
    bridge_conn_r_a.set_width_scale(0.8);
    bridge_conn_r_a.set_collidable(true);
    bridge_conn_r_a.set_primary_color(col_b);
    bridge_conn_a.set_rect_id(bridge_conn_r_a.get_id().clone());

    let mut bridge_conn_b = PolyPolyConstraint::new(tuple.8);
    bridge_conn_b.init_spring(
        (bridge_pa.id.clone(), bridge_pb.id.clone()),
        bridge_pa.get_position().distance(&bridge_pb.get_position()),
        0.9,
    );
    bridge_conn_b.set_collidable(true);
    bridge_conn_b.set_primary_color(col_c);
    bridge_conn_b.set_secondary_color(col_b);
    bridge_conn_b.set_height(5.0);
    bridge_conn_b.set_width_scale(0.8);
    bridge_conn_b.circ_circ = true;

    let mut bridge_conn_r_b = RectangleParticle::new(tuple.9);
    bridge_conn_r_b.set_constraint(
        bridge_conn_b.id.clone(),
        bridge_pa.id.clone(),
        bridge_pb.id.clone(),
        &bridge_pa.get_position(),
        &bridge_pb.get_position(),
        bridge_conn_b.get_height() * 2.0,
    );
    bridge_conn_r_b.set_width_scale(0.8);
    bridge_conn_r_b.set_collidable(true);
    bridge_conn_r_b.set_primary_color(col_b);
    bridge_conn_b.set_rect_id(bridge_conn_r_b.get_id().clone());

    let mut bridge_conn_c = PolyPolyConstraint::new(tuple.10);
    bridge_conn_c.init_spring(
        (bridge_pb.id.clone(), bridge_pc.id.clone()),
        bridge_pb.get_position().distance(&bridge_pc.get_position()),
        0.9,
    );
    bridge_conn_c.set_collidable(true);
    bridge_conn_c.set_primary_color(col_c);
    bridge_conn_c.set_secondary_color(col_b);
    bridge_conn_c.set_height(5.0);
    bridge_conn_c.set_width_scale(0.8);
    bridge_conn_c.circ_circ = true;

    let mut bridge_conn_r_c = RectangleParticle::new(tuple.11);
    bridge_conn_r_c.set_constraint(
        bridge_conn_c.id.clone(),
        bridge_pb.id.clone(),
        bridge_pc.id.clone(),
        &bridge_pb.get_position(),
        &bridge_pc.get_position(),
        bridge_conn_c.get_height() * 2.0,
    );
    bridge_conn_r_c.set_width_scale(0.8);
    bridge_conn_r_c.set_collidable(true);
    bridge_conn_r_c.set_primary_color(col_b);
    bridge_conn_c.set_rect_id(bridge_conn_r_c.get_id().clone());

    let mut bridge_conn_d = PolyPolyConstraint::new(tuple.12);
    bridge_conn_d.init_spring(
        (bridge_pc.id.clone(), bridge_pd.id.clone()),
        bridge_pc.get_position().distance(&bridge_pd.get_position()),
        0.9,
    );
    bridge_conn_d.set_collidable(true);
    bridge_conn_d.set_primary_color(col_c);
    bridge_conn_d.set_secondary_color(col_b);
    bridge_conn_d.set_height(5.0);
    bridge_conn_d.set_width_scale(0.8);
    bridge_conn_d.circ_circ = true;

    let mut bridge_conn_r_d = RectangleParticle::new(tuple.13);
    bridge_conn_r_d.set_constraint(
        bridge_conn_d.id.clone(),
        bridge_pc.id.clone(),
        bridge_pd.id.clone(),
        &bridge_pc.get_position(),
        &bridge_pd.get_position(),
        bridge_conn_d.get_height() * 2.0,
    );
    bridge_conn_r_d.set_width_scale(0.8);
    bridge_conn_r_d.set_collidable(true);
    bridge_conn_r_d.set_primary_color(col_b);
    bridge_conn_d.set_rect_id(bridge_conn_r_d.get_id().clone());

    let mut bridge_conn_e = PolyPolyConstraint::new(tuple.14);
    bridge_conn_e.init_spring(
        (bridge_pd.id.clone(), bridge_pdd.id.clone()),
        bridge_pd
            .get_position()
            .distance(&bridge_pdd.get_position()),
        0.9,
    );
    bridge_conn_e.set_collidable(true);
    bridge_conn_e.set_primary_color(col_c);
    bridge_conn_e.set_secondary_color(col_b);
    bridge_conn_e.set_height(5.0);
    bridge_conn_e.set_width_scale(0.8);
    bridge_conn_e.circ_circ = true;

    let mut bridge_conn_r_e = RectangleParticle::new(tuple.15);
    bridge_conn_r_e.set_constraint(
        bridge_conn_e.id.clone(),
        bridge_pd.id.clone(),
        bridge_pdd.id.clone(),
        &bridge_pd.get_position(),
        &bridge_pdd.get_position(),
        bridge_conn_e.get_height() * 2.0,
    );
    bridge_conn_r_e.set_width_scale(0.8);
    bridge_conn_r_e.set_collidable(true);
    bridge_conn_r_e.set_primary_color(col_b);
    bridge_conn_e.set_rect_id(bridge_conn_r_e.get_id().clone());

    part.add_circle_particle(bridge_paa);
    part.add_circle_particle(bridge_pa);
    part.add_circle_particle(bridge_pb);
    part.add_circle_particle(bridge_pc);
    part.add_circle_particle(bridge_pd);
    part.add_circle_particle(bridge_pdd);

    part.add_rectangle_particle(bridge_conn_r_a);
    part.add_rectangle_particle(bridge_conn_r_b);
    part.add_rectangle_particle(bridge_conn_r_c);
    part.add_rectangle_particle(bridge_conn_r_d);
    part.add_rectangle_particle(bridge_conn_r_e);

    part.add_poly_poly_constraint(bridge_conn_a);
    part.add_poly_poly_constraint(bridge_conn_b);
    part.add_poly_poly_constraint(bridge_conn_c);
    part.add_poly_poly_constraint(bridge_conn_d);
    part.add_poly_poly_constraint(bridge_conn_e);

    part.set_collide_internal(false);
}
