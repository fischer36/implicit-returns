# Proof of concept: Implicit function return values

## Concept 
After learning that function return values are undefined within their signature in assembly language, I wanted to find out if it was possible to manually return a value. Normally, return values in compiled languages are managed by the compiler according to conventions; for instance, in x86_64 architecture, the *RAX* register is used to store return values. RAX is a 64-bit register, with its lower 32 bits referred to as *EAX*. As demonstrated in the disassembly of testing.rs, 64-bit values are moved into RAX, while 32-bit returns are moved into EAX.

## Proof
The experiment successfully demonstrates implicit 32-bit return values by manually setting and retrieving a value via the EAX register in a function that lacks an explicit return type in its Rust signature. This was achieved by using inline assembly to move values into EAX just before returning from the function, and then fetching those values after the function call.
