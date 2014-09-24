//! moudule for finite automata.

trait State: Eq + Copy {}
trait Symbol: Eq + Copy {}

/// Transition represents the rule that moves from one state to another.
/// `S` means the state, `T` means the input symbol.
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

  /// get the next state from the transition
  pub fn follow(&self) -> &S {
    &self.next_state
  }
}

/// a collection of transition rules
pub struct TransitionRelation<S: State, T: Symbol> {
  transitions: Vec<Transition<S, T>>
}

impl<S: State, T: Symbol> TransitionRelation<S, T> {
  pub fn new(transitions: Vec<Transition<S, T>>) -> TransitionRelation<S, T> {
    TransitionRelation { transitions: transitions }
  }

  /// get the next state for the input condition
  pub fn next_state_for(&self, state: &S, symbol: &T) -> Option<&S> {
    self.transition_for(state, symbol).map(|trans| { trans.follow() })
  }

  /// get the transition of the state and the symbol
  fn transition_for(&self, state: &S, symbol: &T) -> Option<&Transition<S, T>> {
    self.transitions.iter().find(|transition| {
      transition.is_apply_to(state, symbol)
    })
  }
}

pub struct DFA<S: State, T: Symbol> {
  current_state: S,
  accept_states: Vec<S>,
  transition_relation: TransitionRelation<S, T>
}

impl<S: State, T: Symbol> DFA<S, T> {
  pub fn new(current_state: &S, accept_states: Vec<S>,
             transition_relation: TransitionRelation<S, T>) -> DFA<S, T> {
    DFA {
      current_state: *current_state,
      accept_states: accept_states,
      transition_relation: transition_relation
    }
  }

  /// read a symbol and change the dfa state
  pub fn read_symbol(&mut self, sym: &T) {
    self.current_state = *self.transition_relation
                             .next_state_for(&self.current_state, sym)
                             .expect("cannot read the symbol in the current state")
  }

  pub fn read_symbol_seq<I: Iterator<T>>(&mut self, syms: &mut I) {
    for ref sym in *syms {
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


#[cfg(test)]
mod test {
  use super::State;
  use super::Symbol;
  use super::Transition;
  use super::TransitionRelation;

  impl State for int {}
  impl Symbol for char {}

  #[test]
  fn test_transition() {
    let rule = Transition::new(&1i, & 'a', &2i);
    assert_eq!(2i, rule.next_state);
  }

  #[test]
  fn test_transition_relation() {
    let rules = vec![
      Transition::new(&1i, & 'a', &1i),
      Transition::new(&1i, & 'b', &2i),
      Transition::new(&2i, & 'c', &2i)
    ];
    let relation = TransitionRelation::new(rules);

    let some_rule = relation.transition_for(&1i, & 'a').unwrap();
    assert_eq!(some_rule.state, 1);
    assert_eq!(some_rule.symbol, 'a');
  }
}
