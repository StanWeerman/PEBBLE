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
        let mut padded_msg: Vec<[bool; RATE]> = vec![];
        for i in 0..((msg.len() / RATE) - 1) {
            let mut arr = [false; RATE];
            arr.copy_from_slice(&msg[(RATE * i)..(RATE * (i + 1))]);
            padded_msg.append(&mut vec![arr]);
        }
        return padded_msg;
    }
    fn run_keccak(&mut self) -> Vec<bool> {
        self.absorb();
        let mut output: Vec<bool> = vec![];
        for _ in 0..12 {
            output.extend_from_slice(&self.squeeze());
        }
        return output;
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
        return new_state;
    }
    fn state_to_block(state: [[[bool; WIDTH]; 5]; 5]) -> [bool; RATE] {
        let mut new_block = [false; RATE];
        let mut index = 0;
        for i in state {
            for j in i {
                for k in j {
                    if index >= RATE {
                        return new_block;
                    }
                    new_block[index] = k;
                    index = index + 1;
                }
            }
        }
        return new_block;
    }
    fn squeeze(&mut self) -> [bool; RATE] {
        for block in &mut self.msg {
            let next_msg_state = Sponge::block_to_state(block);
            self.state.add_to_state(next_msg_state);
            self.state.permute();
        }
        return Sponge::state_to_block(self.state.get_state());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        println!("Test 1:");
        let mut sponge: Sponge<50, 10> = Sponge::new(vec![
            true, true, false, false, false, false, false, false, false, false,
        ]);
        let result = sponge.run_keccak();
        println!("Result is: {:?}", result);
    }
}
