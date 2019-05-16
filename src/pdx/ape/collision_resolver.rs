 
 use crate::polygon_particle::polygon_particle;
  use crate::rectangle_particle::rectangle_particle;
 use crate::vector::vector;
 use crate::collision::collision;
 use crate::particle::particle;
 use crate::circle_particle::circle_particle;
 use std::f64;

pub fn resolve_particle_particle(pa:&mut polygon_particle, pb:&mut polygon_particle, normal:&vector, depth:f64)
{
   // println!("Collision: {:?} {:?} {:?} {:?} ", pa.get_curr(), pb.get_curr(), normal, depth);
    let im_pb_inv_mass:f64 = pb.get_inv_mass();
    let im_pa_inv_mass:f64 = pa.get_inv_mass();
    // {
        /*
        let im_pb:&polygon_particle = pb;
        let im_pa:&polygon_particle = pa;
        im_pb_inv_mass = im_pb.get_inv_mass();
        im_pa_inv_mass = im_pa.get_inv_mass();
        */
    // }
    
    // a collision has occured. set the current positions to sample locations
    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());
    
    let mtd:vector = normal.mult(depth);       
    //println!("Collision: mtd {:?} ", mtd);    
    let te:f64 = ( pa.get_elasticity() + pb.get_elasticity() ) * 0.5;
   // println!("Collision: te {:?} ", te);    
    let sum_inv_mass:f64 = im_pa_inv_mass + im_pb_inv_mass;
   // println!("Collision: sum_inv_mass {:?} ", sum_inv_mass);    
    
    // the total friction in a collision is combined but clamped to [0,1]
    let tf:f64 = 1.0;//clamp(1 - (pa.friction + pb.friction), 0, 1);
    
    // get the collision components, vn and vt
    let mut ca:collision = pa.get_components(normal);
    let mut cb:collision = pb.get_components(normal);

        // calculate the coefficient of restitution based on the mass, as the normal component
    
    let mult_b:&mut vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:vector = plus_b.divided_by(sum_inv_mass);

    let mult_a:&mut vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:vector = plus_a.divided_by(sum_inv_mass);
    // apply friction to the tangental component
    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    
    // scale the mtd by the ratio of the masses. heavier particles move less 
    let mtd_a:vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    
    // add the tangental component to the normal component for the new velocity 
    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, 1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  -1);
}

pub fn resolve_circle_circle(pa:&mut circle_particle, pb:&mut circle_particle, normal:&vector, depth:f64)
{
    let im_pb_inv_mass:f64 = pb.get_inv_mass();
    let im_pa_inv_mass:f64 = pa.get_inv_mass();
    
    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());
    
    let mtd:vector = normal.mult(depth);       

    let te:f64 = ( pa.get_elasticity() + pb.get_elasticity() ) * 0.5;

    let mut tf:f64  = 1.0 - (pa.get_friction() + pb.get_friction());
    if tf > 1.0
    {
        tf = 1.0;
    }
    if tf < 0.0
    {
        tf = 0.0;
    }

    let sum_inv_mass:f64 = im_pa_inv_mass + im_pb_inv_mass;
    
    
    let mut ca:collision = pa.get_components(normal);
    let mut cb:collision = pb.get_components(normal);
    
    let mult_b:&mut vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:vector = plus_b.divided_by(sum_inv_mass);

    let mult_a:&mut vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:vector = plus_a.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    

    let mtd_a:vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, 1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  -1);
}

pub fn resolve_collision_rect_rect(pa:&mut rectangle_particle, pb:&mut rectangle_particle, normal:vector, depth:f64)
{
    println!("Depth {}", depth);
    let im_pb_inv_mass:f64 = pb.get_inv_mass();
    let im_pa_inv_mass:f64 = pa.get_inv_mass();
    
    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());
    
    let mtd:vector = normal.mult(depth);       

    let te:f64 = ( pa.get_elasticity() + pb.get_elasticity() ) * 0.5;

    let mut tf:f64  = 1.0 - (pa.get_friction() + pb.get_friction());
    if tf > 1.0
    {
        tf = 1.0;
    }
    if tf < 0.0
    {
        tf = 0.0;
    }

    let sum_inv_mass:f64 = im_pa_inv_mass + im_pb_inv_mass;
    
    
    let mut ca:collision = pa.get_components(&normal);
    let mut cb:collision = pb.get_components(&normal);
    
    let mult_b:&mut vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:vector = plus_b.divided_by(sum_inv_mass);

    let mult_a:&mut vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:vector = plus_a.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    

    let mtd_a:vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, 1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  -1);
}

pub fn resolve_collision_rect_circ(pa:&mut circle_particle, pb:&mut rectangle_particle, normal:vector, depth:f64)
{
    println!("Depth {}", depth);
    let im_pb_inv_mass:f64 = pb.get_inv_mass();
    let im_pa_inv_mass:f64 = pa.get_inv_mass();
    
    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());
    
    let mtd:vector = normal.mult(depth);       

    let mut te:f64 = ( pa.get_elasticity() + pb.get_elasticity() ) * 0.5;
    if te > 1.0
    {
        te = 1.0;
    }
    if te < 0.0
    {
        te = 0.0;
    }
    let mut tf:f64  = 1.0 - (pa.get_friction() + pb.get_friction());
    if tf > 1.0
    {
        tf = 1.0;
    }
    if tf < 0.0
    {
        tf = 0.0;
    }

    let sum_inv_mass:f64 = im_pa_inv_mass + im_pb_inv_mass;
    
    
    let mut ca:collision = pa.get_components(&normal);
    let mut cb:collision = pb.get_components(&normal);
    
    let mult_b:&mut vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:vector = plus_b.divided_by(sum_inv_mass);

    let mult_a:&mut vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:vector = plus_a.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    

    let mtd_a:vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, 1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  -1);
}

pub fn solve(numContactPoints:usize, ca:Vec<vector>, cb:Vec<vector>, normal:vector, depthTime:f64, pa:&mut polygon_particle, pb:&mut polygon_particle)
{
    for i in 0..numContactPoints
    {
        resolveOverlap(&ca[i], &cb[i],&normal, depthTime, pa, pb);
    }
  
    for i in 0..numContactPoints
    {
        resolveOverlap(&ca[i], &cb[i], &normal, depthTime,  pa, pb);
    }
    let mut va:vector = vector::new(0.0,0.0);
    va.copy(&pa.get_velocity());
    let ava:f64 = pa.get_ang_velocity();
    let mut vb:vector = vector::new(0.0,0.0);
    vb.copy(&pb.get_velocity());
    let avb:f64 = pb.get_ang_velocity();
    for i in 0..numContactPoints
    {
        resolveCollision(normal.mult(-1.0), depthTime, &cb[i], &ca[i], pb, pa, &va, ava, &vb, avb);
    }	
}
                  //  c0:Vec<vector>, c1:Vec<vector>, normal:vector, depthTime:f64, pa:&mut polygon_particle, pb:&mut polygon_particle)
pub fn resolveOverlap(c0:&vector, c1:&vector, normal:&vector, depthTime:f64, pa:&mut polygon_particle, pb:&mut polygon_particle)
{
    let invMass0:f64 = pa.get_inv_mass();
    let invMass1:f64 = pb.get_inv_mass();
    let invMassTotal:f64 = invMass0 + invMass1;
    
    let diff:vector = c1.minus(c0);
    let relaxation:f64 = 0.5;
    
    //trace("diff "+diff);
    //trace(depthTime);
    
    //diff.multEquals(relaxation);
    
    let mut displace0:vector = vector::new(0.0,0.0);
    let mut displace1:vector = vector::new(0.0,0.0);
    
    if invMass0 > 0.0
    {
        displace0 = diff.mult(invMass0/invMassTotal);
        pa.set_curr(&pa.get_position().plus(&displace0));
       // pa.get_curr().plus_equals(&displace0);
        //pa.prev.plusEquals(displace0);
        if pa.get_inv_inertia() == 0.0
        {
            //pa.prev.plusEquals(displace0);
            pa.set_prev(&pa.get_position().plus(&displace0));
           // pa.get_prev().plus_equals(&displace0);
        }
    }
    if invMass1 > 0.0
    {
        displace1 = diff.mult(-invMass1/invMassTotal);
        //trace(pb.velocity.magnitude());
        pb.set_curr(&pb.get_position().plus(&displace1));
       // pb.get_curr().plus_equals(&displace1);
        //trace(" "+pb.velocity.magnitude());
        //pb.prev.plusEquals(displace1);
        if pb.get_inv_inertia() == 0.0
        {
            //trace(" uhh");
            //pb.get_prev().plus_equals(&displace1);
            pb.set_prev(&pb.get_position().plus(&displace1));
        }
    }
        //trace(" "+pb.velocity.magnitude());
}
pub fn resolve_collision_rect_rect2( pa:&mut rectangle_particle, pb:&mut rectangle_particle, normal:vector, depthTime:f64,)
{	
    //pre-computations
    let mtd = normal.mult(depthTime);
    let te = ( pa.get_elasticity() + pb.get_elasticity() ) * 0.5;
    let mut tf = 1.0- pa.get_friction() + pb.get_friction();
    if tf > 1.0
    {
        tf = 1.0;
    }
    if tf < 0.0
    {
        tf = 0.0;
    }
    let ma = pa.get_mass();
    let mb = pb.get_mass();
    let tm = ma + mb;

    let mut ca = pa.get_components(&normal);
    let mut cb = pb.get_components(&normal);

    let mut vn_a = cb.vn.mult((te + 1.0) * mb);
    vn_a.plus_equals(&ca.vn.mult(ma - te * mb));
    vn_a.div_equals(tm);

    let mut vn_b = ca.vn.mult((te + 1.0) * ma);
    vn_a.plus_equals(&cb.vn.mult(mb - te * ma));
    vn_a.div_equals(tm);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
 
    let mtd_a = mtd.mult(mb/tm);
    let mtd_b = mtd.mult(-ma/tm);

    if !pa.get_fixed()
    {
        pa.resolve_collision(&mtd_a, &vn_a.plus(&ca.vt), &normal, depthTime, -1)
    }
    if !pb.get_fixed()
    {
        pb.resolve_collision(&mtd_b, &vn_b.plus(&cb.vt), &normal, depthTime, 1)
    }
}			
pub fn resolveCollision(normal:vector, depthTime:f64, c0:&vector, c1:&vector, pa:&mut polygon_particle, pb:&mut polygon_particle, va:&vector, ava:f64, vb:&vector, avb:f64)
{	
    //pre-computations
    
    let r0:vector = c0.minus(pa.get_curr());
    let r1:vector = c1.minus(pa.get_curr());
    let T0:vector = r0.swap().times(&vector::new(-1.0, 1.0));//new Vector(-r0.y, r0.x);
    let T1:vector = r1.swap().times(&vector::new(-1.0, 1.0));
    let vp0:vector = va.minus(&T0.mult(ava));
    let vp1:vector = vb.minus(&T1.mult(avb));
    
    //impact velocity
    
    let vcoll:vector = vp0.minus(&vp1);
    let vn:f64 = vcoll.dot(&normal);
    let Vn:vector = normal.mult(vn);
    let mut Vt:vector = vcoll.minus(&Vn);
    
    //separation
    if vn > 0.0
    {
        return;
    }
    let mut vt:f64 = Vt.magnitude();
    //Vt.normalize();
    
    // compute impulse (friction and restitution).
    // ------------------------------------------
    //
    //									-(1+Cor)(Vel.norm)
    //			j =  ------------------------------------------------------------
    //			     [1/Ma + 1/Mb] + [Ia' * (ra x norm)²] + [Ib' * (rb x norm)²]
    
    let J:vector;
    let Jt:vector;
    let Jn:vector;
    
    let t0:f64 = (r0.cross(&normal)) * (r0.cross(&normal)) * pa.get_inv_inertia();
    let t1:f64 = (r1.cross(&normal)) * (r1.cross(&normal)) * pb.get_inv_inertia();
    let invMass0:f64 = pa.get_inv_mass();
    let invMass1:f64 = pb.get_inv_mass();
    let invMassTotal = invMass0 + invMass1;
    
    let denom:f64 = invMassTotal + t0 + t1;
    
    let jn:f64 = vn/denom;

    let restitution:f64 = clamp(pa.get_elasticity() + pb.get_elasticity(), 0.0, 1.0);
    Jn = normal.mult(-(1.0 + restitution) * jn);
    
    //if(useFriction){
    let totalFriction:f64 = clamp(pa.get_friction() + pb.get_friction(), 0.0, 1.0);
    Jt = Vt.normalize().mult(totalFriction * jn);
    
    J = Jn.plus(&Jt);
    
    // changes in momentum
    
    let dV0:vector = J.mult(invMass0);
    let dV1:vector = J.mult(-invMass1);

    
    let avdamping = 1.0;
    let dw0:f64 = -(r0.cross(&J)) * pa.get_inv_inertia() * avdamping;
    let dw1:f64 = (r1.cross(&J)) * pb.get_inv_inertia() * avdamping;
    
    // apply changes in momentum
    //trace(pa);
    //trace(dV0.magnitude());
    pa.resolve_velocities(dV0, dw0, normal.clone());
    pb.resolve_velocities(dV1, dw1, normal);
}

pub fn clamp(mut t:f64, min:f64, max:f64)->f64
{
    if t > max
    {t = max}
    if t < min
    {t = min}
    return t
}
