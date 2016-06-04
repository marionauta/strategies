//! The Backtracking stategy.

use std::f64;
use std::hash::Hash;
use std::collections::HashSet;
use std::marker::PhantomData;

use super::Type;

pub trait State<S, A> {
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
    fn is_final(&self) -> bool {
        self.size() == 0
    }

    /// List of different ways the problem can go forward (and backwards after).
    fn alternatives(&self) -> Vec<A>;

    fn forward(&mut self, a: A);
    fn backward(&mut self, a: A);

    /// Current state's value.
    fn value(&self) -> f64;

    /// An estimation of the best value the problem could reach if it chose the
    /// specified alternative.
    fn estimated_value(&self, _: A) -> f64 {
        match self.problem_type() {
            Type::Max => f64::MAX,
            Type::Min => f64::MIN,
            _ => 0.0, // In this case, the value is never used.
        }
    }

    /// Solution to a final state.
    fn solution(self) -> Option<S>;
}

pub struct Algorithm<P, S, A>
    where P: State<S, A>
{
    // This two are here so `S` and `A` are used.
    phans: PhantomData<S>,
    phana: PhantomData<A>,

    solution_count: usize,

    solutions: HashSet<P>,
    best_value: f64,
    success: bool,
    state: P,
}

impl<P, S, A> Algorithm<P, S, A>
    where P: State<S, A> + Clone + Eq + Hash + Ord,
          A: Clone
{
    /// Create a new algorithm to solve `state`.
    pub fn new(state: P) -> Self {
        Algorithm {
            phans: PhantomData,
            phana: PhantomData,

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
    pub fn all_solutions(&self) -> HashSet<P> {
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
    fn is_to_prune(&self, a: A) -> bool {
        match self.state.problem_type() {
            Type::Max => self.state.estimated_value(a) <= self.best_value,
            Type::Min => self.state.estimated_value(a) >= self.best_value,
            _ => false,
        }
    }

    pub fn solve(&mut self) {
        if self.state.is_final() {
            self.update_solutions();
            self.success = self.solutions.len() >= self.solution_count;

        } else {
            let alternatives = self.state
                .alternatives()
                .into_iter()
                .filter(|a| !self.is_to_prune(a.clone()))
                .collect::<Vec<A>>();

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
