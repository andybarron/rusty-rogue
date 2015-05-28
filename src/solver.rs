use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::mpsc::RecvError;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::TryRecvError::*;
use graph::Graph;
use search::{SearchStrategy,AStarSearch};
use std::sync::{Arc,RwLock};
use std::thread;

struct Problem {
	id: usize,
	graph: Arc<RwLock<Graph>>,
	start: (isize,isize),
	end: (isize,isize),
}

pub struct Solution {
	pub id: usize,
	pub path: Option<Vec<(isize,isize)>>,
}

pub struct Solver {
	prob_send: Sender<Option<Problem>>,
	soln_recv: Receiver<Solution>,
	count: usize,
}

impl Solver {
	pub fn new() -> Solver {
		let (prob_send,prob_recv) = channel();
		let (soln_send,soln_recv) = channel();

		let this = Solver {
			prob_send: prob_send,
			soln_recv: soln_recv,
			count: 0,
		};

		thread::spawn(move || {
			let prob_recv = prob_recv;
			let soln_send = soln_send;
			let search = AStarSearch::new_diagonal();
			loop {
				match prob_recv.recv() {
					Ok(maybe_problem) => match maybe_problem {
						Some(problem) => {
							let id = problem.id;
							let path = search.solve(
								&*problem.graph.read().ok()
								.expect("threading sucks"),problem.start,
								problem.end);
							soln_send.send( Solution { id: id, path: path } );
						}
						None => break,
					},
					Err(_) => panic!("ERROR: Solver receiver disconnected")
				}
			}
		});

		this
	}
	pub fn queue_solve(&mut self, id: usize, graph: Arc<RwLock<Graph>>, start: (isize,isize), end: (isize,isize)) {
		let p = Problem{
			id: id,
			graph: graph,
			start: start,
			end: end,
		};
		self.prob_send.send(Some(p));
		self.count += 1;
	}
	pub fn poll(&mut self) -> Option<Solution> {
		match self.soln_recv.try_recv() {
			Ok(soln) => {
				self.count -= 1;
				Some(soln)
			}
			Err(e) => match e {
				Empty => None,
				Disconnected => panic!("ERROR: Solver task killed prematurely"),
			}
		}
	}
	pub fn get_problem_count(&self) -> usize {
		self.count
	}
}

impl Drop for Solver {
	fn drop(&mut self) {
		self.prob_send.send(None);
	}
}