extern crate num;
use std::cmp;

fn rec(i: usize, j: usize, n: usize, w_list: &Vec<usize>, v_list: &Vec<usize>, dp: &mut Vec<Vec<Option<usize>>>) -> usize {
	if dp[i][j] != None {
		return dp[i][j].unwrap()
	}
	let mut res = None;
	if i == n {
		res = Some(0)
	}else if j < w_list[i] {
		res = Some(rec(i + 1, j, n, w_list, v_list, dp))
	}else{
		res = Some(cmp::max(rec(i + 1, j, n, w_list, v_list, dp),
							rec(i + 1, j - w_list[i], n, w_list, v_list, dp) + v_list[i]))
	}
	dp[i][j] = res;
	println!("calc => {}", dp[i][j].unwrap());
	return dp[i][j].unwrap()
}

fn act2_3_1(n: usize, w_list: &Vec<usize>, v_list: &Vec<usize>, w: usize) -> usize{
	let mut dp: &mut Vec<Vec<Option<usize>>> = &mut Vec::new();
	for _ in 0..1000 {
		let mut line = Vec::new();
		for _ in 0..1000{
			line.push(None)
		}
		dp.push(line);
	}
	return rec(0, w, n, w_list, v_list, dp)
}

fn act2_3_2(n: usize, m: usize, s: &str, t: &str) -> usize {
	let max_n = 1000;
	let max_m = 1000;
	let mut dp: &mut Vec<Vec<usize>> = &mut Vec::new();
	for _ in 0..max_n {
		let mut line = Vec::new();
		for _ in 0..max_m{
			line.push(0)
		}
		dp.push(line);
	}
	let s_chrs:Vec<char> = s.chars().collect();
	let t_chrs:Vec<char> = t.chars().collect();
	for i in 0..n{
		for j in 0..m{
			if s_chrs[i] == t_chrs[j] {
				dp[i+1][j+1] = dp[i][j] + 1;
			}else{
				dp[i+1][j+1] = cmp::max(dp[i][j + 1], dp[i+1][j]);
			}
		}
	}
	return dp[n][m]
}

fn act2_3_3(n: usize, w_list: &Vec<usize>, v_list: &Vec<usize>, w: usize) -> usize{
	let mut dp: &mut Vec<usize> = &mut Vec::new();
	for x in 0..100 {
		dp.push(0);
	}
	for i in 0..n {
		for j in w_list[i]..w+1 {
			dp[j] = cmp::max(dp[j], dp[j - w_list[i]] + v_list[i]);
		}
	}
	return dp[w]
}

fn act2_3_4(n: usize, w_list: &Vec<usize>, v_list: &Vec<usize>, w: usize) -> usize {
	let max_n = 100;
	let max_m = 100;
	let mut dp: &mut Vec<Vec<usize>> = &mut Vec::new();
	for _ in 0..max_n {
		let mut line: Vec<usize> = Vec::new();
		for _ in 0..(max_m * max_n){
			line.push(usize::max_value())
		}
		dp.push(line);
	}
	dp[0][0] = 0;
	for i in 0..n {
		for j in 0..(max_n * max_m) {
			if j < v_list[i] {
				dp[i + 1][j] = dp[i][j];
			}else{
				let right = if dp[i][j - v_list[i]] == usize::max_value() {
					dp[i][j - v_list[i]]
				}else{
					dp[i][j - v_list[i]] + w_list[i]
				};
				dp[i + 1][j] = cmp::min(dp[i][j], right);
			}
		}
	}
	let mut res = 0;
	for i in 0..(max_n * max_m) {
		if dp[n][i] <= w{
			res = i;
		}
	}
	return res
}

fn act2_3_5(n: usize, a: &Vec<usize>, m: &Vec<usize>, k: usize) -> String {
	let max_n = 1000;
	let max_m = 1000;
	let mut dp: &mut Vec<i32> = &mut Vec::new();
	for _ in 0..max_n {
		dp.push(-1);
	}
	dp[0] = 0;
	for i in 0..n {
		for j in 0..k+1 {
			if dp[j] >= 0 {
				dp[j] = m[i] as i32;
			}else if j < a[i] || dp[j - a[i]] <= 0 {
				dp[j] = -1;
			}else{
				dp[j] = dp[j - a[i]] - 1;
			}
		}
	}
	for i in 0..max_n {
		if dp[i] >= 0 {
			println!("i:{}, value:{}", i, dp[i]);
		}
	}
	if dp[k] >= 0 {
		return "Yes".to_string()
	}else {
		return "No".to_string()
	}
}

fn act2_3_6(n: usize, a:&Vec<usize>) -> usize {
	let mut res = 0;
	let mut dp: &mut Vec<usize> = &mut Vec::new();
	let max_n = 1000;
	for _ in 0..max_n {
		dp.push(1);
	}
	for i in 0..n {
		for j in 0..i {
			if a[i] < a[j] {
				dp[i] = cmp::max(dp[i], dp[j] + 1);
			}
		}
		res = cmp::max(res, dp[i]);
	}
	return res
}

fn act2_3_7(n: usize, m:usize, bigM:usize) -> usize {
	let max_n = 100;
	let max_m = 100;
	let mut dp: &mut Vec<Vec<usize>> = &mut Vec::new();
	for _ in 0..max_n {
		let mut line: Vec<usize> = Vec::new();
		for _ in 0..(max_m * max_n){
			line.push(0)
		}
		dp.push(line);
	}
	dp[0][0] = 1;

	for i in 1..m+1 {
		for j in 0..n+1 {
			if (j as i32 - i as i32) >= 0 {
				dp[i][j] = (dp[i - 1][j] + dp[i][j - i]) % bigM;
			}else{
				dp[i][j] = dp[i - 1][j];
			}
		}
	}
	return dp[m][n]
}

fn act2_3_8(n: usize, m: usize, a: &Vec<usize>, bigM: usize) -> usize {
	let max_n = 100;
	let max_m = 100;
	let mut dp: &mut Vec<Vec<usize>> = &mut Vec::new();
	for i in 0..max_n {
		let mut line: Vec<usize> = Vec::new();
		for j in 0..(max_m * max_n){
			if j == 0 {
				line.push(1)
			}else{
				line.push(0)
			}
		}
		dp.push(line);
	}
	for i in 0..n{
		for j in 1..m+1{
			let x = ((j as i32) - (1 as i32) - (a[i] as i32)) as i32;
			if x >= 0 {
				dp[i + 1][j] = (dp[i + 1][j - 1] + dp[i][j] - dp[i][j - 1 - a[i]] + bigM) % bigM;
			}else{
				dp[i + 1][j] = (dp[i + 1][j - 1] + dp[i][j]) % bigM;
			}
		}
	}
	return dp[n][m]
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn act2_3_1_test(){
		assert_eq!(7, act2_3_1(4, &vec![2,1,3,2], &vec![3,2,4,2], 5));
	}
	#[test]
	fn act2_3_2_test(){
		assert_eq!(3, act2_3_2(4, 4, "abcd", "becd"));
	}
	#[test]
	fn act2_3_3_test(){
		assert_eq!(10, act2_3_3(3, &vec![3,4,2], &vec![4,5,3], 7));
	}
	#[test]
	fn act2_3_4_test(){
		assert_eq!(7, act2_3_4(4, &vec![2,1,3,2], &vec![3,2,4,2], 5));
	}
	#[test]
	fn act2_3_5_test(){
		assert_eq!("Yes", act2_3_5(3, &vec![3,5,8], &vec![3,2,2], 17));
	}
	#[test]
	fn act2_3_6_test(){
		assert_eq!(3, act2_3_6(5, &vec![4,2,3,1,5]));
	}
	#[test]
	fn act2_3_7_test(){
		assert_eq!(4, act2_3_7(4, 3, 1000));
	}
	#[test]
	fn act2_3_8_test(){
		assert_eq!(6, act2_3_8(3, 3, &vec![1,2,3], 10000));
	}
}
