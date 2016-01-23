//! # Divide & Conquer
//!
//! Everything related to Divide & Conquer problems.

use std::marker::PhantomData;

/// Divide & Conquer Problem.
pub trait DacProblem<S, E> {
    fn size(&self) -> usize;
    fn is_base_case(&self) -> bool;
    fn base_case_solution(&self) -> E;
    fn subproblem_count(&self) -> usize;
    fn get_subproblem(&self, i: usize) -> Self;
    fn combine(&self, solutions: Vec<E>) -> E;
    fn get_solution(&self, partial_solution: &E) -> Option<S>;
}

pub struct DacAlgorithm<S, E, P: DacProblem<S, E>> {
    phan: PhantomData<S>,
    partial_solution: E,
    problem: P,
}

impl<S, E, P: DacProblem<S, E>> DacAlgorithm<S, E, P> {
    pub fn new(problem: P) -> Self {
        let e = Self::solve(&problem);

        DacAlgorithm {
            phan: PhantomData,
            partial_solution: e,
            problem: problem,
        }
    }

    fn solve(problem: &P) -> E {
        if problem.is_base_case() {
            problem.base_case_solution()

        } else {
            let solutions = (0..problem.subproblem_count())
                                .map(|i| problem.get_subproblem(i))
                                .map(|p| Self::solve(&p))
                                .collect::<Vec<E>>();

            problem.combine(solutions)
        }
    }

    pub fn get_solution(&self) -> Option<S> {
        self.problem.get_solution(&self.partial_solution)
    }
}
