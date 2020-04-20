use queue::Queue;

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
	for x_ in -1..2 {
		for y_ in -1..2 {
			let nx = x + x_;
			let ny = y + y_;

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

const INF: i32 = 100000000;

fn act2_1_3_bfs(sx: i32, sy: i32, gx: i32, gy: i32, maze: &mut Vec<Vec<String>>, d: &mut Vec<Vec<i32>>) -> i32 {
	let dx: Vec<i32> = [1, 0, -1, 0].to_vec();
	let dy: Vec<i32> = [0, 1, 0, -1].to_vec();
	let mut q = Queue::with_capacity(1000000);
	for i in 0..maze.len() {
		for j in 0..maze[0].len() {
			d[i as usize][j as usize] = INF;
		}
	}
	q.queue((sx, sy)).unwrap();
	d[sx as usize][sy as usize] = 0;
	while !q.is_empty() {
		let p = match q.dequeue() {
			Some(pt) => pt,
			_ => (0, 0)
		};
		if p.0 == gx && p.1 == gy {
			break;
		}
		for i in 0..4{
			let nx = p.0 + dx[i];
			let ny = p.1 + dy[i];

			if 0 <= nx && (nx as usize) < maze.len() && 0 <= ny && (ny as usize) < maze[0].len()
				&& maze[nx as usize][ny as usize] != "#" && d[nx as usize][ny as usize] == INF {
					q.queue((nx, ny)).unwrap();
					d[nx as usize][ny as usize] = d[p.0 as usize][p.1 as usize] + 1;
				}
		}
	}
	return d[gx as usize][gy as usize]
}

fn act2_1_3(n: i32, m: i32, maze: &mut Vec<Vec<String>>) -> i32 {
	let mut sx: i32 = 0;
	let mut sy: i32 = 0;
	let mut gx: i32 = 0;
	let mut gy: i32 = 0;
	let d: &mut Vec<Vec<i32>> = &mut Vec::new();

	for x in 0..maze.len() {
		d.push(Vec::new());
		for y in 0..maze[x].len() {
			d[x].push(INF);
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
	use super::*;
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
							 vec!["#","#",".","#","#",".","#","#","#","#"],
							 vec![".",".",".",".","#",".",".",".",".","#"],
							 vec![".","#","#","#","#","#","#","#",".","#"],
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
		assert_eq!(act2_1_3(10, 10, &mut sInput), 22);
	}
}
