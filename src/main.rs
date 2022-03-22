const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
struct Chip8 {
    registers: [u8; 16],
    i_reg: u16,
    st: u8,
    dt: u8,
    memory: [u8; 4096],
    display: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    stack: Vec<u16>,
    sp: u8,
}

impl Chip8 {
    fn new() -> Self {
        Chip8 {
            registers: [0; 16],
            i_reg: 0,
            st: 0,
            dt: 0,
            memory: [0; 4096],
            display: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            stack: vec![],
            sp: 0,
        }
    }
}
fn main() {
    println!("Chip 8 emulator");
}
