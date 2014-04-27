// implementing graph data structures and search algs
use std::num::pow;
use collections::hashmap::{HashMap,HashSet};

pub struct Graph {
	node_map: HashMap<GraphNode, HashSet<GraphNode>>,
}

/// UNDIRECTED graph implementation
// TODO remove_node methods
impl Graph {

	pub fn new() -> Graph {
		Graph {
			node_map: HashMap::new(),
		}
	}

	/* dealing with node locations/coordinates */

	pub fn add_node_at(&mut self, x: int, y: int) -> bool {
		self.node_map.insert(GraphNode::new(x,y), HashSet::new())
	}

	pub fn find_node_at(&self, x: int, y: int) -> Option<GraphNode> {
		for node_ref in self.node_map.keys() {
			if node_ref.x == x && node_ref.y == y {
				return Some(node_ref.clone());
			}
		}
		None
	}

	pub fn find_node_at_tuple(&self, coords: (int,int)) -> Option<GraphNode> {
		let (x,y) = coords;
		self.find_node_at(x,y)
	}

	pub fn connect_nodes_at(&mut self, ax: int, ay: int, bx: int, by: int) -> bool {
		match (self.find_node_at(ax,ay),self.find_node_at(bx,by)) {
			(Some(ref a),Some(ref b)) => {
				self.connect_nodes(a,b)
			}
			(_,_) => false
		}
	}

	pub fn remove_node_at(&mut self, x: int, y: int) -> bool {
		match self.find_node_at(x,y) {
			None => false,
			Some(ref node) => self.remove_node(node)
		}
	}

	/* dealing with nodes directly */

	pub fn get_neighbors(&self, node: &GraphNode) -> Option<HashSet<GraphNode>> {
		match self.node_map.find(node) {
			None => None,
			Some(set) => Some(set.clone())
		}
	}

	pub fn remove_node(&mut self, node: &GraphNode) -> bool {
		match self.node_map.pop(node) {
			None => false,
			Some(neighbors) => {
				for neighbor in neighbors.iter() {
					self.node_map.find_mut(neighbor).expect("Invalid neighbor to remove!").remove(node);
				}
				true
			}
		}
	}

	pub fn connect_nodes(&mut self, a: &GraphNode, b: &GraphNode) -> bool {
		match (self.node_map.contains_key(a),self.node_map.contains_key(b)) {
			(true,true) => {
				self.node_map.get_mut(a).insert(b.clone());
				self.node_map.get_mut(b).insert(a.clone());
				true
			}
			(_,_) => false
		}
	}

	pub fn get_node_set(&self) -> HashSet<GraphNode> {
		let mut s = HashSet::new();
		for node in self.node_map.keys() {
			s.insert(node.clone());
		}
		s
	}
}

#[deriving(Clone,Hash,TotalEq,Show)]
pub struct GraphNode {
	x: int,
	y: int,
}

impl GraphNode {
	pub fn new(x: int, y: int) -> GraphNode {
		GraphNode {
			x: x,
			y: y,
		}
	}
	pub fn get_x(&self) -> int {
		self.x
	}
	pub fn get_y(&self) -> int {
		self.y
	}
	pub fn distance_to(&self, other: &GraphNode) -> f32 {
		let mut dy_sq: int = pow(other.get_y()-self.get_y() , 2);
		let mut dx_sq: int = pow(other.get_x()-self.get_x() , 2);

		(dx_sq as f32 + dy_sq as f32).sqrt()
	}
}

impl Eq for GraphNode {
	fn eq(&self, other: &GraphNode) -> bool {
		self.x == other.x && self.y == other.y
	}
}