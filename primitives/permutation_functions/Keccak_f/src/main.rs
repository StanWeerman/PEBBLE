fn main() {
    println!("TesT");
    let state: State<50> = State::new();
}

pub struct State<const WIDTH: usize> {
    state: [[[bool; WIDTH]; 5]; 5],
}
impl<const WIDTH: usize> State<WIDTH> {
    pub fn new() -> Self {
        State {
            state: [[[false; WIDTH]; 5]; 5],
        }
    }
    pub fn chi(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
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
