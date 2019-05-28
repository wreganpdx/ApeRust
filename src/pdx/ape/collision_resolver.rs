use crate::polygon_particle::PolygonParticle;
use crate::rectangle_particle::RectangleParticle;
use crate::vector::Vector;
use crate::collision::Collision;
use crate::particle::Particle;
use crate::circle_particle::CircleParticle;
use std::f64;

pub fn resolve_particle_particle(pa:&mut PolygonParticle, pb:&mut PolygonParticle, normal:&Vector, depth:f64)
{
   // println!("Collision: {:?} {:?} {:?} {:?} ", pa.get_curr(), pb.get_curr(), normal, depth);
    let im_pb_inv_mass:f64 = pb.get_inv_mass();
    let im_pa_inv_mass:f64 = pa.get_inv_mass();
    // {
        /*
        let im_pb:&PolygonParticle = pb;
        let im_pa:&PolygonParticle = pa;
        im_pb_inv_mass = im_pb.get_inv_mass();
        im_pa_inv_mass = im_pa.get_inv_mass();
        */
    // }
    
    // a collision has occured. set the current positions to sample locations
    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());
    
    let mtd:Vector = normal.mult(depth);       
    //println!("Collision: mtd {:?} ", mtd);    
    let te:f64 = ( pa.get_elasticity() + pb.get_elasticity() ) * 0.5;
   // println!("Collision: te {:?} ", te);    
    let sum_inv_mass:f64 = im_pa_inv_mass + im_pb_inv_mass;
   // println!("Collision: sum_inv_mass {:?} ", sum_inv_mass);    
    
    // the total friction in a collision is combined but clamped to [0,1]
    let tf:f64 = 1.0;//clamp(1 - (pa.friction + pb.friction), 0, 1);
    
    // get the collision components, vn and vt
    let mut ca:Collision = pa.get_components(normal);
    let mut cb:Collision = pb.get_components(normal);

        // calculate the coefficient of restitution based on the mass, as the normal component
    
    let mult_b:&mut Vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut Vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut Vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:Vector = plus_b.divided_by(sum_inv_mass);

    let mult_a:&mut Vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut Vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut Vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:Vector = plus_a.divided_by(sum_inv_mass);
    // apply friction to the tangental component
    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    
    // scale the mtd by the ratio of the masses. heavier particles move less 
    let mtd_a:Vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:Vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    
    // add the tangental component to the normal component for the new velocity 
    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, 1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  -1);
}

pub fn resolve_circle_circle(pa:&mut CircleParticle, pb:&mut CircleParticle, normal:&Vector, depth:f64)
{
    let im_pb_inv_mass:f64 = pb.get_inv_mass();
    let im_pa_inv_mass:f64 = pa.get_inv_mass();
    
    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());
    
    let mtd:Vector = normal.mult(depth);       

    let te:f64 = ( pa.get_elasticity() + pb.get_elasticity() );

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
    
    
    let mut ca:Collision = pa.get_components(normal);
    let mut cb:Collision = pb.get_components(normal);
    
    let mult_b:&mut Vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut Vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut Vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:Vector = plus_b.divided_by(sum_inv_mass);

    let mult_a:&mut Vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut Vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut Vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:Vector = plus_a.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    

    let mtd_a:Vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:Vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, 1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  -1);
}

pub fn resolve_collision_rect_rect(pa:&mut RectangleParticle, pb:&mut RectangleParticle, normal:Vector, depth:f64)
{
    //println!("Depth {}", depth);
    let im_pb_inv_mass:f64 = pb.get_inv_mass();
    let im_pa_inv_mass:f64 = pa.get_inv_mass();
    
    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());
    
    let mtd:Vector = normal.mult(depth);       

    let te:f64 = ( pa.get_elasticity() + pb.get_elasticity() );

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
    
    
    let mut ca:Collision = pa.get_components(&normal);
    let mut cb:Collision = pb.get_components(&normal);
    
    let mult_b:&mut Vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut Vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut Vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:Vector = plus_b.divided_by(sum_inv_mass);

    let mult_a:&mut Vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut Vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut Vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:Vector = plus_a.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    

    let mtd_a:Vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:Vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, 1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  -1);
}

pub fn resolve_collision_rect_circ(pa:&mut CircleParticle, pb:&mut RectangleParticle, normal:Vector, depth:f64)
{
    //println!("Depth {}", depth);
    let im_pb_inv_mass:f64 = pb.get_inv_mass();
    let im_pa_inv_mass:f64 = pa.get_inv_mass();
    
    pa.set_curr(&pa.get_samp());
    pb.set_curr(&pb.get_samp());
    
    let mtd:Vector = normal.mult(depth);       

    let mut te:f64 = ( pa.get_elasticity() + pb.get_elasticity() ) ;
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
    
    
    let mut ca:Collision = pa.get_components(&normal);
    let mut cb:Collision = pb.get_components(&normal);
    
    let mult_b:&mut Vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut Vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut Vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:Vector = plus_b.divided_by(sum_inv_mass);

    let mult_a:&mut Vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut Vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut Vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:Vector = plus_a.divided_by(sum_inv_mass);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    

    let mtd_a:Vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:Vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    

    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, 1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  -1);
}

pub fn solve(num_contact_points:usize, ca:Vec<Vector>, cb:Vec<Vector>, normal:Vector, depth_time:f64, pa:&mut PolygonParticle, pb:&mut PolygonParticle)
{
    for i in 0..num_contact_points
    {
        resolve_overlap(&ca[i], &cb[i],&normal, depth_time, pa, pb);
    }
  
    for i in 0..num_contact_points
    {
        resolve_overlap(&ca[i], &cb[i], &normal, depth_time,  pa, pb);
    }
    let mut va:Vector = Vector::new(0.0,0.0);
    va.copy(&pa.get_velocity());
    let ava:f64 = pa.get_ang_velocity();
    let mut vb:Vector = Vector::new(0.0,0.0);
    vb.copy(&pb.get_velocity());
    let avb:f64 = pb.get_ang_velocity();
    for i in 0..num_contact_points
    {
        resolve_collisions(normal.mult(-1.0), depth_time, &cb[i], &ca[i], pb, pa, &va, ava, &vb, avb);
    }	
}
                  //  c0:Vec<Vector>, c1:Vec<Vector>, normal:Vector, depth_time:f64, pa:&mut PolygonParticle, pb:&mut PolygonParticle)
pub fn resolve_overlap(c0:&Vector, c1:&Vector, _normal:&Vector, _depth_time:f64, pa:&mut PolygonParticle, pb:&mut PolygonParticle)
{
    let inv_mass0:f64 = pa.get_inv_mass();
    let inv_mass1:f64 = pb.get_inv_mass();
    let inv_mass_total:f64 = inv_mass0 + inv_mass1;
    
    let diff:Vector = c1.minus(c0);
    let _relaxation:f64 = 0.5;
    
    //trace("diff "+diff);
    //trace(depth_time);
    
    //diff.multEquals(relaxation);
    
    let mut _displace0:Vector = Vector::new(0.0,0.0);
    let mut _displace1:Vector = Vector::new(0.0,0.0);
    
    if inv_mass0 > 0.0
    {
        _displace0 = diff.mult(inv_mass0/inv_mass_total);
        pa.set_curr(&pa.get_position().plus(&_displace0));
       // pa.get_curr().plus_equals(&displace0);
        //pa.prev.plusEquals(displace0);
        if pa.get_inv_inertia() == 0.0
        {
            //pa.prev.plusEquals(displace0);
            pa.set_prev(&pa.get_position().plus(&_displace0));
           // pa.get_prev().plus_equals(&displace0);
        }
    }
    if inv_mass1 > 0.0
    {
        _displace1 = diff.mult(-inv_mass1/inv_mass_total);
        //trace(pb.velocity.magnitude());
        pb.set_curr(&pb.get_position().plus(&_displace1));
       // pb.get_curr().plus_equals(&displace1);
        //trace(" "+pb.velocity.magnitude());
        //pb.prev.plusEquals(displace1);
        if pb.get_inv_inertia() == 0.0
        {
            //trace(" uhh");
            //pb.get_prev().plus_equals(&displace1);
            pb.set_prev(&pb.get_position().plus(&_displace1));
        }
    }
        //trace(" "+pb.velocity.magnitude());
}
pub fn resolve_collision_rect_rect2( pa:&mut RectangleParticle, pb:&mut RectangleParticle, normal:Vector, depth_time:f64,)
{	
    //pre-computations
    let mtd = normal.mult(depth_time);
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

    let vn_b = ca.vn.mult((te + 1.0) * ma);
    vn_a.plus_equals(&cb.vn.mult(mb - te * ma));
    vn_a.div_equals(tm);

    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
 
    let mtd_a = mtd.mult(mb/tm);
    let mtd_b = mtd.mult(-ma/tm);

    if !pa.get_fixed()
    {
        pa.resolve_collision(&mtd_a, &vn_a.plus(&ca.vt), &normal, depth_time, -1)
    }
    if !pb.get_fixed()
    {
        pb.resolve_collision(&mtd_b, &vn_b.plus(&cb.vt), &normal, depth_time, 1)
    }
}			
pub fn resolve_collisions(normal:Vector, _depth_time:f64, c0:&Vector, c1:&Vector, pa:&mut PolygonParticle, pb:&mut PolygonParticle, va:&Vector, ava:f64, vb:&Vector, avb:f64)
{	
    //pre-computations
    
    let r0:Vector = c0.minus(pa.get_curr());
    let r1:Vector = c1.minus(pa.get_curr());
    let t_0:Vector = r0.swap().times(&Vector::new(-1.0, 1.0));//new Vector(-r0.y, r0.x);
    let t_1:Vector = r1.swap().times(&Vector::new(-1.0, 1.0));
    let vp0:Vector = va.minus(&t_0.mult(ava));
    let vp1:Vector = vb.minus(&t_1.mult(avb));
    
    //impact velocity
    
    let vcoll:Vector = vp0.minus(&vp1);
    let vn:f64 = vcoll.dot(&normal);
    let v_n:Vector = normal.mult(vn);
    let mut v_t:Vector = vcoll.minus(&v_n);
    
    //separation
    if vn > 0.0
    {
        return;
    }
    let mut _vt:f64 = v_t.magnitude();
    //v_t.normalize();
    
    // compute impulse (friction and restitution).
    // ------------------------------------------
    //
    //									-(1+Cor)(Vel.norm)
    //			j =  ------------------------------------------------------------
    //			     [1/Ma + 1/Mb] + [Ia' * (ra x norm)²] + [Ib' * (rb x norm)²]
    
    let j_0:Vector;
    let j_t:Vector;
    let j_n:Vector;
    
    let t0:f64 = (r0.cross(&normal)) * (r0.cross(&normal)) * pa.get_inv_inertia();
    let t1:f64 = (r1.cross(&normal)) * (r1.cross(&normal)) * pb.get_inv_inertia();
    let inv_mass0:f64 = pa.get_inv_mass();
    let inv_mass1:f64 = pb.get_inv_mass();
    let inv_mass_total = inv_mass0 + inv_mass1;
    
    let denom:f64 = inv_mass_total + t0 + t1;
    
    let jn:f64 = vn/denom;

    let restitution:f64 = clamp(pa.get_elasticity() + pb.get_elasticity(), 0.0, 1.0);
    j_n = normal.mult(-(1.0 + restitution) * jn);
    
    //if(useFriction){
    let total_friction:f64 = clamp(pa.get_friction() + pb.get_friction(), 0.0, 1.0);
    j_t = v_t.normalize().mult(total_friction * jn);
    
    j_0 = j_n.plus(&j_t);
    
    // changes in momentum
    
    let d_v0:Vector = j_0.mult(inv_mass0);
    let d_v1:Vector = j_0.mult(-inv_mass1);

    
    let avdamping = 1.0;
    let dw0:f64 = -(r0.cross(&j_0)) * pa.get_inv_inertia() * avdamping;
    let dw1:f64 = (r1.cross(&j_0)) * pb.get_inv_inertia() * avdamping;
    
    // apply changes in momentum
    //trace(pa);
    //trace(d_v0.magnitude());
    pa.resolve_velocities(d_v0, dw0, normal.clone());
    pb.resolve_velocities(d_v1, dw1, normal);
}

pub fn clamp(mut t:f64, min:f64, max:f64)->f64
{
    if t > max
    {t = max}
    if t < min
    {t = min}
    return t
}
