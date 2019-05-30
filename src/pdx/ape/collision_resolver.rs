
use crate::rectangle_particle::RectangleParticle;
use crate::vector::Vector;
use crate::collision::Collision;
use crate::particle::Particle;
use crate::circle_particle::CircleParticle;
use std::f64;


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


pub fn clamp(mut t:f64, min:f64, max:f64)->f64
{
    if t > max
    {t = max}
    if t < min
    {t = min}
    return t
}
