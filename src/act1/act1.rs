use std::cmp;
fn act1_6_1(n: i32, a: Vec<u32>) -> u32 {
	for x in &a {
		for y in &a {
			for z in &a {
				if (x.pow(2) + y.pow(2)) == z.pow(2) {
					return x + y + z
				}
			}
		}
	}
	return 0
}

fn act1_6_2(l: i32, n: i32, x: Vec<i32>) -> (i32, i32) {
	let mut min = 0;
	for i in &x {
		min = cmp::max(min, cmp::min(*i, l - i))
	}
	let mut max = 0;
	for i in &x {
		max = cmp::max(max, cmp::max(*i, l - i))
	}
	return (min, max)
}

fn dfs(i: i32, sum: i32, n:i32, a: &Vec<i32>, k: i32) -> bool {
	if i == n {
		return sum == k
	}
	if dfs(i + 1, sum, n,  a, k) {
		return true
	}
	if dfs(i + 1, sum + a[i as usize], n, a, k) {
		return true
	}
	return false;
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn act1_6_1_test() {
        assert_eq!(act1_6_1(5, vec![2, 3, 4, 5, 10]), 12);
		assert_eq!(act1_6_1(4, vec![4, 5, 10, 20]), 0);
    }
	#[test]
	fn act1_6_2_test(){
		assert_eq!(act1_6_2(10, 3, vec![2, 6, 7]), (4, 8));
	}
}
