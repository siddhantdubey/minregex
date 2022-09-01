use std::vec::Vec;
use std::iter::Map;

pub struct State {
    pub name: String,
    pub transitions: Vec<Transition>,
    pub starts_groups: (),
    pub ends_groups: (),
}

pub struct Transition {
    pub next_state: State,
    pub matcher: char,
}

impl State {
    fn new(name: String, transitions: Vec<Transition>) -> Self {
        State {
            name,
            transitions,
            starts_groups: (),
            ends_groups: (),
        }
    }

    fn add_transition(&mut self, next_state: State, matcher: char) {
        let transition: Transition = Transition {next_state, matcher};
        self.transitions.push(transition);
    }

    fn undo_transition(&mut self, next_state: State, matcher: char) {
        let transition: Transition = Transition {next_state, matcher};
        self.transitions.insert(0, transition);
    }
}

pub trait Matcher {
    fn matches(&self, _character: char) -> bool {
        false
    }

    fn is_epsilon()  -> bool {
        false
    }
}

pub struct CharacterMatcher {
    pub c: char,
}

impl Matcher for CharacterMatcher {
    fn matches(&self, character: char) -> bool {
        character == self.c 
    }

    fn is_epsilon() -> bool {
        false
    }
}

pub struct EpsilonMatcher {}

impl Matcher for EpsilonMatcher {
    fn matches(&self, _character: char) -> bool {
        true
    }

    fn is_epsilon() -> bool{
        true
    }
}

pub struct EngineNFA {
    states: Map<State, State>,
    initial_state: State,
    ending_states: Vec<State>,
}

impl EngineNFA {
    fn new(&self, states: Map<State, State>, initial_state: State, ending_states: Vec<State>) -> Self {
        EngineNFA {
            states,
            initial_state,
            ending_states,
        }
    }

    fn set_initial_state(&mut self, state: State) {
        self.initial_state = state;
    }

    fn set_ending_states(&mut self, states: Vec<State>) {
        self.ending_states = states;
    }
}
