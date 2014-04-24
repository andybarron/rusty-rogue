
mod graph;
mod search;

fn main() {
	let graph = Graph::new();
	let obstacles = vec!(
		(4,4),
		(4,5),
		(5,4),
		(5,5),
	);
	for y in range(0,10) {
		for x in range(0,10) {
			let coords = (x,y);
			match obstacles.contains(&coords) {
				true => break,
				false => {
					graph.add_node_at(x,y);
				}
			}
		}
	}
}