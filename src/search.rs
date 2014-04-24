use graph::{Graph,GraphNode};
use collections::hashmap::{HashMap,HashSet};
use std::num::pow;

pub trait SearchStrategy {
	fn solve(&self, graph: &Graph, start: (int,int), end: (int,int)) -> Option<Vec<(int,int)>>;
}

pub struct AStarSearch; // TODO heuristic as field

impl SearchStrategy for AStarSearch {
	fn solve(&self, graph: &Graph, start: (int,int), end: (int,int)) -> Option<Vec<(int,int)>> {

		let start_node = graph.find_node_at_tuple(start).expect("ERROR: Could not find start node :-(");
		let end_node = graph.find_node_at_tuple(end).expect("ERROR: Could not find end node :-(");

		//set of nodes evaluated (initially empty)
		let mut visited = HashSet::new();

		//set of nodes to be evaluated (ie the entire dungeon)
		let mut unvisited = graph.get_node_set();

		//list of navigated nodes (initially empty)
		let mut path = vec!(start);
		let mut path_cost = 0.0;
		let mut current = start;

		//while loop until unvisited is empty
		//ie while unvisited not empty

		while unvisited.len() > 0 {
			// replace this with actual jank
			// 1. look at neighbors of node "current"
			// 2. if all are visited, backtrack one step (and update path cost)
			// 3. otherwise, pick the one with the lowest f+g (heuristic + path length)
			// 4. set current to it and update path and path cost
			// 5. ??????????
			// 6. PROFIT!
		}



		None // TODO so it compiles
	}

	fn heuristic_cost_estimate(&self, start: &GraphNode, end: &GraphNode) -> f32 {
		//sqrt( (endY-startY)^2 + (endX - startX)^2 )

		let mut dy_sq: f32 = pow(end.y-start.y , 2);
		let mut dy_sq: f32 = pow(end.x-start.x , 2);

		(dx_sq + dy_sq).sqrt()

	}

// D = 1, D2 = sqrt(2)
// function heuristic(node) =
//    dx = abs(node.x - goal.x)
//    dy = abs(node.y - goal.y)
//    return D * (dx + dy) + (D2 - 2 * D) * min(dx, dy)


}