// implementing graph data structures and search algs
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Graph {
    node_map: HashMap<GraphNode, HashSet<GraphNode>>,
}

// UNDIRECTED graph implementation
// TODO return references to nodes rather than clones...
impl Graph {
    pub fn new() -> Graph {
        Graph {
            node_map: HashMap::new(),
        }
    }

    /* dealing with node locations/coordinates */

    pub fn add_node_at(&mut self, x: isize, y: isize) {
        self.node_map.insert(GraphNode::new(x, y), HashSet::new());
    }

    pub fn find_node_at(&self, x: isize, y: isize) -> Option<GraphNode> {
        for node_ref in self.node_map.keys() {
            if node_ref.x == x && node_ref.y == y {
                return Some(node_ref.clone());
            }
        }
        None
    }

    pub fn find_node_at_tuple(&self, coords: (isize, isize)) -> Option<GraphNode> {
        let (x, y) = coords;
        self.find_node_at(x, y)
    }

    pub fn connect_nodes_at(&mut self, ax: isize, ay: isize, bx: isize, by: isize) -> bool {
        match (self.find_node_at(ax, ay), self.find_node_at(bx, by)) {
            (Some(ref a), Some(ref b)) => self.connect_nodes(a, b),
            (_, _) => false,
        }
    }

    // pub fn remove_node_at(&mut self, x: isize, y: isize) -> bool {
    // 	match self.find_node_at(x,y) {
    // 		None => false,
    // 		Some(ref node) => self.remove_node(node)
    // 	}
    // }

    // pub fn get_neighbors_at(&self, x: isize, y: isize) -> Option<HashSet<GraphNode>> {
    // 	match self.find_node_at(x,y) {
    // 		None => None,
    // 		Some(ref tile) => self.get_neighbors(tile)
    // 	}
    // }

    /* dealing with nodes directly */

    pub fn get_neighbors(&self, node: &GraphNode) -> Option<HashSet<GraphNode>> {
        match self.node_map.get(node) {
            None => None,
            Some(set) => Some(set.clone()),
        }
    }

    // pub fn remove_node(&mut self, node: &GraphNode) -> bool {
    // 	match self.node_map.remove(node) {
    // 		None => false,
    // 		Some(neighbors) => {
    // 			for neighbor in neighbors.iter() {
    // 				self.node_map.get_mut(neighbor).expect("Invalid neighbor to remove!").remove(node);
    // 			}
    // 			true
    // 		}
    // 	}
    // }

    pub fn connect_nodes(&mut self, a: &GraphNode, b: &GraphNode) -> bool {
        match (self.node_map.contains_key(a), self.node_map.contains_key(b)) {
            (true, true) => {
                self.node_map.get_mut(a).expect("ea").insert(b.clone());
                self.node_map.get_mut(b).expect("ea").insert(a.clone());
                true
            }
            (_, _) => false,
        }
    }

    // pub fn get_node_set(&self) -> HashSet<GraphNode> {
    // 	let mut s = HashSet::new();
    // 	for node in self.node_map.keys() {
    // 		s.insert(node.clone());
    // 	}
    // 	s
    // }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct GraphNode {
    x: isize,
    y: isize,
}

impl GraphNode {
    pub fn new(x: isize, y: isize) -> GraphNode {
        GraphNode { x, y }
    }
    pub fn get_x(&self) -> isize {
        self.x
    }
    pub fn get_y(&self) -> isize {
        self.y
    }
    pub fn distance_to(&self, other: &GraphNode) -> f32 {
        let dy_sq: isize = isize::pow(other.get_y() - self.get_y(), 2);
        let dx_sq: isize = isize::pow(other.get_x() - self.get_x(), 2);

        (dx_sq as f32 + dy_sq as f32).sqrt()
    }
}
