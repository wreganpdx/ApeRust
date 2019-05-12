 
 use crate::polygon_particle::polygon_particle;
 use crate::vector::vector;
 use crate::collision::collision;
 use crate::particle::particle;

pub fn resolve_particle_particle(pa:&mut polygon_particle, pb:&mut polygon_particle, normal:&vector, depth:f64)
{
    println!("Collision: {:?} {:?} {:?} {:?} ", pa.get_curr(), pb.get_curr(), normal, depth);
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
    println!("Collision: mtd {:?} ", mtd);    
    let te:f64 = pa.get_elasticity() + pb.get_elasticity();
    println!("Collision: te {:?} ", te);    
    let sum_inv_mass:f64 = im_pa_inv_mass + im_pb_inv_mass;
    println!("Collision: sum_inv_mass {:?} ", sum_inv_mass);    
    
    // the total friction in a collision is combined but clamped to [0,1]
    let tf:f64 = 1.0;//clamp(1 - (pa.friction + pb.friction), 0, 1);
    
    // get the collision components, vn and vt
    let mut ca:collision = pa.get_components(normal);
    let mut cb:collision = pb.get_components(normal);

        // calculate the coefficient of restitution based on the mass, as the normal component
    
    let mult_b:&mut vector = &mut ca.vn.mult((te + 1.0) * im_pb_inv_mass);
    let mult_b_2:&mut vector = &mut cb.vn.mult(im_pa_inv_mass - te * im_pb_inv_mass);
    let plus_b:&mut vector = &mut mult_b.plus(mult_b_2);
    let mut vn_b:vector = plus_b.div_equals(sum_inv_mass).clone();
    let mult_a:&mut vector = &mut cb.vn.mult((te + 1.0) * im_pa_inv_mass);
    let mult_a_2:&mut vector = &mut ca.vn.mult(im_pb_inv_mass - te * im_pa_inv_mass);
    let plus_a:&mut vector = &mut mult_a.plus(mult_a_2);
    let mut vn_a:vector = plus_a.div_equals(sum_inv_mass).clone();
    // apply friction to the tangental component
    ca.vt.mult_equals(tf);
    cb.vt.mult_equals(tf);
    
    // scale the mtd by the ratio of the masses. heavier particles move less 
    let mtd_a:vector = mtd.mult( im_pa_inv_mass / sum_inv_mass);     
    let mtd_b:vector = mtd.mult(-im_pb_inv_mass / sum_inv_mass);
    
    // add the tangental component to the normal component for the new velocity 
    vn_a.plus_equals(&ca.vt);
    vn_b.plus_equals(&cb.vt);
    
    
    pa.resolve_collision(&mtd_a, &vn_a, &normal, depth, -1);
    pb.resolve_collision(&mtd_b, &vn_b, &normal, depth,  1);
}
        