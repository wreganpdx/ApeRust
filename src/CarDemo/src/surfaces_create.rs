extern crate ApeRust;
use ApeRust::polygon_particle::polygon_particle;
use ApeRust::circle_particle::circle_particle;
use ApeRust::particle_collection::particle_collection;
use ApeRust::rectangle_particle::rectangle_particle;
use ApeRust::poly_poly_constraint::poly_poly_constraint;
use ApeRust::vector::vector;
use ApeRust::particle::particle;

pub fn surfaces_create(part:&mut particle_collection, tuple:(i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,i64,))
{
    /*
    var floor:RectangleParticle = new RectangleParticle(340,327,550,50,0,true);
			floor.setStyle(0, colD, 1, colD);
			addParticle(floor);
			
			var ceil:RectangleParticle = new RectangleParticle(325,-33,649,80,0,true);
			ceil.setStyle(0, colD, 1, colD);
			addParticle(ceil);

			var rampRight:RectangleParticle = new RectangleParticle(375,220,390,20,0.405,true);
			rampRight.setStyle(0, colD, 1, colD);
			addParticle(rampRight);
			
			var rampLeft:RectangleParticle = new RectangleParticle(90,200,102,20,-.7,true);
			rampLeft.setStyle(0, c
            olD, 1, colD);
			addParticle(rampLeft);
			
			var rampLeft2:RectangleParticle = new RectangleParticle(96,129,102,20,-.7,true);
			rampLeft2.setStyle(0, colD, 1, colD);
			addParticle(rampLeft2);
			
			var rampCircle:CircleParticle = new CircleParticle(175,190,60,true);
			rampCircle.setStyle(1, colD, 1, colB);
			addParticle(rampCircle);
			
			var floorBump:CircleParticle = new CircleParticle(600,660,400,true);
			floorBump.setStyle(1, colD, 1, colB);
			addParticle(floorBump);
			
			var bouncePad:RectangleParticle = new RectangleParticle(35,370,40,60,0,true);
			bouncePad.setStyle(0, colD, 1, 0x996633);
			bouncePad.elasticity = 4;
			addParticle(bouncePad);
			
			var leftWall:RectangleParticle = new RectangleParticle(1,99,30,500,0,true);
			leftWall.setStyle(0, colD, 1, colD);
			addParticle(leftWall);
			
			var leftWallChannelInner:RectangleParticle = new RectangleParticle(54,300,20,150,0,true);
			leftWallChannelInner.setStyle(0, colD, 1, colD);
			addParticle(leftWallChannelInner);
			
			var leftWallChannel:RectangleParticle = new RectangleParticle(54,122,20,94,0,true);
			leftWallChannel.setStyle(0, colD, 1, colD);
			addParticle(leftWallChannel);
			
			var leftWallChannelAng:RectangleParticle = new RectangleParticle(75,65,60,25,- 0.7,true);
			leftWallChannelAng.setStyle(0, colD, 1, colD);
			addParticle(leftWallChannelAng);
			
			var topLeftAng:RectangleParticle = new RectangleParticle(23,11,65,40,-0.7,true);
			topLeftAng.setStyle(0, colD, 1, colD);
			addParticle(topLeftAng);
			
			var rightWall:RectangleParticle = new RectangleParticle(654,230,50,500,0,true);
			rightWall.setStyle(0, colD, 1, colD);
			addParticle(rightWall);
	
			var bridgeStart:RectangleParticle = new RectangleParticle(127,49,75,25,0,true);
			bridgeStart.setStyle(0, colD, 1, colD);
			addParticle(bridgeStart);
			
			var bridgeEnd:RectangleParticle = new RectangleParticle(483,55,100,15,0,true);
			bridgeEnd.setStyle(0, colD, 1, colD);
			addParticle(bridgeEnd);
            */

    let mut floor = rectangle_particle::new(tuple.0);
    floor.set_position(&vector::new(340.0, 327.0));
    floor.create_rectangle(550.0,50.0);
    floor.set_fixed(true);
    floor.set_collidable(true);
    part.add_rectangle_particle(floor);


    let mut ceil = rectangle_particle::new(tuple.0);
    ceil.set_position(&vector::new(325.0, -33.0));
    ceil.create_rectangle(649.0,80.0);
    ceil.set_fixed(true);
    ceil.set_collidable(true);
    part.add_rectangle_particle(ceil);


    let mut rampRight = rectangle_particle::new(tuple.0);
    rampRight.set_position(&vector::new(375.0, 220.0));
    rampRight.create_rectangle(390.0,20.0);
    rampRight.set_fixed(true);
    rampRight.set_radian(0.405);
    rampRight.set_collidable(true);
    part.add_rectangle_particle(rampRight);


    let mut rampLeft = rectangle_particle::new(tuple.0);
    rampLeft.set_position(&vector::new(90.0, 200.0));
    rampLeft.create_rectangle(102.0,20.0);
    rampLeft.set_fixed(true);
    rampLeft.set_radian(-0.7);
    rampLeft.set_collidable(true);
    part.add_rectangle_particle(rampLeft);


    let mut ramp = circle_particle::new(tuple.0);
    ramp.set_position(&vector::new(175.0, 190.0));
    ramp.init_circle(60.0);
    ramp.set_fixed(true);
    ramp.set_collidable(true);
    part.add_circle_particle(ramp);


    let mut bump = circle_particle::new(tuple.0);
    bump.set_position(&vector::new(600.0, 660.0));
    bump.init_circle(400.0);
    bump.set_fixed(true);
    bump.set_collidable(true);
    part.add_circle_particle(bump);


    let mut bouncePad = rectangle_particle::new(tuple.0);
    bouncePad.set_position(&vector::new(35.0, 370.0));
    bouncePad.create_rectangle(40.0,60.0);
    bouncePad.set_fixed(true);
    bouncePad.set_collidable(true);
    bouncePad.set_elasticity(4.0);
    part.add_rectangle_particle(bouncePad);


    let mut leftWall = rectangle_particle::new(tuple.0);
    leftWall.set_position(&vector::new(1.0, 99.0));
    leftWall.create_rectangle(30.0,500.0);
    leftWall.set_fixed(true);
    leftWall.set_collidable(true);
    part.add_rectangle_particle(leftWall);


    let mut leftWallch = rectangle_particle::new(tuple.0);
    leftWallch.set_position(&vector::new(54.0, 300.0));
    leftWallch.create_rectangle(20.0,150.0);
    leftWallch.set_fixed(true);
    leftWallch.set_collidable(true);
    part.add_rectangle_particle(leftWallch);

    let mut leftWallch_i = rectangle_particle::new(tuple.0);
    leftWallch_i.set_position(&vector::new(54.0, 122.0));
    leftWallch_i.create_rectangle(20.0,94.0);
    leftWallch_i.set_fixed(true);
    leftWallch_i.set_collidable(true);
    part.add_rectangle_particle(leftWallch_i);

    let mut leftWallch_a = rectangle_particle::new(tuple.0);
    leftWallch_a.set_position(&vector::new(75.0, 65.0));
    leftWallch_a.create_rectangle(60.0,25.0);
    leftWallch_a.set_fixed(true);
    leftWallch_a.set_collidable(true);
    leftWallch_a.set_radian(-0.7);
    part.add_rectangle_particle(leftWallch_a);

     let mut topLeftAng = rectangle_particle::new(tuple.0);
    topLeftAng.set_position(&vector::new(23.0, 11.0));
    topLeftAng.create_rectangle(40.0,11.0);
    topLeftAng.set_fixed(true);
    topLeftAng.set_collidable(true);
    topLeftAng.set_radian(-0.7);
    part.add_rectangle_particle(topLeftAng);

    let mut rightWall = rectangle_particle::new(tuple.0);
    rightWall.set_position(&vector::new(654.0, 230.0));
    rightWall.create_rectangle(50.0,500.0);
    rightWall.set_fixed(true);
    rightWall.set_collidable(true);
    part.add_rectangle_particle(rightWall);

    let mut bridge_start = rectangle_particle::new(tuple.0);
    bridge_start.set_position(&vector::new(127.0, 49.0));
    bridge_start.create_rectangle(75.0,25.0);
    bridge_start.set_fixed(true);
    bridge_start.set_collidable(true);
    part.add_rectangle_particle(bridge_start);

    let mut bridge_end = rectangle_particle::new(tuple.0);
    bridge_end.set_position(&vector::new(483.0, 55.0));
    bridge_end.create_rectangle(100.0,15.0);
    bridge_end.set_fixed(true);
    bridge_end.set_collidable(true);
    part.add_rectangle_particle(bridge_end);

}