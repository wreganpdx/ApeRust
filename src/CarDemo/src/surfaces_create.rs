extern crate ape_rust;
//use ape_rust::polygon_particle::PolygonParticle;
use ape_rust::circle_particle::CircleParticle;
use ape_rust::particle_collection::ParticleCollection;
use ape_rust::rectangle_particle::RectangleParticle;
//use ape_rust::poly_poly_constraint::PolyPolyConstraint;
use ape_rust::particle::Particle;
use ape_rust::vector::Vector;

pub fn surfaces_create(
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
    col_d: [f32; 4],
    col_b: [f32; 4],
) {
    /*
    var floor:RectangleParticle = new RectangleParticle(340,327,550,50,0,true);
            floor.setStyle(0, colD, 1, colD);
            addParticle(floor);

            var ceil:RectangleParticle = new RectangleParticle(325,-33,649,80,0,true);
            ceil.setStyle(0, colD, 1, colD);
            addParticle(ceil);

            var ramp_right:RectangleParticle = new RectangleParticle(375,220,390,20,0.405,true);
            ramp_right.setStyle(0, colD, 1, colD);
            addParticle(ramp_right);

            var ramp_left:RectangleParticle = new RectangleParticle(90,200,102,20,-.7,true);
            ramp_left.setStyle(0, c
            olD, 1, colD);
            addParticle(ramp_left);

            var ramp_left2:RectangleParticle = new RectangleParticle(96,129,102,20,-.7,true);
            ramp_left2.setStyle(0, colD, 1, colD);
            addParticle(ramp_left2);

            var rampCircle:CircleParticle = new CircleParticle(175,190,60,true);
            rampCircle.setStyle(1, colD, 1, colB);
            addParticle(rampCircle);

            var floorBump:CircleParticle = new CircleParticle(600,660,400,true);
            floorBump.setStyle(1, colD, 1, colB);
            addParticle(floorBump);

            var bounce_pad:RectangleParticle = new RectangleParticle(35,370,40,60,0,true);
            bounce_pad.setStyle(0, colD, 1, 0x996633);
            bounce_pad.elasticity = 4;
            addParticle(bounce_pad);

            var left_wall:RectangleParticle = new RectangleParticle(1,99,30,500,0,true);
            left_wall.setStyle(0, colD, 1, colD);
            addParticle(left_wall);

            var leftWallChannelInner:RectangleParticle = new RectangleParticle(54,300,20,150,0,true);
            leftWallChannelInner.setStyle(0, colD, 1, colD);
            addParticle(leftWallChannelInner);

            var leftWallChannel:RectangleParticle = new RectangleParticle(54,122,20,94,0,true);
            leftWallChannel.setStyle(0, colD, 1, colD);
            addParticle(leftWallChannel);

            var leftWallChannelAng:RectangleParticle = new RectangleParticle(75,65,60,25,- 0.7,true);
            leftWallChannelAng.setStyle(0, colD, 1, colD);
            addParticle(leftWallChannelAng);

            var top_left_ang:RectangleParticle = new RectangleParticle(23,11,65,40,-0.7,true);
            top_left_ang.setStyle(0, colD, 1, colD);
            addParticle(top_left_ang);

            var right_wall:RectangleParticle = new RectangleParticle(654,230,50,500,0,true);
            right_wall.setStyle(0, colD, 1, colD);
            addParticle(right_wall);

            var bridgeStart:RectangleParticle = new RectangleParticle(127,49,75,25,0,true);
            bridgeStart.setStyle(0, colD, 1, colD);
            addParticle(bridgeStart);

            var bridgeEnd:RectangleParticle = new RectangleParticle(483,55,100,15,0,true);
            bridgeEnd.setStyle(0, colD, 1, colD);
            addParticle(bridgeEnd);
            */

    let mut floor = RectangleParticle::new(tuple.0);
    floor.set_position(&Vector::new(340.0, 327.0));
    floor.create_rectangle(550.0, 50.0);
    floor.set_fixed(true);
    floor.set_collidable(true);
    floor.set_primary_color(col_d.clone());
    part.add_rectangle_particle(floor);

    let mut ceil = RectangleParticle::new(tuple.0);
    ceil.set_position(&Vector::new(325.0, -33.0));
    ceil.create_rectangle(649.0, 80.0);
    ceil.set_fixed(true);
    ceil.set_collidable(true);
    ceil.set_primary_color(col_d.clone());
    part.add_rectangle_particle(ceil);

    let mut ramp_right = RectangleParticle::new(tuple.0);
    ramp_right.set_position(&Vector::new(375.0, 220.0));
    ramp_right.create_rectangle(390.0, 20.0);
    ramp_right.set_fixed(true);
    ramp_right.set_radian(0.405);
    ramp_right.set_collidable(true);
    ramp_right.set_primary_color(col_d.clone());
    part.add_rectangle_particle(ramp_right);

    let mut ramp_left = RectangleParticle::new(tuple.0);
    ramp_left.set_position(&Vector::new(90.0, 200.0));
    ramp_left.create_rectangle(102.0, 20.0);
    ramp_left.set_fixed(true);
    ramp_left.set_radian(-0.7);
    ramp_left.set_collidable(true);
    ramp_left.set_primary_color(col_d.clone());
    part.add_rectangle_particle(ramp_left);

    let mut ramp = CircleParticle::new(tuple.0);
    ramp.set_position(&Vector::new(175.0, 190.0));
    ramp.init_circle(60.0);
    ramp.set_fixed(true);
    ramp.set_collidable(true);
    ramp.set_primary_color(col_d.clone());
    part.add_circle_particle(ramp);

    let mut bump = CircleParticle::new(tuple.0);
    bump.set_position(&Vector::new(600.0, 660.0));
    bump.init_circle(400.0);
    bump.set_fixed(true);
    bump.set_collidable(true);
    bump.set_primary_color(col_d.clone());
    part.add_circle_particle(bump);

    let mut bounce_pad = RectangleParticle::new(tuple.0);
    bounce_pad.set_position(&Vector::new(35.0, 370.0));
    bounce_pad.create_rectangle(40.0, 60.0);
    bounce_pad.set_fixed(true);
    bounce_pad.set_collidable(true);
    bounce_pad.set_elasticity(4.0);
    bounce_pad.set_primary_color(col_d.clone());
    part.add_rectangle_particle(bounce_pad);

    let mut left_wall = RectangleParticle::new(tuple.0);
    left_wall.set_position(&Vector::new(1.0, 99.0));
    left_wall.create_rectangle(30.0, 500.0);
    left_wall.set_fixed(true);
    left_wall.set_collidable(true);
    left_wall.set_primary_color(col_d.clone());
    part.add_rectangle_particle(left_wall);

    let mut left_wall_ch = RectangleParticle::new(tuple.0);
    left_wall_ch.set_position(&Vector::new(54.0, 300.0));
    left_wall_ch.create_rectangle(20.0, 150.0);
    left_wall_ch.set_fixed(true);
    left_wall_ch.set_collidable(true);
    left_wall_ch.set_primary_color(col_d.clone());
    part.add_rectangle_particle(left_wall_ch);

    let mut left_wall_ch_i = RectangleParticle::new(tuple.0);
    left_wall_ch_i.set_position(&Vector::new(54.0, 122.0));
    left_wall_ch_i.create_rectangle(20.0, 94.0);
    left_wall_ch_i.set_fixed(true);
    left_wall_ch_i.set_collidable(true);
    left_wall_ch_i.set_primary_color(col_d.clone());
    part.add_rectangle_particle(left_wall_ch_i);

    let mut left_wall_ch_a = RectangleParticle::new(tuple.0);
    left_wall_ch_a.set_position(&Vector::new(75.0, 65.0));
    left_wall_ch_a.create_rectangle(60.0, 25.0);
    left_wall_ch_a.set_fixed(true);
    left_wall_ch_a.set_collidable(true);
    left_wall_ch_a.set_radian(-0.7);
    left_wall_ch_a.set_primary_color(col_d.clone());
    part.add_rectangle_particle(left_wall_ch_a);

    let mut top_left_ang = RectangleParticle::new(tuple.0);
    top_left_ang.set_position(&Vector::new(23.0, 11.0));
    top_left_ang.create_rectangle(40.0, 11.0);
    top_left_ang.set_fixed(true);
    top_left_ang.set_collidable(true);
    top_left_ang.set_radian(-0.7);
    top_left_ang.set_primary_color(col_d.clone());
    part.add_rectangle_particle(top_left_ang);

    let mut right_wall = RectangleParticle::new(tuple.0);
    right_wall.set_position(&Vector::new(654.0, 230.0));
    right_wall.create_rectangle(50.0, 500.0);
    right_wall.set_fixed(true);
    right_wall.set_collidable(true);
    right_wall.set_primary_color(col_d.clone());
    part.add_rectangle_particle(right_wall);

    let mut bridge_start = RectangleParticle::new(tuple.0);
    bridge_start.set_position(&Vector::new(127.0, 49.0));
    bridge_start.create_rectangle(75.0, 25.0);
    bridge_start.set_fixed(true);
    bridge_start.set_collidable(true);
    bridge_start.set_primary_color(col_d.clone());
    part.add_rectangle_particle(bridge_start);

    let mut bridge_end = RectangleParticle::new(tuple.0);
    bridge_end.set_position(&Vector::new(483.0, 55.0));
    bridge_end.create_rectangle(100.0, 15.0);
    bridge_end.set_fixed(true);
    bridge_end.set_collidable(true);
    bridge_end.set_primary_color(col_d.clone());
    part.add_rectangle_particle(bridge_end);
}
