use std::collections::HashMap;

pub struct State {
    name: String,
    transitions: Vec<Transition>,
    starts_groups: Vec<String>,
    ends_groups: Vec<String>,
}

impl State {
    pub fn new(name: String) -> State {
        State {
            name,
            transitions: Vec::new(),
            starts_groups: Vec::new(),
            ends_groups: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, transition: Transition) {
        self.transitions.push(transition);
    }

    pub fn unshift_transition(&mut self, transition: Transition) {
        self.transitions.insert(0, transition);
    }
}

pub struct Transition {
    pub to: String,
    pub condition: Condition,
}

impl Transition {
    pub fn new(to: String, condition: Condition) -> Transition {
        Transition {
            to,
            condition,
        }
    }
}

pub enum Condition {
    Any,
    Char(char),
    Range(char, char),
    Set(Vec<char>),
    Not(Box<Condition>),
}

impl Condition {
    pub fn matches(&self, c: char) -> bool {
        match *self {
            Condition::Any => true,
            Condition::Char(ch) => ch == c,
            Condition::Range(start, end) => start <= c && c <= end,
            Condition::Set(ref set) => set.contains(&c),
            Condition::Not(ref condition) => !condition.matches(c),
        }
    }
}

pub struct NFA {
    states: HashMap<String, State>,
    start_state: String,
    end_states: Vec<String>,
}

impl NFA {
    fn new() -> NFA {
        NFA {
            states: HashMap::new(),
            start_state: String::new(),
            end_states: Vec::new(),
        }
    }

    fn set_start_state(&mut self, state: String) {
        self.start_state = state;
    }

    fn set_end_states(&mut self, states: Vec<String>) {
        self.end_states = states;
    }

    fn add_state(&mut self, state: State) {
        self.states.insert(state.name.clone(), state);
    }

    fn declare_states(&mut self, names: Vec<String>) {
        for name in names {
            self.add_state(State::new(name));
        }
    }

    fn add_transition(&mut self, from: String, to: String, condition: Condition) {
        let state = self.states.get_mut(&from).unwrap();
        state.add_transition(Transition::new(to, condition));
    }

    fn unshift_transition(&mut self, from: String, to: String, condition: Condition) {
        let state = self.states.get_mut(&from).unwrap();
        state.unshift_transition(Transition::new(to, condition));
    }
}
