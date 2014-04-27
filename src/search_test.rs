extern crate collections;
extern crate rand;

use graph::Graph;
use search::{SearchStrategy,AStarSearch};
use collections::hashmap::HashSet;
use rand::{Rng,task_rng};

mod graph;
mod search;

fn print_graph(graph: &Graph, w: int, h: int, path: Option<Vec<(int,int)>>) {
	for y in range(0,w) {
		for x in range(0,h) {
			let ch = match graph.find_node_at(x,y) {
				None => ' ',
				Some(node) => match path {
					None => '-',
					Some(ref vec) => match vec.contains( &(node.get_x(),node.get_y()) ) {
						false => '-',
						true => 'X'
					}
				}
			};
			print!("{}",ch);
		}
		print!("\n");
	}
}

fn main() {

	let mut graph = Graph::new();
	let w = 10;
	let h = 10;
	let obstacle_count = 25;
	let start = (0,0);
	let end = (w-1,h-1);

	// list of obstacles
	let mut obstacles = HashSet::new();
	while obstacles.len() < obstacle_count {
		let ob = (
			task_rng().gen_range(0,w),
			task_rng().gen_range(0,h)
		);
		if ob != start && ob != end {
			obstacles.insert(ob);
		}
	}


	// add nodes
	for y in range(0,h) {
		for x in range(0,w) {
			let coords = (x,y);
			match obstacles.contains(&coords) {
				true => continue,
				false => {
					graph.add_node_at(x,y);
				}
			}
		}
	}

	// connect nodes
	for y in range (0,h-1) {
		for x in range (0,w-1) {
			match graph.find_node_at(x,y) {
				Some(node) => {
					// right
					match graph.find_node_at(x+1,y) {
						Some(neighbor) => { graph.connect_nodes(&node,&neighbor); }
						None => {}
					}
					// down-right
					match graph.find_node_at(x+1,y+1) {
						Some(neighbor) => { graph.connect_nodes(&node,&neighbor); }
						None => {}
					}
					// down
					match graph.find_node_at(x,y+1) {
						Some(neighbor) => { graph.connect_nodes(&node,&neighbor); }
						None => {}
					}
				}
				None => {}
			}
		}
	}

	print_graph(&graph,w,h,None);

	let search = AStarSearch;
	match search.solve(&graph,(0,0),(9,9)) {
		Some(soln) => {
			println!("Solution: {}",soln);
			print_graph(&graph,w,h,Some(soln));
		}
		None => println!("No solution found :(")
	}
}