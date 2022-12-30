pub struct BinaryChunk {
    header: Header,
    size_upvalues: u8,
    main_func: Prototype,
}

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