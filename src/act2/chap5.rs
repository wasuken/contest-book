extern crate num;
use std::collections::VecDeque;
use std::collections::HashSet;

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

#[derive(Clone)]
struct UVEdge{
	u: usize,
	v: usize,
	cost: i32
}

impl UVEdge{
	fn new(u: usize, v: usize, cost: i32) -> UVEdge {
		UVEdge{
			u: u,
			v: v,
			cost: cost
		}
	}
	fn comp(&mut self, e: UVEdge) -> bool {
		self.cost < e.cost
	}
}

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

fn kruskal(es: &mut Vec<UVEdge>, v: usize, e:usize) -> i32 {
	es.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
	let mut uf = UnionFind::new(v);
	let mut res = 0;
	for i in 0..e {
		let edge = es[i].clone();
		if !uf.same(edge.u, edge.v){
			uf.unite(edge.u, edge.v);
			res += edge.cost;
		}
	}
	return res
}

fn act2_5_3(n: usize, m: usize, r: usize, xyd: &Vec<Vec<usize>>) -> i32 {
	let mut x_list = Vec::new();
	let mut y_list = Vec::new();
	let mut d_list = Vec::new();
	let mut vs = HashSet::new();

	for x in xyd {
		x_list.push(x[0]);
		y_list.push(x[1]);
		d_list.push(x[2]);
		vs.insert(x[0]);
		vs.insert(x[1]);
	}
	let mut es = Vec::new();
	for i in 0..r {
		es.push(UVEdge::new(x_list[i], n + y_list[i], -(d_list[i] as i32)))
	}
	let result = (10000i32 * ((n + m) as i32) + kruskal(&mut es, vs.len(), x_list.len()));
	return result
}

// 実装できないです。まけました。
fn act2_5_4(m: usize, ml: usize, md: usize, abd_l: &Vec<Vec<usize>>,
			abd_d: &Vec<Vec<usize>>) -> usize{
	let mut al = Vec::new();
	let mut bl = Vec::new();
	let mut dl = Vec::new();

	let mut ad = Vec::new();
	let mut bd = Vec::new();
	let mut dd = Vec::new();

	for x in abd_l {
		al.push(x[0]);
		bl.push(x[1]);
		dl.push(x[2]);
	}
	for x in abd_d {
		ad.push(x[0]);
		bd.push(x[1]);
		dd.push(x[2]);
	}
	let mut res = 0;
	return res
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
	fn act2_5_3_test(){
		assert_eq!(71071, act2_5_3(5,5,8,&vec![vec![4,3,6831],
											   vec![1,3,4583],
											   vec![0,0,6592],
											   vec![0,1,3063],
											   vec![3,3,4975],
											   vec![1,3,2049],
											   vec![4,2,2104],
											   vec![2,2,781]]));
	}
	fn act2_5_4_test(){
		assert_eq!(27, act2_5_4(4, 2, 1, &vec![vec![1,3,10],
											   vec![2,4,20]],
								&vec![vec![2,3,3]]));
	}
}
