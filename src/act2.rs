extern crate num;
use queue::Queue;
use std::cmp;
use std::mem;
use std::collections::VecDeque;

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

fn act2_4_1(n: usize, l: usize, p:usize, a: &Vec<usize>, b: &Vec<usize>) -> Option<usize> {
	let mut aCp = a.clone();
	let mut bCp = b.clone();
	aCp.push(l);
	aCp.push(0);
	let mut n2 = n;

	let mut que = VecDeque::new();
	let mut ans = 0;
	let mut pos = 0;
	let mut tank = p;

	for i in 0..n2{
		let mut d = aCp[i] - pos;
		let mut tankd: i32 = (tank as i32) - (d as i32);
		while tankd < 0 {
			if que.is_empty() {
				println!("-1");
				return None
			}
			tank += que.pop_back().unwrap();
			ans+=1;
			tankd = (tank as i32) - (d as i32);
		}
		tank -= d;
		pos = aCp[i];
		que.push_back(bCp[i]);
	}
	return Some(ans)
}

fn vecDequeSort(vd: &VecDeque<usize>) -> VecDeque<usize> {
	let mut que: VecDeque<usize> = VecDeque::new();
	let mut v = Vec::new();
	for (_, &item) in vd.as_slices().0.iter().enumerate() {
		v.push(item);
	}
	v.sort();
	for x in v {
		que.push_back(x);
	}
	return que
}

fn act2_4_2(n: usize, l: &Vec<usize>) -> i64 {
	let mut ans: i64 = 0;
	let mut que: VecDeque<usize> = VecDeque::new();
	for i in 0..n {
		que.push_back(l[i])
	}
	while que.len() > 1 {
		let mut l1 = que.pop_back().unwrap();
		let mut l2 = que.pop_back().unwrap();
		ans += (l1 + l2) as i64;
		que.push_back(l1 + l2);
		que = vecDequeSort(&que);
	}
	return ans
}

// 引用元
// http://sntea.hatenablog.com/entry/2017/06/07/091246
// 上記を少しだけ修正した。
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut vec = vec![0;n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind {
            par : vec,
            rank : vec![0;n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        }else{
            let res = self.find(self.par[x]);
            self.par[x] = res;
            res
        }
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn unite(&mut self, a: usize, b: usize){
        let apar = self.find(a);
        let bpar = self.find(b);
		if apar == bpar{
			return
		}
        if self.rank[apar] < self.rank[bpar] {
            self.par[apar] = bpar;
        }else{
            self.par[bpar] = apar;
            if self.rank[apar] == self.rank[bpar] {
                self.rank[apar] += 1;
            }
        }
    }
	fn debug(&mut self){
		println!("par:{:?}, rank:{:?}", self.par, self.rank);
	}
}

fn act2_4_3(n: usize, k: usize, inputList: &Vec<Vec<usize>>) -> usize {
	let mut xList = Vec::new();
	let mut yList = Vec::new();
	let mut tList = Vec::new();
	for x in inputList {
		tList.push(x[0]);
		xList.push(x[1]);
		yList.push(x[2]);
	}
	println!("xList:{:?}", xList);
	println!("yList:{:?}", yList);
	println!("tList:{:?}", tList);
	let mut uf = UnionFind::new(n * 3);

	let mut ans = 0;
	for i in 0..k {
		println!("{}つ目", i + 1);
		let t = tList[i];
		let x = xList[i] - 1;
		let y = yList[i] - 1;

		if x < 0 || n <= x || y < 0 || n <= y {
			ans += 1;
			continue
		}
		if t == 1 {
			if uf.same(x, y + n) || uf.same(x, y + 2 * n) {
				ans += 1;
			}else{
				uf.unite(x, y);
				uf.unite(x + n, y + n);
				uf.unite(x + n * 2, y + n * 2);
			}
		}else{
			if uf.same(x, y) || uf.same(x, y + 2 * n) {
				ans += 1;
			}else{
				uf.unite(x, y + n);
				uf.unite(x + n, y + 2 * n);
				uf.unite(x + 2 * n, y);
			}
		}
	}

	return ans
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
	#[test]
	fn act2_4_1_test(){
		assert_eq!(Some(2), act2_4_1(4, 25, 10, &vec![10, 14, 20, 21], &vec![10, 5, 2, 4]));
	}
	#[test]
	fn act2_4_2_test(){
		assert_eq!(34, act2_4_2(3, &vec![8, 5, 8]));
	}
	#[test]
	fn act2_4_3_test(){
		let inputList = vec![vec![1,101,1],
							 vec![2,1,2],
							 vec![2,2,3],
							 vec![2,3,3],
							 vec![1,1,3],
							 vec![2,3,1],
							 vec![1,5,5]];
		assert_eq!(3, act2_4_3(100, 7, &inputList));
	}
}
