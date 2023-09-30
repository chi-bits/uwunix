#[derive(Debug)]
pub enum instr {
    // memory shit
    ld,
    st,
    write,
    read,
    
    // math 
    add,
    sub,
    mul,
    div,
    inc,
    dec,

    // logic
    and,
    or,
    xor,
    not,


    // function shit
    func,
    call,
    jmp,

    cmp,
    halt,
}
