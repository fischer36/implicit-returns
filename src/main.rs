fn main() {
    // Defines what 32-bit value to return
    let value_to_return: u32 = 43;
    unsafe {
        // Function that puts param into 32-bit return register (eax)
        return_this(value_to_return);
    }

    let returned_value: u32;
    unsafe {
        std::arch::asm!(
            "mov {var:e}, eax", // move value in eax into returned_value, ':e' specifies that value is 32-bit
            var = out(reg) returned_value, // provide Rust "return_value" variable to to asm block
        );
    }

    assert_eq!(returned_value, value_to_return);
    println!("Fetched value: {}", returned_value);
}

unsafe fn return_this(return_val: u32) {
    std::arch::asm!(
        "mov eax, {value:e}",
        value = in(reg) return_val,
        options(nostack) // nostack option to prevent eax from being overwritten?
    );
}
