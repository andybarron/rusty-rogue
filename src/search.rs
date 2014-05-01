use graph::{Graph,GraphNode};
use collections::hashmap::{HashMap,HashSet};

pub trait SearchStrategy {
	fn solve(&self, graph: &Graph, start: (int,int), end: (int,int)) -> Option<Vec<(int,int)>>;
}

pub struct AStarSearch {
	heuristic: fn(&GraphNode,&GraphNode) -> f32
}

// functionality unique to AStar
impl AStarSearch {

	// public
	pub fn new_euclidean() -> AStarSearch {
		AStarSearch {
			heuristic: AStarSearch::h_euclidean
		}
	}

	pub fn new_diagonal() -> AStarSearch {
		AStarSearch {
			heuristic: AStarSearch::h_diagonal
		}
	}

	pub fn new_dijkstra() -> AStarSearch {
		AStarSearch {
			heuristic: AStarSearch::h_zero
		}
	}

	// private
	// heuristic functions
	fn h_euclidean(a: &GraphNode, b: &GraphNode) -> f32 {
		a.distance_to(b)
	}

	fn h_diagonal(a: &GraphNode, b: &GraphNode) -> f32 {
		let dx = (a.get_x() - b.get_x()).abs() as f32;
		let dy = (a.get_y() - b.get_y()).abs() as f32;
		let sqrt2 = (2.0 as f32).sqrt();
		(dx+dy) + ( sqrt2 - 2.0 ) * if dx < dy { dx } else { dy }
	}

	fn h_zero(a: &GraphNode, b: &GraphNode) -> f32 {
		0.0
	}

	fn h(&self, current: &GraphNode, end: &GraphNode) -> f32 {
		let func = self.heuristic;
		func(current,end)
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
			// find node in open set with lowest f_score
			let mut lowest_f = None;
			let mut lowest_node = None;
			for node in open.iter() {
				let current_f = f_score.find(node).expect("No expected f score for node").clone();
				if lowest_f.is_none() || current_f < lowest_f.expect("THIS = NO") {
					lowest_f = Some(current_f);
					lowest_node = Some(node.clone());
				}
			}
			// set it to current
			let current_node = lowest_node.expect("Didn't find current node");
			// if we found the goal node, return the whole path
			if current_node == end_node {
				let path = self.build_coord_path(&came_from,&current_node);
				return Some(path);
			}
			// move current node form open set to closed set
			open.remove(&current_node);
			closed.insert(current_node);
			// for all neighbors in open set... do stuff
			for neighbor_ref in graph.get_neighbors(&current_node).expect("Couldn't find node in graph").iter() {
				if closed.contains(neighbor_ref) {
					continue
				}
				let tentative_g = g_score.find(&current_node).expect("NO g score for cur")
					+ current_node.distance_to(neighbor_ref);
				// update f and g scores for unvisisted neighbors
				if !open.contains(neighbor_ref) ||
						tentative_g < *g_score.find(neighbor_ref).expect("NO g for neighbor") {
					came_from.insert(neighbor_ref.clone(), current_node);
					g_score.insert(neighbor_ref.clone(), tentative_g);
					f_score.insert(neighbor_ref.clone(), g_score.find(neighbor_ref).expect("NO g for neighbor")
						+ self.h(neighbor_ref,&end_node));
					// insert neighbor into open set
					if !open.contains(neighbor_ref) {
						open.insert(neighbor_ref.clone());
					}
				}
			}
		}
		None // if we haven't found a solution, it's impossible :'(
	}


}