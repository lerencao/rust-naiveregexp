extern crate naiveregex;

#[cfg(test)]
mod test {
  use std::collections::HashSet;
  use naiveregex::finite_automata::Rule;
  use naiveregex::finite_automata::DFATransitions;
  use naiveregex::finite_automata::DFAModel;
  use naiveregex::finite_automata::NFATransitions;
  use naiveregex::finite_automata::NFAModel;

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

  #[test]
  fn test_nfa_model() {
    let start_state = 1i;

    let mut accept_states = HashSet::new();
    accept_states.insert(4i);

    let mut rules = HashSet::new();
    rules.insert(Rule { state: 1i,symbol: Some('a'), next_state: 1i });
    rules.insert(Rule { state: 1i,symbol: Some('b'), next_state: 1i });
    rules.insert(Rule { state: 1i,symbol: Some('b'), next_state: 2i });
    rules.insert(Rule { state: 2i,symbol: Some('a'), next_state: 3i });
    rules.insert(Rule { state: 2i,symbol: Some('b'), next_state: 3i });
    rules.insert(Rule { state: 3i,symbol: Some('a'), next_state: 4i });
    rules.insert(Rule { state: 3i,symbol: Some('b'), next_state: 4i });
    let transitions = NFATransitions { rules: rules };

    let nfa_model = NFAModel {
      start_state: start_state,
      accept_states: accept_states,
      transitions: transitions
    };

    assert!(nfa_model.accept("baa".chars()));
    assert!(!nfa_model.accept("aaa".chars()));
    assert!(nfa_model.accept("abaa".chars()));
    assert!(nfa_model.accept("aaaabaa".chars()));
  }

  #[test]
  fn test_nfa_model_with_free_moves() {
    let start_state = 1i;

    let mut accept_states = HashSet::new();
    accept_states.insert(2i);
    accept_states.insert(4i);

    let mut rules = HashSet::new();
    rules.insert(Rule { state: 1i,symbol: None, next_state: 2i });
    rules.insert(Rule { state: 1i,symbol: None, next_state: 4i });

    rules.insert(Rule { state: 2i,symbol: Some('a'), next_state: 3i });
    rules.insert(Rule { state: 3i,symbol: Some('a'), next_state: 2i });

    rules.insert(Rule { state: 4i,symbol: Some('a'), next_state: 5i });
    rules.insert(Rule { state: 5i,symbol: Some('a'), next_state: 6i });
    rules.insert(Rule { state: 6i,symbol: Some('a'), next_state: 4i });

    let transitions = NFATransitions { rules: rules };

    let nfa_model = NFAModel {
      start_state: start_state,
      accept_states: accept_states,
      transitions: transitions
    };

    assert!(nfa_model.accept("aa".chars()));
    assert!(nfa_model.accept("aaaa".chars()));
    assert!(nfa_model.accept("aaa".chars()));
    assert!(nfa_model.accept("aaaaaa".chars()));

    assert!(!nfa_model.accept("aaaaa".chars()));
    assert!(!nfa_model.accept("aaaaaaa".chars()));
  }
}
