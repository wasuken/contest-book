extern crate num;
use std::collections::VecDeque;

fn act2_5_1_dfs(v: usize, c: i32, color: &mut Vec<i32>, graph_list: &Vec<Vec<usize>>) -> bool {
	color[v] = c;
	for i in 0..graph_list[v].len() {
		if color[graph_list[v][i]] == c {
			return false
		}
		if color[graph_list[v][i]] == 0 && !act2_5_1_dfs(graph_list[v][i], -c, color, graph_list) {
			return false
		}
	}
	return true
}

fn act2_5_1(v:usize, graph_list: &Vec<Vec<usize>>) -> String {
	let max_v = 1000;
	let mut color = Vec::new();
	for _ in 0..max_v {
		color.push(0);
	}
	for i in 0..v{
		println!("line {}", i);
		if color[i] == 0 {
			if !act2_5_1_dfs(i, 1, &mut color, graph_list){
				return "No".to_string()
			}
		}
	}
	return "Yes".to_string()
}

#[derive(Clone)]
struct Edge{
	to: usize,
	cost: usize
}

impl Edge{
	fn new(to: usize, cost: usize) -> Edge {
		Edge{
			to: to,
			cost: cost
		}
	}
	fn comp(&mut self, e: Edge) -> bool {
		self.cost < e.cost
	}
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

fn act2_5_2(n: usize, r: usize, g_list: &Vec<Vec<Edge>>) -> usize {
	let mut que = VecDeque::new();

	let mut dist: Vec<Option<usize>> = Vec::new();
	let mut dist2: Vec<Option<usize>> = Vec::new();

	for i in 0..n {
		dist.push(None);
		dist2.push(None);
	}
	dist[0] = Some(0);
	que.push_back((0, 0));

	while !que.is_empty() {
		let p = que.pop_back().unwrap();
		let v = p.1;
		let d = p.0;

		for i in 0..g_list[v].len() {
			let e = g_list[v][i].clone();
			let mut d2 = d + e.cost;
			if dist[e.to].unwrap() > d2 {
				let temp = dist[e.to].unwrap();
				dist[e.to] = Some(d2);
				d2 = temp;
				que.push_back((dist[e.to].unwrap(), e.to));
			}
			if dist2[e.to].unwrap() > d2 && dist[e.to].unwrap() < d2 {
				dist2[e.to] = Some(d2);
				que.push_back((dist2[e.to].unwrap(), e.to));
			}

		}
	}

	dist2[n - 2].unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn act2_5_1_test(){
		assert_eq!("No".to_string(), act2_5_1(3, &vec![vec![1,2],
													   vec![0,2],
													   vec![0,1]]));
		assert_eq!("Yes".to_string(), act2_5_1(4, &vec![vec![1,3],
														vec![0,2],
														vec![1,3],
														vec![0,2]]));
	}
	fn act2_5_2_test(){
		assert_eq!(450, act2_5_2(4, 4, &vec![vec![Edge::new(2,100)],
											 vec![Edge::new(1,100),
												  Edge::new(4,200),
												  Edge::new(3,250)],
											 vec![Edge::new(2,250),
												  Edge::new(4,100)],
											 vec![Edge::new(2,200),
												  Edge::new(4,100)]]));
	}
}
