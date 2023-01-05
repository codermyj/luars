// 编码模式
pub const IABC: u8 = 0x00;
pub const IABx: u8 = 0x01;
pub const IAsBx: u8 = 0x02;
pub const IAx: u8 = 0x03;

// 操作码
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

// 操作数类型
pub const OP_ARG_N: u8 = 0x00;
pub const OP_ARG_U: u8 = 0x01;
pub const OP_ARG_R: u8 = 0x03;
pub const OP_ARG_K: u8 = 0x04;


pub struct OpCode<'a> {
    pub test_flag: u8,
    pub set_A_flag: u8,
    pub arg_B_mode: u8,
    pub arg_C_mode: u8,
    pub op_mode: u8,
    pub name: &'a str,
}

impl<'a> OpCode<'a> {
    pub const fn new(test_flag: u8, set_A_flag: u8, arg_B_mode: u8, arg_C_mode: u8, op_mode: u8, name: &str) -> OpCode {
        OpCode {
            test_flag,
            set_A_flag, // 1表示将值存入寄存器A中
            arg_B_mode,
            arg_C_mode,
            op_mode,
            name
        }
    }
}


/// R(X)：第X个寄存器;
/// Kst(X)：第X个常量;
/// RK(X)：第X寄存器或者常量，是常量还是寄存器有X的最高位决定，最高位是1表示常量，否则表示寄存器;
/// UpValue(X)：第X个UpValue;
/// KPROTO(X)：第X个函数原型;
pub const OPCODES: &[OpCode] = &[
    /*          T  A      B        C      mode   name    */
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_N, IABC,  "MOVE    "), // 在寄存器间拷贝值。R(A) := R(B)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_N, IABx,  "LOADK   "), // 加载常量到寄存器。R(A) := Kst(B)
    OpCode::new(0, 1, OP_ARG_N, OP_ARG_N, IABx,  "LOADKX  "), // 加载常量到寄存器，常量索引从下一条OP_EXTRAARG指令获取。R(A) := Kst(extra arg)
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_U, IABC,  "LOADBOOL"), // 设置bool值B给R(A)，如果C为true，则跳过下一条指令。R(A) := (bool)B; if(C) pc++
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_N, IABC,  "LOADNIL "), // 加载nil给寄存器。R(A), R(A+1), ..., R(A+B) := nil
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_N, IABC,  "GETUPVAL"), // 取第B个UpValue给R(A)。R(A) := UpValue(B)
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_K, IABC,  "GETTABUP"), // 取出第B个UpValue表，再从中取出以RK(C)为Key的值。R(A) := UpValue(B)[RK(C)]
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_K, IABC,  "GETTABLE"), // R(A) := R(B)[RK(C)]
    OpCode::new(0, 0, OP_ARG_K, OP_ARG_K, IABC,  "SETTABUP"), // UpValue(A)[RK(B)] := RK(C)
    OpCode::new(0, 0, OP_ARG_U, OP_ARG_N, IABC,  "SETUPVAL"), // UpValue(B) := R(A)
    OpCode::new(0, 0, OP_ARG_K, OP_ARG_K, IABC,  "SETTABLE"), // R(A)[RK(B)] := RK(C)
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_U, IABC,  "NEWTABLE"), // R(A) := {}(size = B, C)
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_K, IABC,  "SELF    "), // R(A+1) := R(B), R(A) := R(B)[RK(C)]
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "ADD     "), // R(A) := RK(B) + RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "SUB     "), // R(A) := RK(B) - RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "MUL     "), // R(A) := RK(B) * RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "MOD     "), // R(A) := RK(B) % RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "POW     "), // R(A) := RK(B) ^ RK(C)  求幂
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "DIV     "), // R(A) := RK(B) / RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "IDIV    "), // R(A) := RK(B) // RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "BAND    "), // R(A) := RK(B) & RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "BOR     "), // R(A) := RK(B) | RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "BXOR    "), // R(A) := RK(B) ~ RK(C)  异或
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "SHL     "), // R(A) := RK(B) << RK(C)
    OpCode::new(0, 1, OP_ARG_K, OP_ARG_K, IABC,  "SHR     "), // R(A) := RK(B) >> RK(C)
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_N, IABC,  "UNUM    "), // R(A) := -R(B)
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_N, IABC,  "BNOT    "), // R(A) := ~R(B)
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_N, IABC,  "NOT     "), // R(A) := not R(B)
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_N, IABC,  "LEN     "), // R(A) := length of R(B)
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_R, IABC,  "CONCAT  "), // R(A) := R(B).. ... ..R(C)
    OpCode::new(0, 0, OP_ARG_R, OP_ARG_N, IAsBx, "JMP     "), // pc+=sBx; if(A) close all upvalues > R(A - 1)
    OpCode::new(1, 0, OP_ARG_K, OP_ARG_K, IABC,  "EQ      "), // if ((RK(B) == RK(C)) ~= A) then pc++
    OpCode::new(1, 0, OP_ARG_K, OP_ARG_K, IABC,  "LT      "), // if ((RK(B) < RK(C)) ~= A) then pc++
    OpCode::new(1, 0, OP_ARG_K, OP_ARG_K, IABC,  "LE      "), // if ((RK(B) <= RK(C)) ~= A) then pc++
    OpCode::new(1, 0, OP_ARG_N, OP_ARG_U, IABC,  "TEST    "), // if not (R(A) <=> C) then pc++
    OpCode::new(1, 1, OP_ARG_R, OP_ARG_U, IABC,  "TESTSET "), // if (R(B) <=> C) then R(A) := R(B) else pc++
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_U, IABC,  "CALL    "), // 函数调用。
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_U, IABC,  "TAILCALL"), // 函数尾调用。
    OpCode::new(0, 0, OP_ARG_U, OP_ARG_N, IABC,  "RETURN  "), // 返回语句。
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_N, IAsBx, "FORLOOP "),
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_N, IAsBx, "FORPREP "),
    OpCode::new(0, 0, OP_ARG_N, OP_ARG_U, IABC,  "TFORCALL"),
    OpCode::new(0, 1, OP_ARG_R, OP_ARG_N, IAsBx, "TFORLOOP"),
    OpCode::new(0, 0, OP_ARG_U, OP_ARG_U, IABC,  "SETLIST "),
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_N, IABx,  "CLOSURE "),
    OpCode::new(0, 1, OP_ARG_U, OP_ARG_N, IABC,  "VARARG  "),
    OpCode::new(0, 0, OP_ARG_U, OP_ARG_U, IAx,   "EXTRAARG"),
];