extern crate naiveregex;

#[cfg(test)]
mod test {
  use std::collections::HashSet;
  use naiveregex::finite_automata::Rule;
  use naiveregex::finite_automata::DFATransitions;
  use naiveregex::finite_automata::DFAModel;

  #[test]
  fn test_rule() {
    let rule = Rule { state: 1i, symbol: Some('a'), next_state: 2i };
    assert!(rule.apply_to(&1i, &Some('a')));
  }

  #[test]
  fn test_transitions() {
    let mut rules = HashSet::new();
    rules.insert(Rule { state: 1i,symbol: Some('a'), next_state: 1i });
    rules.insert(Rule { state: 1i,symbol: Some('b'), next_state: 2i });
    rules.insert(Rule { state: 2i,symbol: Some('c'), next_state: 2i });

    let transitions = DFATransitions { rules: rules };
    assert_eq!(1i, *transitions.next_state(&1i, &Some('a')).unwrap());
  }

  #[test]
  fn test_dfa_model() {
    let state = 1i;

    let mut accept_states = HashSet::new();
    accept_states.insert( 2i);

    let mut rules = HashSet::new();
    rules.insert(Rule { state: 1i,symbol: Some('a'), next_state: 1i });
    rules.insert(Rule { state: 1i,symbol: Some('b'), next_state: 2i });
    rules.insert(Rule { state: 2i,symbol: Some('c'), next_state: 2i });
    let transitions = DFATransitions { rules: rules };

    let dfa_model = DFAModel {
      start_state: state,
      accept_states: accept_states,
      transitions: transitions
    };

    assert!(dfa_model.accept("aabc".chars()));
  }
}
