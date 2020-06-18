extern crate num;
use std::collections::VecDeque;

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
