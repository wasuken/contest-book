use std::cmp;

fn gcd(a: i32, b: i32) -> i32 {
	if b == 0 {
		return a
	}
	gcd(b, a % b)
}

fn act2_6_1(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
	let mut mx:i32 = 0;
	let mut mn:i32 = 0;
	if p1[0].wrapping_sub(p2[0]) > p1[1].wrapping_sub(p2[1]) {
		mx = cmp::max(p1[0], p2[0]).wrapping_sub(cmp::min(p1[0], p2[0]));
		mn = cmp::max(p1[1], p2[1]).wrapping_sub(cmp::min(p1[1], p2[1]));
	}else{
		mn = cmp::max(p1[0], p2[0]).wrapping_sub(cmp::min(p1[0], p2[0]));
		mx = cmp::max(p1[1], p2[1]).wrapping_sub(cmp::min(p1[1], p2[1]));
	}
	gcd(mx, mn) - 1
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn act2_6_1_test(){
		assert_eq!(3, act2_6_1(&vec![1,11], &vec![5,3]));
	}
}
