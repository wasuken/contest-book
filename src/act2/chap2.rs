extern crate num;
use queue::Queue;
use std::cmp;
use std::mem;
use std::collections::VecDeque;

fn act2_2_1(yen_cnt_list: &Vec<i32>, target: i32) -> i32 {
	let yen_int = [1, 5, 10, 50, 100, 500];
	let mut ans = 0;
	let mut a = target;
	for i in num::range_step(5i32, 0i32, -1) {
		println!("{}", i);
		let t = cmp::min(a / yen_int[i as usize], yen_cnt_list[i as usize]);
		a -= t * yen_int[i as usize];
		ans += t;
	}
	return ans
}

fn act2_2_2(n: usize, s: &Vec<usize>, t: &Vec<usize>) -> usize {
	let mut itv: Vec<(usize, usize)> = Vec::new();
	for i in 0..n {
		itv.push((t[i], s[i]));
	}
	itv.sort();
	let mut ans = 0;
	let mut t = 0;
	for i in 0..n {
		if t < itv[i].1 {
			ans+=1;
			t = itv[i].0;
		}
	}

	return ans
}

fn act2_2_3(n: usize, s: &str) -> String {
	let mut a = 0;
	let mut b = n - 1;
	let s_chrs: Vec<char> = s.chars().collect();
	let mut result: String = "".to_string();
	while a <= b {
		let mut left = false;
		let mut i = 0;
		while (a + i) <= b {
			let a_i = s_chrs[a + i];
			let b_i = s_chrs[b - i];
			if a_i < b_i {
				left = true;
				break;
			}else if a_i > b_i {
				left = false;
				break;
			}
			i += 1;
		}
		if left {
			result = format!("{}{}", result, s_chrs[a]);
			a+=1;
		}else{
			result = format!("{}{}", result, s_chrs[b]);
			b-=1;
		}
	}
	return result
}

fn act2_2_4(n: usize, r: usize, x: &Vec<usize>) -> usize {
	let mut x_ = x.clone();
	x_.sort();

	let mut i = 0;
	let mut ans = 0;

	while i < n {
		let s = x[i];
		i += 1;
		while i < n && x_[i] <= s + r {
			i += 1;
		}
		let p = x_[i - 1];
		while i < n && x_[i] <= p + r {
			i += 1;
		}
		ans += 1;
	}
	return ans
}

fn act2_2_5(n: usize, l: &Vec<usize>) -> usize {
	let mut ans: usize = 0;
	let mut n_ = n;
	let mut l_ = l.clone();
	while n_ > 1 {
		let mut mii1: usize = 0;
		let mut mii2: usize = 1;
		if l_[mii1] > l_[mii2] {
			mem::swap(&mut mii1, &mut mii2);
		}
		for i in 2..n_ {
			if l_[i] < l_[mii1] {
				mii2 = mii1;
				mii1 = i;
			}else if l_[i] < l_[mii2] {
				mii2 = i;
			}
		}
		let t: usize = l_[mii1] + l_[mii2];
		ans += t;

		if mii1 == n - 1 {
			mem::swap(&mut mii1, &mut mii2);
		}
		l_[mii1] = t;
		l_[mii2] = l_[n_ - 1];
		n_-=1;
	}
	return ans
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn act2_2_1_test(){
		assert_eq!(6, act2_2_1(&vec![3,2,1,3,0,2], 620));
	}
	#[test]
	fn act2_2_2_test(){
		assert_eq!(3, act2_2_2(5, &vec![1,2,4,6,8], &vec!(3,5,7,9,10)));
	}
	#[test]
	fn act2_2_3_test(){
		assert_eq!("ABCBCD", act2_2_3(6, "ACDBCB"));
	}
	#[test]
	fn act2_2_4_test(){
		assert_eq!(3, act2_2_4(6, 10, &vec![1, 7, 15, 20, 30, 50]));
	}
	#[test]
	fn act2_2_5_test(){
		assert_eq!(34, act2_2_5(3, &vec![8, 5, 8]));
	}
}
