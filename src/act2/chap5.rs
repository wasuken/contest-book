extern crate num;

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
}
