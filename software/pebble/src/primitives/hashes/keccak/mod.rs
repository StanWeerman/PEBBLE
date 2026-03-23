use crate::primitives::permutation_functions::keccak_f::{State, lane::Lane};

pub struct Sponge<const WIDTH: usize, const RATE: usize> {
    state: State<WIDTH>,
    msg: Vec<[bool; RATE]>,
}

impl<const WIDTH: usize, const RATE: usize> Sponge<WIDTH, RATE> {
    pub fn new(msg: Vec<bool>) -> Self {
        Self {
            state: State::new(),
            msg: Sponge::<WIDTH, RATE>::padding(msg),
        }
    }
    fn padding(msg: Vec<bool>) -> Vec<[bool; RATE]> {
        todo!()
    }
    fn absorb(&mut self) {
        for block in &mut self.msg {
            let next_msg_state = Sponge::block_to_state(block);
            self.state.add_to_state(next_msg_state);
            self.state.permute();
        }
    }
    fn block_to_state(block: &mut [bool; RATE]) -> [[[bool; WIDTH]; 5]; 5] {
        let mut new_state = [[[false; WIDTH]; 5]; 5];
        for i in 0..(RATE / WIDTH) {
            new_state[i % 5][i / 5] = block[i * RATE / WIDTH..(i + 1) * RATE / WIDTH]
                .try_into()
                .unwrap();
        }
        todo!()
    }
    fn squeeze() -> Vec<bool> {
        todo!()
    }
}
