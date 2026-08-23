#[derive(Debug)]

pub struct Calculator {
    pub state: State,
}

impl Calculator {
    pub fn new() -> Self {
        Self{
            state: State::new(),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub value: i128,
}

impl State {
    fn new() -> Self {
        Self{
            value: 0,
        }
    }
}
