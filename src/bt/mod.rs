//! The Backtracking stategy.
//!
//! From [Wikipedia][1]:
//!
//! > Backtracking is a general algorithm for finding all (or some) solutions
//! > to computational problems, notably _constraint satisfaction problems_,
//! > that builds solution candidates, and abandons each partial candidate
//! > - **backtracks** - as soon as it determines that it cannot be a valid
//! > solution.
//!
//! To use this strategy you need to implement the [`bt::State`][2] trait and
//! everything else is handled for you.
//!
//! [1]: https://en.wikipedia.org/wiki/Backtracking
//! [2]: trait.State.html

use std::f64;
use std::hash::Hash;
use std::collections::HashSet;

use super::Type;

/// Backtracking problem state.
///
/// In order to use this strategy you have to implement this trait. There are
/// two generic types:
///
/// * `A`: Alternatives the algorithm uses to explore many posible solutions.
/// * `S`: The final solution, the output of the algorithm.
///
/// In backtracking, this is called _state_ instead of _problem_ because it
/// isn't a mere description, it **holds the values** while the algorithm is
/// running. When a `State` is created, it has to hold the problem's
/// **initial state**.
///
/// Maybe the two most important methods are `State::forward` and
/// `State::backward` because they define **how the `State` changes** when the
/// algorithm decides to take a certain alternative. Calling `forward` and
/// then `backward` must lead to the exact same state as the one before calling
/// them.
///
/// Once you have defined the **initial state**, use [`bt::Algorithm`][1] to
/// solve it.
///
/// [1]: struct.Algorithm.html
pub trait State {
    type Solution;
    type Alternative: Clone;

    /// Type of the problem.
    ///
    /// Supported values are `Max` and `Min` for optimization problems, and
    /// `All` if you want to retrieve all the posible solutions.
    fn problem_type(&self) -> Type;

    /// Size of your problem. Usually referred to as `n`.
    ///
    /// It should decrease when going forward and increase when going backward.
    fn size(&self) -> usize;

    /// Final state - when going forward is imposible.
    ///
    /// The default implementation is valid for a reduced number of problems.
    /// You may have other conditions to stop the algorithm: your budget has
    /// been spent, you don't have room left...
    fn is_final(&self) -> bool {
        self.size() == 0
    }

    /// List of different ways the problem can go forward (and backwards after).
    fn alternatives(&self) -> Vec<Self::Alternative>;

    /// Apply a change with the `a` alternative.
    ///
    /// The state must change its properties, according to what the `a`
    /// alternative does.
    fn forward(&mut self, a: Self::Alternative);

    /// Discard a change taken with the `a` alternative.
    ///
    /// It has to revert the state to the previos one, just before calling the
    /// `State::forward` method.
    fn backward(&mut self, a: Self::Alternative);

    /// Current state's value.
    ///
    /// Only called when `State::is_final`, gives the algorithm information
    /// about how good this final state is.
    fn value(&self) -> f64;

    /// An estimation of the best value the problem could reach if it chose the
    /// specified alternative.
    fn estimated_value(&self, _a: Self::Alternative) -> f64 {
        match self.problem_type() {
            Type::Max => f64::MAX,
            Type::Min => f64::MIN,
            _ => 0.0, // In this case, the value is never used.
        }
    }

    /// Solution to a final state.
    fn solution(self) -> Option<Self::Solution>;
}

/// The problem solver.
///
/// Solves a backtracking state.
///
/// # Usage
///
/// First, implement a [`bt::State`][1]. Then `solve` it.
///
/// The _catch_ is that your problem,`State`, has to implement some traits:
///
/// * `Clone`
/// * `Eq` (and `PartialEq`)
/// * `Hash`
/// * `Ord` (and `PartialOrd`)
///
/// ```
/// #[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
/// struct MyState;
/// ```
///
/// And your alternative, `A`, has to be `Clone` too. But that's not much of a
/// hassle as you'll probably use `bool` or numbers and all those already
/// implement the trait.
///
/// [1]: trait.State.html
pub struct Algorithm<S: State> {
    // The only 'option' to change a bit the algorithm's behaviour.
    // More could be added in the future.
    solution_count: usize,

    solutions: HashSet<S>,
    best_value: f64,
    success: bool,
    state: S,
}

impl<S> Algorithm<S> where S: State + Clone + Eq + Hash + Ord {
    /// Create a new algorithm to solve `state`.
    pub fn new(state: S) -> Self {
        Algorithm {
            solution_count: 100,

            solutions: HashSet::new(),
            best_value: match state.problem_type() {
                Type::Max => f64::MIN,
                Type::Min => f64::MAX,
                _ => 0.0, // In this case, the value is never used.
            },
            success: false,
            state: state,
        }
    }

    /// Change the number of solutions to calculate.
    ///
    /// If the default value is too small (e.g. your problem ramificates a lot)
    /// or too big (you don't want innecesary calculations) you can change the
    /// algorithm so it adjusts to your case.
    ///
    /// If it is set to 1, the algorithm will stop after the first solution.
    ///
    /// Default is 100.
    pub fn solution_count(mut self, count: usize) -> Self {
        self.solution_count = count;
        self
    }

    /// All the solutions calculated with the algorithm.
    pub fn all_solutions(&self) -> HashSet<S> {
        self.solutions.clone()
    }

    /// Store the current solution in the 'solutions' set if it's better than
    /// any of the allready stored, or if the problem is of type 'All'.
    fn update_solutions(&mut self) {
        let value = self.state.value();
        let problem_type = self.state.problem_type();

        if (problem_type != Type::Min && problem_type != Type::Max) ||
           (problem_type == Type::Min && value < self.best_value) ||
           (problem_type == Type::Max && value > self.best_value) {

            self.solutions.insert(self.state.clone());
            self.best_value = value;
        }
    }

    /// Decide if alternative `a` is not worthy of being explored.
    ///
    /// Here we need a well written `State::estimated_value`.
    fn is_to_prune(&self, a: S::Alternative) -> bool {
        match self.state.problem_type() {
            Type::Max => self.state.estimated_value(a) <= self.best_value,
            Type::Min => self.state.estimated_value(a) >= self.best_value,
            _ => false,
        }
    }

    /// Solve the problem.
    ///
    /// After creating the `Algorithm` with a `State`, solve the problem so you
    /// can get all the solutions.
    pub fn solve(&mut self) {
        if self.state.is_final() {
            self.update_solutions();
            self.success = self.solutions.len() >= self.solution_count;

        } else {
            let alternatives = self.state
                .alternatives()
                .into_iter()
                .filter(|a| !self.is_to_prune(a.clone()))
                .collect::<Vec<S::Alternative>>();

            for alternative in alternatives {
                self.state.forward(alternative.clone());
                self.solve();
                self.state.backward(alternative.clone());

                if self.success {
                    break;
                }
            }
        }
    }
}
