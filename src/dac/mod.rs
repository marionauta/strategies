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

use std::collections::HashMap;
use std::hash::Hash;
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
/// Once you have defined the problem, use [`DacAlgorithm`][1] or
/// [`DacMemAlgorithm`][2] to solve it.
///
/// # Examples
///
/// In [Required Methods][3] you'll find the description to all the methods you
/// need to implement. Here is an example solving the [Fibonacci][4] problem:
///
///     use strategies::dac::DacProblem;
///     use strategies::dac::{DacAlgorithm, DacMemAlgorithm};
///
///     // This is where we store our problem.
///     // (The derive part is only needed for DacMemAlgorithm).
///     #[derive(Clone, Eq, PartialEq, Hash)]
///     struct Fibonacci(u64);
///
///     // Since the problem is simple, `E` and `S` are the same type.
///     impl DacProblem<u64, u64> for Fibonacci {
///         fn size(&self) -> usize {
///             self.0 as usize
///         }
///
///         // We know directly that if the problem is 0 or 1, the solution
///         // is the same as the problem.
///         fn is_base_case(&self) -> bool {
///             self.size() < 2
///         }
///
///         fn base_case_solution(&self) -> u64 {
///             self.0
///         }
///
///         // As the fibonacci definition has 2 recursive subproblems...
///         fn subproblem_count(&self) -> usize {
///             2
///         }
///
///         fn get_subproblem(&self, i: usize) -> Fibonacci {
///             match i {
///                 0 => Fibonacci(self.0 - 1),
///                 _ => Fibonacci(self.0 - 2),
///             }
///         }
///
///         // We add the to subproblems' solutions.
///         fn combine(&self, solutions: Vec<u64>) -> u64 {
///             let a = solutions.get(0).unwrap();
///             let b = solutions.get(1).unwrap();
///
///             a + b
///         }
///
///         fn get_solution(&self, partial_solution: &u64) -> Option<u64> {
///             Some(*partial_solution)
///         }
///     }
///
///     // Now we can use DacAlgorithm...
///     let p = DacAlgorithm::new(Fibonacci(5));
///     let s = p.get_solution().unwrap();
///     assert_eq!(s, 5);
///
///     // ...or DacMemAlgorithm, which is the best option for this problem
///     // (go to DacMemAlgorithm's page to see why).
///     let p = DacMemAlgorithm::new(Fibonacci(50));
///     let s = p.get_solution().unwrap();
///     assert_eq!(s, 12586269025);
///
/// **Note**: Although you will be doing any other kind of problems, you'll
/// have to pick the best types for the problem. In the above's example I used
/// `u64` because it's part of Rust, but it can't store the solution to
/// Fibonacci(100).
///
/// [1]: struct.DacAlgorithm.html
/// [2]: struct.DacMemAlgorithm.html
/// [3]: #required-methods
/// [4]: https://en.wikipedia.org/wiki/Fibonacci_number
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
///
/// # Usage
///
/// First, implement a `DacProblem`. Then, solve it. A good example is in
/// [`DacProblem`][1].
///
/// [1]: trait.DacProblem.html#examples
pub struct DacAlgorithm<P, S, E>
    where P: DacProblem<S, E>
{
    phan: PhantomData<S>,
    partial_solution: E,
    problem: P,
}

impl<P, S, E> DacAlgorithm<P, S, E>
    where P: DacProblem<S, E>
{
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

/// The problem solver, with memory.
///
/// # Explanation
///
/// Solves a divide & conquer problem with memory: it stores the partial
/// solutions of all the subproblems, in case they are needed by another
/// subproblem. An usual example is the [Fibonacci sequence][1]:
///
/// ``` text
/// fib(0) = 0
/// fib(1) = 1
/// fib(n) = fib(n - 1) + fib(n - 2)
/// ```
///
/// As you can see, `fib(5)` needs `fib(4)` and `fib(3)`. But then `fib(4)` also
/// needs `fib(3)`. Memory here is useful so we don't recalculate values.
///
/// # Usage
///
/// Using this is the same as using [`DacAlgorithm`][2] but you problem has to
/// implement some traits:
///
/// - `DacProblem` has to implement the following traits:
///     - `Eq` (and therefore `PartialEq`)
///     - `Hash`
///     - `Clone`
/// - `E` has to be `Clone` too.
///
/// Which is not complicated to do:
///
///     #[derive(Eq, PartialEq, Hash, Clone)]
///     struct Fibonacci(u64);
///
/// [1]: https://en.wikipedia.org/wiki/Fibonacci_number
/// [2]: struct.DacAlgorithm.html#usage
pub struct DacMemAlgorithm<P, S, E>
    where P: DacProblem<S, E> + Eq + Hash + Clone,
          E: Clone
{
    phan: PhantomData<S>,
    solutions: HashMap<P, E>,
    problem: P,
}

impl<P, S, E> DacMemAlgorithm<P, S, E>
    where P: DacProblem<S, E> + Eq + Hash + Clone,
          E: Clone
{
    pub fn new(problem: P) -> Self {
        let mut map = HashMap::new();
        Self::solve(&problem, &mut map);

        DacMemAlgorithm {
            phan: PhantomData,
            solutions: map,
            problem: problem,
        }
    }

    fn solve(problem: &P, mut solutions: &mut HashMap<P, E>) -> E {
        if problem.is_base_case() {
            problem.base_case_solution()

        } else if solutions.contains_key(&problem) {
            solutions.get(&problem).unwrap().clone()

        } else {
            let solution = {
                let solutions = (0..problem.subproblem_count())
                                    .map(|i| problem.get_subproblem(i))
                                    .map(|p| Self::solve(&p, &mut solutions))
                                    .collect::<Vec<E>>();

                problem.combine(solutions)
            };

            solutions.insert(problem.clone(), solution);
            solutions.get(&problem).unwrap().clone()
        }
    }

    pub fn get_solution(&self) -> Option<S> {
        self.solutions
            .get(&self.problem)
            .and_then(|e| self.problem.get_solution(e))
    }
}
