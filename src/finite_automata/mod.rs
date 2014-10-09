//! module for finite automation.

use std::collections::HashSet;
use std::hash::Hash;


/// The rule that moves state from one to another.
///
/// `S` - the type parameter of state.
///
/// `T` - the type parameter of input symbol.
#[deriving(PartialEq, Eq, Hash)]
pub struct Rule<S, T> {
  pub state: S,
  pub symbol: Option<T>,
  pub next_state: S
}

impl<S: PartialEq, T: PartialEq> Rule<S, T> {
  /// determine whether the rule applies to the given state and symbol
  pub fn apply_to(&self, state: &S, symbol: &Option<T>) -> bool {
    self.state == *state && self.symbol == *symbol
  }
}

/// The transition relation in DFA,
/// containing all the rules needed by the DFA.
pub struct DFATransitions<S, T> {
  pub rules: HashSet<Rule<S, T>>
}

impl<S: Eq + Hash, T: Eq + Hash> DFATransitions<S, T> {
  /// get the next state for the given state and symbol
  pub fn next_state(&self, state: &S, symbol: &Option<T>) -> Option<&S> {
    self.rule_for(state, symbol).map(|rule| &rule.next_state)
  }

  fn rule_for(&self, state: &S, symbol: &Option<T>) -> Option<&Rule<S, T>> {
    self.rules.iter().find(|rule| { rule.apply_to(state, symbol) })
  }
}

/// A running dfa instance, constructed from a DFA model
pub struct DFA<'a, S: 'a, T: 'a> {
  state: S, // current state
  accept_states: &'a HashSet<S>, // set of accept states
  transitions: &'a DFATransitions<S, T>
}

impl<'a, S: Eq + Hash + Clone, T: Eq + Hash> DFA<'a, S, T> {
  pub fn accepted(&self) -> bool {
    self.accept_states.contains(&self.state)
  }

  pub fn read_symbol(&mut self, sym: &Option<T>) {
    self.state = self.transitions.next_state(&self.state, sym)
                                 .expect("unknown input symbol")
                                 .clone()
  }
}


/// DFA template that can generates dfa instance
pub struct DFAModel<S, T> {
  pub start_state: S,
  pub accept_states: HashSet<S>,
  pub transitions: DFATransitions<S, T>
}

impl<S: Clone + Eq + Hash, T: Eq + Hash> DFAModel<S, T> {
  /// determine whether the given symbols can be accepted by the model
  pub fn accept<I: Iterator<T>>(&self, mut iter: I) -> bool {
    let mut dfa = self.gen_dfa();
    for sym in iter {
      dfa.read_symbol(&Some(sym));
    }

    dfa.accepted()
  }

  /// generate a dfa instance
  fn gen_dfa(&self) -> DFA<S, T> {
    DFA {
      state: self.start_state.clone(),
      accept_states: &self.accept_states,
      transitions: &self.transitions
    }
  }
}


/// The transition relation in NFA,
/// containing all the rules needed by the NFA.
pub struct NFATransitions<S, T> {
  pub rules: HashSet<Rule<S, T>>
}

impl<S: Eq + Hash + Clone, T: Eq + Hash> NFATransitions<S, T> {
  fn empty_move(&self, states: HashSet<S>) -> HashSet<S> {
    let mut nexts = self.next_states(&states, &None);
    if nexts.is_subset(&states) {
      states
    } else {
      nexts.extend(states.into_iter());
      self.empty_move(nexts)
    }
  }

  pub fn next_states(&self, states: &HashSet<S>, symbol: &Option<T>) -> HashSet<S> {
    states.iter().flat_map(|state| {
      // NOTE: use `move_iter` instead of `iter`
      self.next_states_for(state, symbol).into_iter()
    }).collect()
  }

  /// get the next state for the given state and symbol
  fn next_states_for(&self, state: &S, symbol: &Option<T>) -> HashSet<S> {
    self.rules.iter().filter_map(|rule| {
      if rule.apply_to(state, symbol) { Some(rule.next_state.clone()) } else { None }
    }).collect()
  }

  fn rules_for<'a>(&'a self, state: &S, symbol: &Option<T>) -> HashSet<&'a Rule<S, T>> {
    self.rules.iter()
              .filter(|rule| { rule.apply_to(state, symbol) })
              .collect()
  }
}


struct NFA<'a, S: 'a, T: 'a> {
  states: HashSet<S>,
  accept_states: &'a HashSet<S>,
  transitions: &'a NFATransitions<S, T>
}

impl<'a, S: Eq + Hash + Clone, T: Eq + Hash> NFA<'a, S, T> {
  pub fn read_symbol(&mut self, symbol: &Option<T>) {
    // do empty moves
    self.states = self.transitions.empty_move(self.states.clone());

    self.states = self.transitions.next_states(&self.states, symbol)
                                  .iter()
                                  .map(|state| { state.clone() }).collect();

    self.states = self.transitions.empty_move(self.states.clone());
  }

  pub fn accepted(&self) -> bool {
    !self.states.is_disjoint(self.accept_states)
  }
}


pub struct NFAModel<S, T> {
  pub start_state: S,
  pub accept_states: HashSet<S>,
  pub transitions: NFATransitions<S, T>
}

impl<S: Eq + Hash + Clone, T: Eq + Hash> NFAModel<S, T> {
  /// determine whether the given symbols can be accepted by the model
  pub fn accept<I: Iterator<T>>(&self, mut iter: I) -> bool {
    let mut dfa = self.gen_nfa();
    for sym in iter {
      dfa.read_symbol(&Some(sym));
    }

    dfa.accepted()
  }

  /// generate a nfa instance
  fn gen_nfa(&self) -> NFA<S, T> {
    let mut start_states = HashSet::new();
    start_states.insert(self.start_state.clone());
    NFA {
      states: start_states,
      accept_states: &self.accept_states,
      transitions: &self.transitions
    }
  }
}
