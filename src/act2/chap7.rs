fn act2_7_1(n: u32, v1: &Vec<i32>, v2:&Vec<i32>) -> i32{
	let mut v1_c = v1.clone();
	let mut v2_c = v2.clone();
	v1_c.sort();
	v2_c.sort();
	let mut ans: i32 = 0;
	for i in 0..(v1_c.len()){
		let index = ((n as i32) - (i as i32) - 1) as usize;
		ans += v1_c[i] * v2_c[index];
	}
	return ans
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn act2_7_1_test(){
		assert_eq!(-25, act2_7_1(3, &vec![1, 3, -5], &vec![-2, 4, 1]));
		assert_eq!(6, act2_7_1(5, &vec![1, 2, 3, 4, 5], &vec![1, 0, 1, 0, 1]));
	}
}
