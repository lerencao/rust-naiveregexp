//! moudule for finite automata.

use std::collections::HashSet;
use std::hash::Hash;

trait State: Eq + Copy + Hash + Clone {}
trait Symbol: Eq + Copy + Hash + Clone {}

/// Transition represents the rule that moves from one state to another.
/// `S` means the state, `T` means the input symbol.
#[deriving(PartialEq, Eq, Hash, Clone)]
pub struct Transition<S: State, T: Symbol> {
  pub state: S,
  pub symbol: T,
  pub next_state: S
}

impl<S: State, T: Symbol> Transition<S, T> {
  pub fn new(state: &S, symbol: &T, next_state: &S) -> Transition<S, T> {
    Transition { state: *state, symbol: *symbol, next_state: *next_state }
  }

  /// determine whether th current transition applies to the state and symbol
  pub fn is_apply_to(&self, state: &S, symbol: &T) -> bool {
    self.state == *state && self.symbol == *symbol
  }
}

/// a collection of transition rules
pub struct TransitionRelation<S: State, T: Symbol> {
  transitions: HashSet<Transition<S, T>>
}

impl<S: State, T: Symbol> TransitionRelation<S, T> {
  pub fn new(transitions: &HashSet<Transition<S, T>>) -> TransitionRelation<S, T> {
    TransitionRelation { transitions: transitions.clone() }
  }

  /// get the next state for the input condition
  pub fn next_state_for(&self, state: &S, symbol: &T) -> Option<S> {
    self.transition_for(state, symbol).map(|trans| { trans.next_state })
  }

  /// get the transition of the state and the symbol
  fn transition_for(&self, state: &S, symbol: &T) -> Option<&Transition<S, T>> {
    self.transitions.iter().find(|transition| {
      transition.is_apply_to(state, symbol)
    })
  }
}

pub struct DFA<'a, S: 'a + State, T: 'a + Symbol> {
  current_state: S,
  accept_states: &'a HashSet<S>,
  transition_relation: &'a TransitionRelation<S, T>
}

impl<'a, S: 'a + State, T: 'a + Symbol> DFA<'a, S, T> {
  pub fn new(current_state: &S, accept_states: &'a HashSet<S>,
             transition_relation: &'a TransitionRelation<S, T>) -> DFA<'a, S, T> {
    DFA {
      current_state: *current_state,
      accept_states: accept_states,
      transition_relation: transition_relation
    }
  }

  /// read a symbol and change the dfa state
  pub fn read_symbol(&mut self, sym: &T) {
    self.current_state = self.transition_relation
                             .next_state_for(&self.current_state, sym)
                             .expect("cannot read the symbol in the current state")
  }

  pub fn read_symbols<A: Iterator<T>>(&mut self, mut syms: A) {
    for ref sym in syms {
      self.read_symbol(sym);
    }
  }

  /// determine whether the current dfa is in an accepted state
  pub fn is_accepting(&self) -> bool {
    self.accept_states.iter().any(|&state| {
      state == self.current_state
    })
  }
}



struct DFAModel<S: State, T: Symbol> {
  start_state: S,
  accept_states: HashSet<S>,
  transition_relation: TransitionRelation<S, T>
}

impl<S: State, T: Symbol> DFAModel<S, T> {
  pub fn new(state: &S,
             accept_states: HashSet<S>,
             relation: TransitionRelation<S, T>) -> DFAModel<S, T> {
    DFAModel {
      start_state: *state,
      accept_states: accept_states,
      transition_relation: relation
    }
  }

  fn gen_dfa(&self) -> DFA<S, T> {
    DFA::new(&self.start_state, &self.accept_states, &self.transition_relation)
  }

  pub fn accept<A: Iterator<T>>(&self, seq: A) -> bool {
    let mut dfa = self.gen_dfa();
    dfa.read_symbols(seq);
    dfa.is_accepting()
  }
}
