pub const OP_MOVE: u8 = 0x00;
pub const OP_LOADK: u8 = 0x01;
pub const OP_LOADKX: u8 = 0x02;
pub const OP_LOADBOOL: u8 = 0x03;
pub const OP_LOADNIL: u8 = 0x04;
pub const OP_GETUPVAL: u8 = 0x05;
pub const OP_GETTABUP: u8 = 0x06;
pub const OP_GETTABLE: u8 = 0x07;
pub const OP_SETTABUP: u8 = 0x08;
pub const OP_SETUPVAL: u8 = 0x09;
pub const OP_SETTABLE: u8 = 0x0a;
pub const OP_NEWTABLE: u8 = 0x0b;
pub const OP_SELF: u8 = 0x0c;
pub const OP_ADD: u8 = 0x0d;
pub const OP_SUB: u8 = 0x0e;
pub const OP_MUL: u8 = 0x0f;
pub const OP_MOD: u8 = 0x10;
pub const OP_POW: u8 = 0x11;
pub const OP_DIV: u8 = 0x12;
pub const OP_IDIV: u8 = 0x13;
pub const OP_BAND: u8 = 0x14;
pub const OP_BOR: u8 = 0x15;
pub const OP_BXOR: u8 = 0x16;
pub const OP_SHL: u8 = 0x17;
pub const OP_SHR: u8 = 0x18;
pub const OP_UNM: u8 = 0x19;
pub const OP_BNOT: u8 = 0x1a;
pub const OP_NOT: u8 = 0x1b;
pub const OP_LEN: u8 = 0x1c;
pub const OP_CONCAT: u8 = 0x1d;
pub const OP_JMP: u8 = 0x1e;
pub const OP_EQ: u8 = 0x1f;
pub const OP_LT: u8 = 0x20;
pub const OP_LE: u8 = 0x21;
pub const OP_TEST: u8 = 0x22;
pub const OP_TESTSET: u8 = 0x23;
pub const OP_CALL: u8 = 0x24;
pub const OP_TAILCALL: u8 = 0x25;
pub const OP_RETURN: u8 = 0x26;
pub const OP_FORLOOP: u8 = 0x27;
pub const OP_FORPREP: u8 = 0x28;
pub const OP_TFORCALL: u8 = 0x29;
pub const OP_TFORLOOP: u8 = 0x2a;
pub const OP_SETLIST: u8 = 0x2b;
pub const OP_CLOSURE: u8 = 0x2c;
pub const OP_VARARG: u8 = 0x2d;
pub const OP_EXTRAARG: u8 = 0x2e;

pub const OP_ARG_N: u8 = 0x00;
pub const OP_ARG_U: u8 = 0x01;
pub const OP_ARG_R: u8 = 0x03;
pub const OP_ARG_K: u8 = 0x04;


pub struct OpCode {

}