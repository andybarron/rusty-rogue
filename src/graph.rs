// implementing graph data structures and search algs
use collections::hashmap::{HashMap,HashSet};

pub struct Graph {
	node_map: HashMap<GraphNode, HashSet<GraphNode>>,
}

// TODO remove_node methods
impl Graph {
	pub fn new() -> Graph {
		Graph {
			node_map: HashMap::new(),
		}
	}
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

#[deriving(Clone,Hash,Eq,TotalEq)]
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
}