use crate::primitives::permutation_functions::keccak_f::{State, lane::Lane};

pub struct Sponge<const WIDTH: usize> {
    state: State<WIDTH>,
    msg: Vec<[bool; WIDTH]>,
}

impl<const WIDTH: usize> Sponge<WIDTH> {
    pub fn new(msg: Vec<bool>) -> Self {
        Self {
            state: State::new(),
            msg: Sponge::padding(msg),
        }
    }
    fn padding(msg: Vec<bool>) -> Vec<[bool; WIDTH]> {
        todo!()
    }
    fn absorb(&mut self) {
        for block in &mut self.msg {
            (self.state).state = Lane(self.state) ^ Lane(block)
        }
    }
    fn squeeze() {}
}
