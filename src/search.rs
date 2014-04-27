use graph::{Graph,GraphNode};
use collections::hashmap::{HashMap,HashSet};
use std::num::pow;

pub trait SearchStrategy {
	fn solve(&self, graph: &Graph, start: (int,int), end: (int,int)) -> Option<Vec<(int,int)>>;
}

pub struct AStarSearch; // TODO heuristic as field

/// functionality unique to AStar
impl AStarSearch {
	fn h(&self, current: &GraphNode, end: &GraphNode) -> f32 {
		current.distance_to(end)
	}
	fn build_node_path(&self, came_from: &HashMap<GraphNode,GraphNode>, current_node: &GraphNode) -> Vec<GraphNode> {
		if came_from.contains_key(current_node) {
			let mut path = self.build_node_path(came_from, came_from.find(current_node).expect("SHOULD have been found"));
			path.push(current_node.clone());
			path
		} else {
			vec!(current_node.clone())
		}
	}
	fn build_coord_path(&self, came_from: &HashMap<GraphNode,GraphNode>, current_node: &GraphNode) -> Vec<(int,int)> {
		let vec = self.build_node_path(came_from, current_node);
		let mut ret = Vec::new();
		for node in vec.iter() {
			ret.push( (node.get_x(),node.get_y()) );
		}
		ret
	}
}

impl SearchStrategy for AStarSearch {

	fn solve(&self, graph: &Graph, start: (int,int), end: (int,int)) -> Option<Vec<(int,int)>> {

		let start_node = graph.find_node_at_tuple(start).expect("ERROR: Could not find start node :-(");
		let end_node = graph.find_node_at_tuple(end).expect("ERROR: Could not find end node :-(");

		// set of nodes evaluated (initially empty)
		let mut closed = HashSet::new();

		// tentative "frontier" set
		let mut open = HashSet::new();
		open.insert(start_node);

		// map of navigated nodes - used to reconstruct the path!
		let mut came_from = HashMap::new();

		// map of f and g scores
		let mut g_score = HashMap::new(); // cost from start along best known path
		let mut f_score = HashMap::new(); // estimated total cost from start to goal through node

		// initialize f and g score maps
		g_score.insert(start_node, 0f32);
		f_score.insert(start_node, g_score.find(&start_node).expect("Start not in g_score") + self.h(&start_node,&end_node));

		//while loop until unvisited is empty
		//ie while unvisited not empty

		while !open.is_empty() {
			let mut lowest_f = None;
			let mut lowest_node = None;
			for node in open.iter() {
				let current_f = f_score.find(node).expect("No expected f score for node").clone();
				if lowest_f.is_none() || current_f < lowest_f.expect("THIS = NO") {
					lowest_f = Some(current_f);
					lowest_node = Some(node.clone());
				}
			}
			let current_node = lowest_node.expect("Didn't find current node");
			// println!("Current: {}", current_node);
			if current_node == end_node {
				let path = self.build_coord_path(&came_from,&current_node);
				return Some(path);
			}
			open.remove(&current_node);
			closed.insert(current_node);
			for neighbor_ref in graph.get_neighbors(&current_node).expect("Couldn't find node in graph").iter() {
				if closed.contains(neighbor_ref) {
					continue
				}
				let tentative_g = g_score.find(&current_node).expect("NO g score for cur")
					+ current_node.distance_to(neighbor_ref);
				if !open.contains(neighbor_ref) ||
						tentative_g < *g_score.find(neighbor_ref).expect("NO g for neighbor") {
					came_from.insert(neighbor_ref.clone(), current_node);
					g_score.insert(neighbor_ref.clone(), tentative_g);
					f_score.insert(neighbor_ref.clone(), g_score.find(neighbor_ref).expect("NO g for neighbor")
						+ self.h(neighbor_ref,&end_node));
					if !open.contains(neighbor_ref) {
						open.insert(neighbor_ref.clone());
					}
				}
			}
		}



		None // TODO so it compiles
	}

// D = 1, D2 = sqrt(2)
// function heuristic(node) =
//    dx = abs(node.x - goal.x)
//    dy = abs(node.y - goal.y)
//    return D * (dx + dy) + (D2 - 2 * D) * min(dx, dy)


}