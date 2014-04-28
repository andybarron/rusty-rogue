use std::comm::TryRecvResult;
use std::comm::{Empty,Disconnected,Data};
use graph::Graph;
use search::{SearchStrategy,AStarSearch};

struct Problem {
	id: uint,
	graph: Graph,
	start: (int,int),
	end: (int,int),
}

struct Solution {
	id: uint,
	path: Option<Vec<(int,int)>>,
}

struct Solver {
	prob_send: Sender<Problem>,
	soln_recv: Receiver<Solution>,
	count: uint,
}

impl Solver {
	fn new() -> Solver {
		let (prob_send,prob_recv) = channel::<Problem>();
		let (soln_send,soln_recv) = channel();

		spawn(proc(){
			let prob_recv = prob_recv;
			let soln_send = soln_send;
			let search = AStarSearch::new_diagonal();
			loop {
				match prob_recv.try_recv() {
					Empty => continue,
					Disconnected => {
						println!("Solver thread disconnected!");
						break;
					}
					Data(problem) => {
						let id = problem.id;
						let path = search.solve(&problem.graph,problem.start,problem.end);
						soln_send.send( Solution { id: id, path: path } );
					}
				}
			}
		});

		Solver {
			prob_send: prob_send,
			soln_recv: soln_recv,
			count: 0,
		}
	}
	fn queue_solve(&mut self, id: uint, graph: &Graph, start: (int,int), end: (int,int)) {
		let p = Problem{
			id: id,
			graph: graph.clone(),
			start: start,
			end: end,
		};
		self.prob_send.send(p);
		self.count += 1;
	}
	fn poll(&mut self) -> Option<Solution> {
		match self.soln_recv.try_recv() {
			Empty => None,
			Disconnected => fail!("ERROR: Solver task killed prematurely"),
			Data(soln) => {
				self.count -= 1;
				Some(soln)
			}
		}
	}
	fn get_problem_count(&self) -> uint {
		self.count
	}
}