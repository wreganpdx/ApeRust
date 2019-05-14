use crate::particle::particle;
use crate::polygon_particle::polygon_particle;
use crate::rectangle_particle::rectangle_particle;
use crate::interval::interval;
use crate::vector::vector;
use crate::collision_resolver;
use std::any::Any;
use num_traits::float::FloatCore;
use std::f64;

#[allow(unused_variables)]
pub fn test_polygon_vs_polygon(ra:& mut polygon_particle, rb:&mut polygon_particle, p_size:usize, p2_size:usize)->bool
{	
	//println!("TESTING COLLISION psize: {}, p2size: {} ", p_size, p2_size);
	let mut collision_normal:vector = vector::new(0.0,0.0);
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


	let mut ca:Vec<vector> = Vec::new();
	let mut cb:Vec<vector> = Vec::new();

	//rotate(&(-f64::consts::PI/2.0)  //one possibility to fix bug is rotate normal, though this seems to pose more problems.
	println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	collision_resolver::resolve_particle_particle(ra, rb, &collision_normal, collision_depth);
	println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	//println!("COLLISION");
	return true;
}

pub fn test_rect_vs_rect(ra:& mut rectangle_particle, rb:&mut rectangle_particle)->bool
{		
	let mut collision_normal:vector = vector::new(0.0,0.0);
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

		if abs_a < collision_depth || abs_b < collision_depth 
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


pub fn test_rigid_polygon_vs_rigid_polygon(ra:& mut polygon_particle, rb:&mut polygon_particle, p_size:usize, p2_size:usize)->bool
{	
	//println!("TESTING COLLISION psize: {}, p2size: {} ", p_size, p2_size);
	let mut collision_normal:vector = vector::new(0.0,0.0);
	let mut collision_depth:f64 = num_traits::float::Float::max_value();//should be float max... 
	let offset:vector = ra.get_curr().minus(&rb.get_curr());
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
	let mut col_normal:&mut vector = &mut collision_normal.clone();
	let tuple = find_contacts(ra, rb, col_normal);

	//rotate(&(-f64::consts::PI/2.0)  //one possibility to fix bug is rotate normal, though this seems to pose more problems.
	println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	//collision_resolver::resolve_particle_particle(ra, rb, &collision_normal, collision_depth);
	collision_resolver::solve(tuple.0, tuple.1, tuple.2, collision_normal, collision_depth, ra, rb);
	println!("Collision: {:?} {:?}", ra.get_curr(), rb.get_curr());
	//println!("COLLISION");
	return true;
}

pub fn find_contacts(ra:& mut polygon_particle, rb:&mut polygon_particle, normal:&mut vector)-> (usize, Vec<vector>, Vec<vector>)
{

	let s0:&mut Vec<vector> = &mut find_support_points(normal, ra);
	let s1:&mut Vec<vector> = &mut find_support_points(&normal.mult(-1.0), rb);
	return convert_support_points_to_contacts(normal, s0, s1);
}
pub fn find_support_points(normal:&vector, rp:&mut polygon_particle)->Vec<vector>
{
			
	let mut support_points:Vec<vector> = Vec::new();
	//let vertices:&Vec<vector> = rp.get_vertices();
	let tuple = rp.get_vertices_and_position();
	let vertices:&Vec<vector> = tuple.0;
	let position = tuple.1;
	let norm:vector = normal.clone();
	let mut d:Vec<f64> = Vec::new();
	
	let mut cur_vec:&vector = &vertices[0];
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
			let contact:vector = position.plus(&vertices[i]);
			support_points.push(contact);
		}
	}
	return support_points;			
}

fn convert_support_points_to_contacts(normal:&mut vector, s0:&mut Vec<vector>, s1:&mut Vec<vector>)-> (usize, Vec<vector>, Vec<vector>)
{
	let mut c0:Vec<vector> = Vec::new();
	let mut c1:Vec<vector> = Vec::new();
	let mut cNum:usize = 0;
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
		cNum += 1;
		return (cNum, c0, c1);
	}
//let x = -1.0 * &normal.y.clone();
	//let y = &normal.x;
	let xPerp:vector = normal.clone().swap().times(&vector::new(-1.0, 1.0));//vector::new(x, y);
	
	let mut currS0:&vector = &s0[0];
	let mut currS1:&vector = &s1[0];
	let mut min0:f64 = currS0.dot(&xPerp);
	let mut max0:f64 = min0;
	let mut min1:f64 = currS1.dot(&xPerp);
	let mut max1:f64 = min1;
	
	if s0num == 2
	{
		currS0 = &s0[1];
		max0 = currS0.dot(&xPerp);
		if max0 < min0 
		{
			let temp0:f64 = min0;
			min0 = max0;
			max0 = temp0;
			let temp_vec0:vector = s0[0].clone();
			s0[0] = s0[1].clone();
			s0[1] = temp_vec0.clone();
		}
	}
	if s1num == 2
	{
		currS1 = &s1[1];
		max1 = currS1.dot(&xPerp);
		if max1 < min1 
		{
			let temp1:f64 = min1;
			min1 = max1;
			max1 = temp1;
			let temp_vec1:vector = s1[0].clone();
			s1[0] = s1[1].clone();
			s1[1] = temp_vec1.clone();
		}
	}
	
	if min0 > max1 || min1 > max0 
	{
		return (0, c0, c1);
	}
	
	//let pSeg:vector = vector::new(0.0,0.0);
	if min0 > min1
	{
		let pSeg = project_point_on_segment(&s0[0], &s1[0], &s1[1]);

		c0.push(s0[0].clone());
		c1.push(pSeg);
		cNum += 1;
	}
	else
	{
		let pSeg = project_point_on_segment(&s1[0], &s0[0], &s0[1]);

		c0.push(pSeg);
		c1.push(s1[0].clone());
		cNum += 1;
	}
	
	if max0 != min0 && max1 != min1 
	{
		if max0 < max1 
		{
			let pSeg = project_point_on_segment(&s0[1], &s1[0], &s1[1]);

			c0.push(s0[1].clone());
			c1.push(pSeg);
			cNum += 1;
		}
		else
		{
			let pSeg = project_point_on_segment(&s1[1], &s0[0], &s0[1]);

			c0.push(pSeg);
			c1.push(s1[1].clone());
			cNum += 1;
		}
	}
	return (cNum, c0,c1);
}
pub fn project_point_on_segment(v:&vector, a:&vector, b:&vector)->vector
{		
	let AV:vector = v.minus(a);
	let AB:vector = b.minus(a);
	let mut t:f64 = (AV.dot(&AB))/(AB.dot(&AB));
	
	if t < 0.0 
	{
		t = 0.0;
	}
	else if  t > 1.0 
	{
		t = 1.0;
	}
	
	let point:vector = a.plus(&AB.mult(t));
	return point;
}


fn test_intervals(interval_a:&interval, interval_b:&interval)->f64 
{

	if interval_a.max < interval_b.min
	{return 0.0;}
	if interval_b.max < interval_a.min
	{return 0.0;}

	//println!("interval a : {:?} interval b : {:?}", interval_a, interval_b);
	let len_a:f64 = interval_b.max - interval_a.min;
	let len_b:f64 = interval_b.min - interval_a.max;

	if len_a.abs() > len_b.abs()
	{
		return len_a;
	}
	return len_b;
	//return (Math.abs(lenA) < Math.abs(lenB)) ? lenA : lenB;
}
