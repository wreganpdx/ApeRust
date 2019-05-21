use crate::particle::Particle;
use crate::polygon_particle::PolygonParticle;
use crate::rectangle_particle::RectangleParticle;
use crate::circle_particle::CircleParticle;
use crate::interval::Interval;
use crate::vector::Vector;
use crate::ap_engine::APValues;
use crate::collision_resolver;
//use std::any::Any;
//use num_traits::float::FloatCore;
use std::f64;

#[allow(unused_variables)]
pub fn test_polygon_vs_polygon(ra:& mut PolygonParticle, rb:&mut PolygonParticle, p_size:usize, p2_size:usize)->bool
{	
	//println!("TESTING COLLISION psize: {}, p2size: {} ", p_size, p2_size);
	let mut collision_normal:Vector = Vector::new(0.0,0.0);
	let mut collision_depth:f64 = num_traits::float::Float::max_value();//should be float max... 
	for i in 0..p_size
	{
		//println!("TESTING COLLISION paxes: {} ", i);
		let tuple = ra.get_interval_and_axe(i);
		let depth_a:f64 = test_intervals(&tuple.1, rb.get_projection(&tuple.0));
		if depth_a == 0.0
		{return false;}
		let abs_a:f64 = depth_a.abs();
		if abs_a < collision_depth.abs() 
		{
			collision_normal.copy(&tuple.0);
			collision_depth = depth_a;
		}
	}
	for i in 0..p2_size
	{
		//println!("TESTING COLLISION p2axes: {} ", i);
		let tuple = rb.get_interval_and_axe(i);
		let depth_b:f64 = test_intervals(&tuple.1, ra.get_projection(&tuple.0));
		if depth_b == 0.0
		{return false;}
		let abs_b:f64 = depth_b.abs();
		if abs_b < collision_depth.abs()
		{
			collision_normal.copy(&tuple.0);
			collision_depth = depth_b;
		}
	}


	let ca:Vec<Vector> = Vec::new();
	let cb:Vec<Vector> = Vec::new();

	//rotate(&(-f64::consts::PI/2.0)  //one possibility to fix bug is rotate normal, though this seems to pose more problems.
	println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	collision_resolver::resolve_particle_particle(ra, rb, &collision_normal, collision_depth);
	println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	//println!("COLLISION");
	return true;
}

pub fn test_rect_vs_rect(ra:& mut RectangleParticle, rb:&mut RectangleParticle)->bool
{	
	ra.set_samp(ra.get_position());
	rb.set_samp(rb.get_position());
	let mut collision_normal:Vector = Vector::new(0.0,0.0);
	let mut collision_depth:f64 = 1000000.0; 
	//println!("{}", collision_depth);
	for i in 0..2
	{

		let axis_a = &ra.get_axe(i);
		let depth_a = test_intervals(ra.get_projection(axis_a), rb.get_projection(axis_a));
		let abs_a:f64 = depth_a.abs();
		if abs_a == 0.0 
		{
		//	println!("ret false");
			return false;
		}
		let axis_b = &rb.get_axe(i);
		let depth_b = test_intervals(ra.get_projection(axis_b), rb.get_projection(axis_b));
		let abs_b:f64 = depth_b.abs();
	//	println!("absA : {} , absB {} ",absA, absB);
		if abs_b == 0.0
		{
		//	println!("ret false");
			return false;
		}

		if abs_a < collision_depth.abs() || abs_b < collision_depth.abs() 
		{
			let altb:bool = abs_a < abs_b;
			if altb 
			{
				collision_normal.copy(axis_a);
			}
			else
			{
				collision_normal.copy(axis_b);
			}
			if altb
			{
				collision_depth = depth_a;
			}
			else
			{
				collision_depth = depth_b;
			}
		}
	}

	//println!("COLLISON");
	//rotate(&(-f64::consts::PI/2.0)  //one possibility to fix bug is rotate normal, though this seems to pose more problems.
	//println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	collision_resolver::resolve_collision_rect_rect(ra, rb, collision_normal, collision_depth);
	//println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	//println!("COLLISION");
	return true;
}

pub fn test_circ_vs_rect(circle:& mut CircleParticle, rect:&mut RectangleParticle)->bool
{	
	circle.set_samp(circle.get_position());
	rect.set_samp(rect.get_position());
	let mut collision_normal:Vector = Vector::new(0.0,0.0);
	let mut collision_depth:f64 = 1000000.0; 
	let mut depths = Vec::new();
	//println!("{}", collision_depth);
	for i in 0..2
	{
		let axis_b = &rect.get_axe(i);
		let depth_b = test_intervals(circle.get_projection(axis_b), rect.get_projection(axis_b));
		let abs_b:f64 = depth_b.abs();
		if abs_b == 0.0
		{
			return false;
		}

		if abs_b < collision_depth.abs() 
		{
			collision_normal.copy(axis_b);
			collision_depth = depth_b;
		}
		depths.push(depth_b);
	}

	let r = circle.get_radius().clone();
	if depths[0].abs() < r && depths[1].abs() < r
	{
		let vert = closest_vertex_on_obb(circle.get_samp(), rect);
		collision_normal.copy(&vert.minus(&circle.get_samp()));
		let mag = collision_normal.magnitude();
		collision_depth = r - mag;
		if collision_depth > 0.0
		{
			collision_normal.div_equals(mag);

		}
		else
		{
			return false;
		}
	}
	collision_resolver::resolve_collision_rect_circ(circle, rect, collision_normal, collision_depth);
	return true;
}

pub fn closest_vertex_on_obb(p:Vector, r:&mut RectangleParticle)-> Vector
{
	
	let d:Vector = p.minus(&r.get_samp());
	let mut q:Vector = r.get_samp().clone();

	for i in 0..2
	{
		let mut dist:f64 = d.dot(&r.get_axe(i));

		if dist >= 0.0
		{ dist = r.get_extent(i);}
		else if dist < 0.0 {dist = -r.get_extent(i);}

		q.plus_equals(&r.get_axe(i).mult(dist));
	}
	return q;

}

pub fn test_circ_vs_circ(ra:& mut CircleParticle, rb:&mut CircleParticle)->bool
{		
	ra.set_samp(ra.get_position());
	rb.set_samp(rb.get_position());
	let depth_a = test_intervals(ra.get_interval_x(), rb.get_interval_x());
	if depth_a == 0.0
	{
		return false;
	}
	let depth_b = test_intervals(ra.get_interval_y(), rb.get_interval_y());
	if depth_b == 0.0
	{
		return false;
	}
	let mut collision_normal:Vector = ra.get_position().minus(&rb.get_position());
	//println!("collision_normal {:?}", collision_normal);
	let mag = collision_normal.clone().magnitude();
	//println!("collision_normal {:?}", collision_normal);
	let collision_depth:f64 = ra.get_radius() + rb.get_radius() - mag;

	if collision_depth > 0.0
	{
		collision_normal.div_equals(mag);
		collision_resolver::resolve_circle_circle(ra, rb, &collision_normal, collision_depth);
	}
	return true;
}


pub fn test_rigid_polygon_vs_rigid_polygon(ra:& mut PolygonParticle, rb:&mut PolygonParticle, p_size:usize, p2_size:usize)->bool
{	
	//println!("TESTING COLLISION psize: {}, p2size: {} ", p_size, p2_size);
	let mut collision_normal:Vector = Vector::new(0.0,0.0);
	let mut collision_depth:f64 = num_traits::float::Float::max_value();//should be float max... 
	let offset:Vector = ra.get_curr().minus(&rb.get_curr());
	for i in 0..p_size
	{
		//println!("TESTING COLLISION paxes: {} ", i);
		let tuple = ra.get_interval_and_axe(i);
		let depth_a:f64 = test_intervals(&tuple.1, rb.get_projection(&tuple.0));
		if depth_a == 0.0
		{return false;}
		let abs_a:f64 = depth_a.abs();
		if abs_a < collision_depth.abs() 
		{
			collision_normal.copy(&tuple.0);
			collision_depth = depth_a;
		}
	}
	for i in 0..p2_size
	{
		//println!("TESTING COLLISION p2axes: {} ", i);
		let tuple = rb.get_interval_and_axe(i);
		let depth_b:f64 = test_intervals(&tuple.1, ra.get_projection(&tuple.0));
		if depth_b == 0.0
		{return false;}
		let abs_b:f64 = depth_b.abs();
		if abs_b < collision_depth.abs()
		{
			collision_normal.copy(&tuple.0);
			collision_depth = depth_b;
		}
	}
	if collision_normal.dot(&offset) < 0.0
	{
		collision_normal.mult_equals(-1.0);
	}
	let col_normal:&mut Vector = &mut collision_normal.clone();
	let tuple = find_contacts(ra, rb, col_normal);

	//rotate(&(-f64::consts::PI/2.0)  //one possibility to fix bug is rotate normal, though this seems to pose more problems.
	println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	//collision_resolver::resolve_particle_particle(ra, rb, &collision_normal, collision_depth);
	collision_resolver::solve(tuple.0, tuple.1, tuple.2, collision_normal, collision_depth, ra, rb);
	println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	//println!("COLLISION");
	return true;
}

pub fn find_contacts(ra:& mut PolygonParticle, rb:&mut PolygonParticle, normal:&mut Vector)-> (usize, Vec<Vector>, Vec<Vector>)
{

	let s0:&mut Vec<Vector> = &mut find_support_points(normal, ra);
	let s1:&mut Vec<Vector> = &mut find_support_points(&normal.mult(-1.0), rb);
	return convert_support_points_to_contacts(normal, s0, s1);
}
pub fn find_support_points(normal:&Vector, rp:&mut PolygonParticle)->Vec<Vector>
{
			
	let mut support_points:Vec<Vector> = Vec::new();
	//let vertices:&Vec<Vector> = rp.get_vertices();
	let tuple = rp.get_vertices_and_position();
	let vertices:&Vec<Vector> = tuple.0;
	let position = tuple.1;
	let norm:Vector = normal.clone();
	let mut d:Vec<f64> = Vec::new();
	
	let mut cur_vec:&Vector = &vertices[0];
	d.push(cur_vec.dot(&norm));
	let mut dmin:f64 = d[0];;
	
	for i in 1..vertices.len()
	{
		cur_vec = &vertices[i];
		d.push(cur_vec.dot(&norm));
		if d[i] < dmin
		{
			dmin = d[i];
		}
	}
	
	// we limit the number of support points to only 2. 
	let threshold:f64 = dmin + 0.5;//.003 originally
	
	for i in 0..vertices.len()
	{
		if d[i] < threshold && support_points.len() < 2
		{
			let contact:Vector = position.plus(&vertices[i]);
			support_points.push(contact);
		}
	}
	return support_points;			
}

fn convert_support_points_to_contacts(normal:&mut Vector, s0:&mut Vec<Vector>, s1:&mut Vec<Vector>)-> (usize, Vec<Vector>, Vec<Vector>)
{
	let mut c0:Vec<Vector> = Vec::new();
	let mut c1:Vec<Vector> = Vec::new();
	let mut c_num:usize = 0;
	let s0num:usize = s0.len();
	let s1num:usize = s1.len();
	if s0num == 0 || s1num == 0
	{
		return (0, c0, c1);
	}
	if s0num == 1 && s1num == 1 
	{
		c0.push(s0[0].clone());
		c1.push(s1[0].clone());
		c_num += 1;
		return (c_num, c0, c1);
	}
//let x = -1.0 * &normal.y.clone();
	//let y = &normal.x;
	let x_perp:Vector = normal.clone().swap().times(&Vector::new(-1.0, 1.0));//Vector::new(x, y);
	
	let mut curr_s0:&Vector = &s0[0];
	let mut curr_s1:&Vector = &s1[0];
	let mut min0:f64 = curr_s0.dot(&x_perp);
	let mut max0:f64 = min0;
	let mut min1:f64 = curr_s1.dot(&x_perp);
	let mut max1:f64 = min1;
	
	if s0num == 2
	{
		curr_s0 = &s0[1];
		max0 = curr_s0.dot(&x_perp);
		if max0 < min0 
		{
			let temp0:f64 = min0;
			min0 = max0;
			max0 = temp0;
			let temp_vec0:Vector = s0[0].clone();
			s0[0] = s0[1].clone();
			s0[1] = temp_vec0.clone();
		}
	}
	if s1num == 2
	{
		curr_s1 = &s1[1];
		max1 = curr_s1.dot(&x_perp);
		if max1 < min1 
		{
			let temp1:f64 = min1;
			min1 = max1;
			max1 = temp1;
			let temp_vec1:Vector = s1[0].clone();
			s1[0] = s1[1].clone();
			s1[1] = temp_vec1.clone();
		}
	}
	
	if min0 > max1 || min1 > max0 
	{
		return (0, c0, c1);
	}
	
	//let p_seg:Vector = Vector::new(0.0,0.0);
	if min0 > min1
	{
		let p_seg = project_point_on_segment(&s0[0], &s1[0], &s1[1]);

		c0.push(s0[0].clone());
		c1.push(p_seg);
		c_num += 1;
	}
	else
	{
		let p_seg = project_point_on_segment(&s1[0], &s0[0], &s0[1]);

		c0.push(p_seg);
		c1.push(s1[0].clone());
		c_num += 1;
	}
	
	if max0 != min0 && max1 != min1 
	{
		if max0 < max1 
		{
			let p_seg = project_point_on_segment(&s0[1], &s1[0], &s1[1]);

			c0.push(s0[1].clone());
			c1.push(p_seg);
			c_num += 1;
		}
		else
		{
			let p_seg = project_point_on_segment(&s1[1], &s0[0], &s0[1]);

			c0.push(p_seg);
			c1.push(s1[1].clone());
			c_num += 1;
		}
	}
	return (c_num, c0,c1);
}
pub fn project_point_on_segment(v:&Vector, a:&Vector, b:&Vector)->Vector
{		
	let a_v:Vector = v.minus(a);
	let a_b:Vector = b.minus(a);
	let mut t:f64 = (a_v.dot(&a_b))/(a_b.dot(&a_b));
	
	if t < 0.0 
	{
		t = 0.0;
	}
	else if  t > 1.0 
	{
		t = 1.0;
	}
	
	let point:Vector = a.plus(&a_b.mult(t));
	return point;
}


fn test_intervals(interval_a:&Interval, interval_b:&Interval)->f64 
{

	if interval_a.max < interval_b.min
	{return 0.0;}
	if interval_b.max < interval_a.min
	{return 0.0;}

	//println!("interval a : {:?} interval b : {:?}", interval_a, interval_b);
	let len_a:f64 = interval_b.max - interval_a.min;
	let len_b:f64 = interval_b.min - interval_a.max;

	if len_a.abs() < len_b.abs()
	{
		return len_a;
	}
	return len_b;
	//return (Math.abs(lenA) < Math.abs(lenB)) ? lenA : lenB;
}

pub fn check_rectangle_vs_rects(p:&mut RectangleParticle, col:&mut Vec<RectangleParticle>, _ap:&APValues)
{
	let length2:usize = col.len();

	for j in 0..length2
	{
		let mut p2 = col.remove(j);
		if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed())
		{
			col.insert(j, p2);
			continue;
		}
		test_rect_vs_rect(p,&mut p2);
		col.insert(j, p2);
	}
}

pub fn check_rectangle_vs_circs(p:&mut RectangleParticle, col:&mut Vec<CircleParticle>, _ap:&APValues)
{
	let length2:usize = col.len();

	for j in 0..length2
	{
		let mut p2 = col.remove(j);
		if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed())
		{
			col.insert(j, p2);
			continue;
		}
		test_circ_vs_rect(&mut p2,p);
		col.insert(j, p2);
	}
}

pub fn check_circ_vs_circ(p:&mut CircleParticle, col:&mut Vec<CircleParticle>, _ap:&APValues)
{
	let length2:usize = col.len();

	for j in 0..length2
	{
		let mut p2 = col.remove(j);
		if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed())
		{
			col.insert(j, p2);
			continue;
		}
		test_circ_vs_circ(&mut p2,p);
		col.insert(j, p2);
	}
}

pub fn check_circ_vs_rects(p:&mut CircleParticle, col:&mut Vec<RectangleParticle>, _ap:&APValues)
{
	let length2:usize = col.len();

	for j in 0..length2
	{
		let mut p2 = col.remove(j);
		if !p2.get_collidable() || (p2.get_fixed() && p.get_fixed())
		{
			col.insert(j, p2);
			continue;
		}
		test_circ_vs_rect(p, &mut p2);
		col.insert(j, p2);
	}
}
