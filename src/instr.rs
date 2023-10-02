#[derive(Debug)]
pub enum instr {
    // memory shit
    ld,
    st,
    write,
    read,
    swap,
    
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

    endi,
    endfn,

    byte, // not an instruction
    // actually this enum is kinda misnamed but like
    // blehhhh!!!!!!!
}

pub struct Token {
    toktype: instr,
    val: u8,
}

pub struct InstrLex {
    out: Vec<Vec<Token>>,
}

impl InstrLex {
    pub fn lex(inp: &[u8]) {

    }
}


