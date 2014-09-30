extern crate naiveregex;

#[cfg(test)]
mod test {
  use std::collections::HashSet;
  use naiveregex::finite_automata::State;
  use naiveregex::finite_automata::Symbol;
  use naiveregex::finite_automata::Transition;
  use naiveregex::finite_automata::TransitionRelation;
  use naiveregex::finite_automata::DFA;
  use naiveregex::finite_automata::DFAModel;

  impl State for int {}
  impl Symbol for char {}

  #[test]
  fn test_transition() {
    let rule = Transition::new(&1i, & 'a', &2i);
    assert_eq!(2i, rule.next_state);
  }

  #[test]
  fn test_transition_relation() {
    let mut rules = HashSet::new();
    rules.insert(Transition::new(&1i, & 'a', &1i));
    rules.insert(Transition::new(&1i, & 'b', &2i));
    rules.insert(Transition::new(&2i, & 'c', &2i));

    let relation = TransitionRelation::new(&rules);

    let some_rule = relation.transition_for(&1i, & 'a').unwrap();
    assert_eq!(some_rule.state, 1);
    assert_eq!(some_rule.symbol, 'a');
  }

  #[test]
  fn test_dfa() {
    let state = 1i;

    let mut accept_states = HashSet::new();
    accept_states.insert( 2i);

    let mut rules = HashSet::new();
    rules.insert(Transition::new(&1i, & 'a', &1i));
    rules.insert(Transition::new(&1i, & 'b', &2i));
    rules.insert(Transition::new(&2i, & 'c', &2i));

    let relation = TransitionRelation::new(&rules);

    let mut dfa = DFA::new(&state, &accept_states, &relation);

    assert!(!dfa.is_accepting());

    dfa.read_symbols("aabcc".chars());
    assert!(dfa.is_accepting());
  }

  #[test]
  fn test_dfa_model() {
    let state = 1i;

    let mut accept_states = HashSet::new();
    accept_states.insert( 2i);

    let mut rules = HashSet::new();
    rules.insert(Transition::new(&1i, & 'a', &1i));
    rules.insert(Transition::new(&1i, & 'b', &2i));
    rules.insert(Transition::new(&2i, & 'c', &2i));

    let relation = TransitionRelation::new(&rules);

    let dfa_model = DFAModel::new(&state, accept_states, relation);
    assert!(dfa_model.accept("aabc".chars()))
  }

}
