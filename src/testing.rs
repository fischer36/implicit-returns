fn main() {
    let x = thirty_two_bits();
    let y = sixty_four_bits();
}

// Viewing this function in dissasembly shows it returning via the eax register
#[no_mangle]
fn thirty_two_bits() -> u32 {
    let value: u32 = std::u32::MAX;
    value
}

// Viewing this function in dissasembly shows it returning via the rax register
#[no_mangle]
fn sixty_four_bits() -> u64 {
    let value: u64 = std::u64::MAX;
    value
}
