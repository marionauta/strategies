//! The Divide & Conquer recursive strategy.
//!
//! From [Wikipedia][1]:
//! > A divide and conquer algorithm works by recursively breaking down a
//! > problem into two or more sub-problems of the same (or related) type
//! > (**divide**) until these become simple enough to be solved directly
//! > (**conquer**). The solution to the sub-problems are then combined
//! > to give a solution to the original problem.
//!
//! To use this strategy you need to implement the [`DacProblem`][2] trait and
//! everything else is handled for you.
//!
//! [1]: https://en.wikipedia.org/wiki/Divide_and_conquer_algorithms
//! [2]: trait.DacProblem.html

use std::marker::PhantomData;

/// Divide & Conquer Problem.
///
/// You have to implement this trait in order to use the algorithm. Simply fill
/// all the methods and you're all set. We have two generic types:
///
/// - `E`: A partial solution, wich we work with.
/// - `S`: The final solution, the output of the algorithm.
///
/// Sometimes `E` and `S` are the same type because we don't need conversion
/// between the partial and the final solution (e.g. when calculating a
/// factorial), but if we are working with a list or an array, `E` could simply
/// be a pointer to an element and `S` the actual element.
///
/// Once you have defined the problem, use [`DacAlgorithm`][1] to solve it.
///
/// [1]: struct.DacAlgorithm.html
pub trait DacProblem<S, E> {
    /// The size of your problem. Usually referred to as `n`.
    ///
    /// It should decrease as you split the problem, otherwise the algorithm
    /// will never stop, and we don't want that.
    fn size(&self) -> usize;

    /// The moment when the problem is simple enough to be solved directly.
    ///
    /// Here `DacProblem::size` is the smallest.
    fn is_base_case(&self) -> bool;

    /// The base case's solution.
    fn base_case_solution(&self) -> E;

    /// The number of subproblems the problem has.
    fn subproblem_count(&self) -> usize;

    /// Return the subproblem corresponding to the given `i`. (i.e. the first
    /// subproblem, the second...).
    ///
    /// `i` is in the range [0, `DacProblem::subproblem_count`).
    fn get_subproblem(&self, i: usize) -> Self;

    /// How to combine the subproblems' solutions into the current problem's
    /// solution.
    fn combine(&self, solutions: Vec<E>) -> E;

    /// Transforms the partial solution `E`, if it is possible, into a final
    /// solution `S`.
    fn get_solution(&self, partial_solution: &E) -> Option<S>;
}

/// The problem solver.
///
/// Solves a divide & conquer problem without memory. Useful if we don't need
/// to store partial solutions.
pub struct DacAlgorithm<S, E, P: DacProblem<S, E>> {
    phan: PhantomData<S>,
    partial_solution: E,
    problem: P,
}

impl<S, E, P: DacProblem<S, E>> DacAlgorithm<S, E, P> {
    /// Solve the `problem` problem.
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

    /// Get the final solution.
    pub fn get_solution(&self) -> Option<S> {
        self.problem.get_solution(&self.partial_solution)
    }
}
