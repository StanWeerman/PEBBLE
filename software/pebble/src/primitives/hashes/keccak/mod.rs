use crate::primitives::permutation_functions::keccak_f::State;

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
        println!("{:?}", msg);
        for i in 0..((msg.len() / RATE) - 1) {
            print!("{}", i);
            let mut arr = [false; RATE];
            arr.copy_from_slice(&msg[(RATE * i)..(RATE * (i + 1))]);
            println!("{:?}", arr);
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
        let mut sponge: Sponge<50, 5> = Sponge::new(vec![
            true, true, false, false, false, true, false, false, true, false, true, true, false,
            false, true, false, true, false, false, true, true, true, false, false, true, false,
            false, true, false, true, true, true, false, false, false, true, false, false, true,
            false, true, true, false, false, false, true, false, false, true, false, true, true,
            false, false, true, false, true, false, false, true, true, true, false, false, true,
            false, false, true, false, true, true, true, false, false, false, true, false, false,
            true, false, true, true, false, false, false, true, false, false, true, false, true,
            true, false, false, true, false, true, false, false, true, true, true, false, false,
            true, false, false, true, false, true, true, true, false, false, false, true, false,
            false, true, false,
        ]);
        let result = sponge.run_keccak();
        println!("Result is: {:?}", result);
    }
    #[test]
    fn string_test() {
        println!("Test String:");
        let test_string = "Hello World";
        let test_bytes = test_string.as_bytes();
        let mut test_bits = vec![];
        for byte in test_bytes {
            let mut test_bit = [false; 8];
            for i in 0..7 {
                let shifted_byte = byte >> i;
                let cur_bit = shifted_byte & 1;
                test_bit[7 - i] = (cur_bit == 1);
            }
            test_bits.extend_from_slice(&test_bit)
        }
        let mut sponge: Sponge<50, 5> = Sponge::new(test_bits);
        let result = sponge.run_keccak();
        println!("Result is: {:?}", result);
    }
}
