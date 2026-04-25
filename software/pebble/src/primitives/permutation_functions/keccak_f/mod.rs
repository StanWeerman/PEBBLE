pub mod lane;
use lane::Lane;
use lfsr::{LFSR, galois::Galois8};

use std::fmt::Display;

pub struct State<const WIDTH: usize> {
    state: [[[bool; WIDTH]; 5]; 5],
    lfsr: Galois8,
}
impl<const WIDTH: usize> Display for State<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "[");
        for i in self.state {
            write!(f, "\n");
            for j in i {
                write!(f, "\n");
                for k in j {
                    if k == false {
                        write!(f, "{}", 0);
                    } else {
                        write!(f, "{}", 1);
                    }
                }
            }
        }
        write!(f, "")
        //write!(f, "]")
    }
}
impl<const WIDTH: usize> State<WIDTH> {
    pub const L: usize = (WIDTH / 25).ilog2() as usize;
    pub fn new() -> Self {
        State {
            state: [[[false; WIDTH]; 5]; 5],
            lfsr: Galois8::new(0),
        }
    }
    pub fn get_state(&mut self) -> [[[bool; WIDTH]; 5]; 5] {
        return self.state.clone();
    }
    pub fn add_to_state(&mut self, new_state: [[[bool; WIDTH]; 5]; 5]) {
        for x in 0..5 {
            for y in 0..5 {
                self.state[x][y] = Lane(self.state[x][y]) ^ Lane(new_state[x][y])
            }
        }
    }
    pub fn permute(&mut self) {
        let mut round = 0;
        while round < 24 {
            self.round(&mut round);
            round += 1;
        }
    }
    pub fn round(&mut self, round: &mut usize) {
        self.theta();
        self.rho();
        self.pi();
        self.chi();
        self.iota(*round);
        *round += 1;
    }
    pub fn chi(&mut self) {
        for y in 0..5 {
            for x in 0..5 {
                self.state[x][y] = Lane(self.state[x][y])
                    ^ Lane(
                        Lane(!Lane(self.state[(x + 1) % 4][y])) & Lane(self.state[(x + 2) % 4][y]),
                    );
            }
        }
    }
    pub fn theta(&mut self) {
        let mut c = [[false; WIDTH]; 5];
        for x in 0..5 {
            c[x] = (self.state[x][0]).clone();
            for y in 1..5 {
                c[x] = Lane(c[x]) ^ Lane(self.state[x][y]);
            }
        }
        let mut d = [[false; WIDTH]; 5];
        for x in 0..5 {
            d[x] = Lane(c[(x + 5) % 5]) ^ Lane(Lane(c[(x + 1) % 5]).rotate(1));
            for y in 0..5 {
                self.state[x][y] = Lane(self.state[x][y]) ^ Lane(d[x]);
            }
        }
    }
    pub fn pi(&mut self) {
        let mut new_state = [[[false; WIDTH]; 5]; 5];
        for x in 0..5 {
            for y in 0..5 {
                let x_new = (1 * y) % 5;
                let y_new = (2 * x + 3 * y) % 5;
                new_state[x_new][y_new] = self.state[x][y]
            }
        }
        self.state = new_state;
    }
    pub fn rho(&mut self) {
        let (mut x, mut y) = (1, 0);
        for t in 0..24 {
            self.state[x][y] = Lane(self.state[x][y]).rotate((t + 1) * (t + 2) / 2);
            let x_new = 1 * y;
            let y_new = 2 * x + 3 * y;
            (x, y) = (x_new % 5, y_new % 5)
        }
    }
    pub fn iota(&mut self, round: usize) {
        self.state[0][0] = Lane(self.state[0][0]) ^ Lane(self.get_rc_i(round));
    }
    pub fn get_rc_i(&mut self, round: usize) -> [bool; WIDTH] {
        let mut rc_i = [false; WIDTH];
        for j in 0..Self::L {
            rc_i[(2 as usize).pow(j as u32) - 1] = self.get_lfsr(j + 7 * round);
        }
        rc_i
    }
    pub fn get_lfsr(&mut self, t: usize) -> bool {
        for _ in 0..t {
            self.lfsr.inc();
        }
        let ret = (self.lfsr.get_state() & 1) == 1;
        self.reset_lfsr();
        ret
    }
    pub fn reset_lfsr(&mut self) {
        self.lfsr = Galois8::new(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        println!("Test 1:");
        let mut state: State<50> = State::new();
        state.permute();
        println!("{}", state);
    }
}
