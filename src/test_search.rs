extern crate rand;

use graph::Graph;
use search::{SearchStrategy,AStarSearch};
use std::collections::HashSet;
use rand::{Rng,thread_rng};

use graph;
use search;

fn print_graph(graph: &Graph, w: isize, h: isize, path: Option<Vec<(isize,isize)>>) {
	for y in (0)..(h) {
		for x in (0)..(w) {
			let ch = match graph.find_node_at(x,y) {
				None => '.',
				Some(node) => match path {
					None => ' ',
					Some(ref vec) => match vec.contains( &(node.get_x(),node.get_y()) ) {
						false => ' ',
						true => 'X'
					}
				}
			};
			print!("{}",ch);
		}
		print!("\n");
	}
}

pub fn main() {

	let mut graph = Graph::new();
	let w = 80;
	let h = 40;
	let obstacle_count = 80*60/10;
	let start = (0,0);
	let end = (w-1,h-1);

	// list of obstacles
	let mut obstacles = HashSet::new();
	while obstacles.len() < obstacle_count {
		let ob = (
			thread_rng().gen_range(0,w),
			thread_rng().gen_range(0,h)
		);
		if ob != start && ob != end {
			obstacles.insert(ob);
		}
	}


	// add nodes
	for y in (0)..(h) {
		for x in (0)..(w) {
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
	for y in (0)..(h-1) {
		for x in (0)..(w-1) {
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

	let search = AStarSearch::new_diagonal();
	match search.solve(&graph,start,end) {
		Some(soln) => {
			// println!("Solution: {:?}",soln);
			println!("\nSolution:");
			print_graph(&graph,w,h,Some(soln));
		}
		None => println!("No solution found :(")
	}
}