//二进制chunk
pub struct BinaryChunk {
    header: Header,
    size_upvalues: u8,
    main_func: Prototype,
}

//头信息
pub struct Header {
    signature: [u8; 4],
    version: u8,
    format: u8,
    luac_data: [u8; 6],
    c_int_size: u8,
    c_size_t_size: u8,
    instruction_size: u8,
    lua_integer_size: u8,
    lua_number_size: u8,
    luac_int: i64,
    luac_num: f64
}

const LUA_SIGNATURE: [u8; 4] = [0x1b, 0x4c, 0x75, 0x61];
const LUA_VERSION: u8 = 0x53;
const LUA_FORMAT: u8 = 0x00;
const LUAC_DATA: [u8; 6] = [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a];
const C_INT_SIZE: u8 = 4;
const C_SIZE_T_SIZE: u8 = 8;
const INSTRUCTION_SIZE: u8 = 4;
const LUA_INTEGER_SIZE: u8 = 8;
const LUA_NUMBER_SIZE: u8 = 8;
const LUAC_INT: i64 = 0x5678;
const LUAC_NUM: f64 = 370.5;

//函数原型
pub struct Prototype {
    pub source: String,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_params: u8,
    pub is_varargs: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,

}

//常量类型
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    String(String)
}
const TAG_NIL: u8 = 0x00;
const TAG_BOOLEAN: u8 = 0x01;
const TAG_NUMBER: u8 = 0x03;
const TAG_INTEGER: u8 = 0x13;
const TAG_SHORT_STR: u8 = 0x04;
const TAG_LONG_STR: u8 = 0x14;

//Upvalue表
pub struct Upvalue {
    pub in_stack: u8,
    pub idx: u8
}

//局部变量表
pub struct LocVar{
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32
}

pub struct Reader {
    data: Vec<u8>,
    pos: usize
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Reader {
        Reader { data,  pos: 0 }
    }

    pub fn read_byte(&mut self) -> u8 {
        let b =  self.data[self.pos];
        self.pos += 1;
        b
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut u: u32 = 0;
        for i in 0..4 {
            u = u | ((self.read_byte() as u32) << (i * 8));
        }
        u
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut u: u64 = 0;
        for i in 0..8 {
            u = u | ((self.read_byte() as u64) << (i * 8));
        }
        u
    }

    pub fn read_lua_integer(&mut self) -> i64 {
        self.read_u64() as i64
    }

    pub fn read_lua_number(&mut self) -> f64 {
        f64::from_bits(self.read_u64())
    }
}