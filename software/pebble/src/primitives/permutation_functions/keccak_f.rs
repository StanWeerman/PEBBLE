use std::fmt::Display;

pub struct State<const WIDTH: usize> {
    state: [[[bool; WIDTH]; 5]; 5],
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
    pub fn new() -> Self {
        State {
            state: [[[false; WIDTH]; 5]; 5],
        }
    }
    pub fn chi(&mut self) {
        for y in 0..5 {
            for x in 0..5 {
                self.state[x][y] = not(self.state[x][y]);
            }
        }
    }
    pub fn theta(&mut self) {}
    pub fn pi(&mut self) {}
    pub fn rho(&mut self) {}
    pub fn iota(&mut self) {}
}

pub fn not<const WIDTH: usize>(lane: [bool; WIDTH]) -> [bool; WIDTH] {
    let mut a = [false; WIDTH];
    for i in 0..WIDTH {
        a[i] = !lane[i]
    }
    return a;
}

pub fn and<const WIDTH: usize>(lane_a: [bool; WIDTH], lane_b: [bool; WIDTH]) -> [bool; WIDTH] {
    let mut a = [false; WIDTH];
    for i in 0..WIDTH {}
    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        println!("Test 1:");
        let mut state: State<50> = State::new();
        state.chi();
        println!("{}", state);
    }
}
