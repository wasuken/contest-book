use std::cmp;
use queue::Queue;

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

fn act2_1_1(n: i32, a: &Vec<i32>, k: i32) -> String {
	if dfs(0, 0, n, a, k) {
		return "Yes".to_owned()
	}else{
		return "No".to_owned()
	}
}

fn act2_1_2_dfs(x: i32, y: i32, n: i32, m: i32, field: &mut Vec<Vec<String>>) {
	field[x as usize][y as usize] = ".".to_owned();
	for dx in -1..2 {
		for dy in -1..2 {
			let nx = x + dx;
			let ny = y + dy;

			if 0 <= nx && nx < n && 0 <= ny && ny < m && field[nx as usize][ny as usize] == "W" {
				act2_1_2_dfs(nx, ny, n, m, field);
			}
		}
	}
}

fn act2_1_2(n: i32, m: i32, field: &mut Vec<Vec<String>>) -> i32 {
	let mut res = 0;
	for i in 0..n {
		for j in 0..m {
			if field[i as usize][j as usize] == "W" {
				act2_1_2_dfs(i, j, n, m, field);
				res+=1;
			}
		}
	}
	return res
}

fn act2_1_3_bfs(sx: i32, sy: i32, gx: i32, gy: i32, maze: &mut Vec<Vec<String>>, d: Vec<Vec<i32>>) -> i32 {
	let mut q = Queue::with_capacity(1000000);
	for i in 0..maze.len() {
		for j in 0..maze[0].len() {

		}
	}
	q.queue((sx, sy)).unwrap();
	d[sx as usize][sy as usize] = 0;
	while !q.is_empty() {
		let ptr = match q.dequeue() {
			Some(pt) => pt,
			_ => (0, 0)
		};
		if ptr.0 == gx && ptr.1 == gy {
			break;
		}
	}
}

fn act2_1_3(n: i32, m: i32, maze: &mut Vec<Vec<String>>) -> i32 {
	let mut sx: i32 = 0;
	let mut sy: i32 = 0;
	let mut gx: i32 = 0;
	let mut gy: i32 = 0;
	let mut d = Vec::new();

	for x in 0..maze.len() {
		for y in 0..maze[x].len() {
			if maze[x as usize][y as usize] == "S" {
				sx = x as i32;
				sy = y as i32;
			}
			if maze[x as usize][y as usize] == "G" {
				gx = x as i32;
				gy = y as i32;
			}
		}
	}

	return act2_1_3_bfs(sx, sy, gx, gy, maze, d)
}

#[cfg(test)]
mod tests {
	use crate::*;
    #[test]
    fn act1_6_1_test() {
        assert_eq!(act1_6_1(5, vec![2, 3, 4, 5, 10]), 12);
		assert_eq!(act1_6_1(4, vec![4, 5, 10, 20]), 0);
    }
	#[test]
	fn act1_6_2_test(){
		assert_eq!(act1_6_2(10, 3, vec![2, 6, 7]), (4, 8));
	}
	#[test]
	fn act2_1_1_test(){
		assert_eq!(act2_1_1(4, &vec![1, 2, 4, 7], 13), "Yes");
		assert_eq!(act2_1_1(4, &vec![1, 2, 4, 7], 15), "No");
	}
	#[test]
	fn act2_1_2_test(){
		let mut input = vec![vec!["W",".",".",".",".",".",".",".",".","W","W","."],
							 vec![".","W","W","W",".",".",".",".",".","W","W","W"],
							 vec![".",".",".",".","W","W",".",".",".","W","W","."],
							 vec![".",".",".",".",".",".",".",".",".","W","W","."],
							 vec![".",".",".",".",".",".",".",".",".","W",".","."],
							 vec![".",".","W",".",".",".",".",".",".","W",".","."],
							 vec![".","W",".","W",".",".",".",".",".","W","W","."],
							 vec!["W",".","W",".","W",".",".",".",".",".","W","."],
							 vec![".","W",".","W",".",".",".",".",".",".","W","."],
							 vec![".",".","W",".",".",".",".",".",".",".","W","."]
		];
		let mut sInput = Vec::new();
		for i in input {
			let mut temp = Vec::new();
			for j in i {
				temp.push(j.to_owned())
			}
			sInput.push(temp)
		}
		assert_eq!(act2_1_2(10, 12, &mut sInput), 3);
	}
	#[test]
	fn act2_1_3_test(){
		let mut input = vec![vec!["#","S","#","#","#","#","#","#",".","#"],
							 vec![".",".",".",".",".",".","#",".",".","#"],
							 vec![".","#",".","#","#",".","#","#",".","#"],
							 vec![".","#",".",".",".",".",".",".",".","."],
							 vec!["#","#",".","#","#",".",".",".",".","."],
							 vec![".",".",".",".","#",".",".",".",".","."],
							 vec![".",".",".",".","#",".",".",".",".","."],
							 vec![".",".",".",".","#",".",".",".",".","."],
							 vec![".","#","#","#","#",".","#","#","#","."],
							 vec![".",".",".",".","#",".",".",".","G","#"]
		];
		let mut sInput = Vec::new();
		for i in input {
			let mut temp = Vec::new();
			for j in i {
				temp.push(j.to_owned())
			}
			sInput.push(temp)
		}
		assert_eq!(act2_1_2(10, 10, &mut sInput), 22);
	}
}
