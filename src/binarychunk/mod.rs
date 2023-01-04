//二进制chunk
pub struct BinaryChunk {
    header: Header,         // 头部
    size_upvalues: u8,      // 主函数upvalue数量
    main_func: Prototype,   // 主函数原型
}

//头部信息
pub struct Header {
    signature: [u8; 4],     // 签名，一个魔数，值为常量，LUA_SIGNATURE，代表esc、L、u、a
    version: u8,            // 版本号
    format: u8,             // 格式号
    luac_data: [u8; 6],     // 1.0发布年份、回车、换行、替换符、换行
    c_int_size: u8,         // c int类型占用字节数
    c_size_t_size: u8,      // size_t类型占用字节数
    instruction_size: u8,   // Lua虚拟机指令占用字节数
    lua_integer_size: u8,   // Lua整数占用字节数
    lua_number_size: u8,    // Lua浮点数占用字节数
    luac_int: i64,          // 整数0x5678，用于检测大小端方式与本机是否匹配
    luac_num: f64           // 浮点数370.5，用于检测浮点数格式与本机是否匹配
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
    pub source: String,             // 源文件名
    pub line_defined: u32,          // 函数起始行号
    pub last_line_defined: u32,     // 函数结束行号
    pub num_params: u8,             // 固定参数个数
    pub is_varargs: u8,             // 函数是否为varargs，即是否有变长参数
    pub max_stack_size: u8,         // 寄存器数量
    pub code: Vec<u32>,             // 指令表，每条指令占4个字节
    pub constants: Vec<Constant>,   //
    pub upvalues: Vec<Upvalue>,
    pub protos: Vec<Prototype>,
    pub line_info: Vec<u32>,
    pub loc_vars: Vec<LocVar>,
    pub upvalue_names: Vec<String>,
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

pub fn undump(data: Vec<u8>) -> Prototype {
    let mut reader = Reader::new(data);
    reader.check_header();
    reader.read_byte();
    reader.read_proto(String::from(""))
}

pub struct Reader {
    data: Vec<u8>,
    pos: usize
}



impl Reader {
    pub fn new(data: Vec<u8>) -> Reader {
        Reader { data,  pos: 0 }
    }

    // 读取一个字节
    pub fn read_byte(&mut self) -> u8 {
        let b =  self.data[self.pos];
        self.pos += 1;
        b
    }

    // 小端法读取一个cint类型数据
    pub fn read_u32(&mut self) -> u32 {
        let mut u: u32 = 0;
        for i in 0..4 {
            u = u | ((self.read_byte() as u32) << (i * 8));
        }
        u
    }

    // 小端法读取一个size_t类型数据
    pub fn read_u64(&mut self) -> u64 {
        let mut u: u64 = 0;
        for i in 0..8 {
            u = u | ((self.read_byte() as u64) << (i * 8));
        }
        u
    }

    // 小端法读取Lua整数类型
    pub fn read_lua_integer(&mut self) -> i64 {
        self.read_u64() as i64
    }

    // 小端法读取Lua浮点数类型
    pub fn read_lua_number(&mut self) -> f64 {
        f64::from_bits(self.read_u64())
    }

    // 从字节流读取字符串
    pub fn read_string(&mut self) -> String {
        let mut size = self.read_byte() as usize;
        if size == 0 {
            return String::from("");
        }
        if size == 0xFF {
            size = self.read_u64() as usize;
        }

        match String::from_utf8(self.read_bytes(size - 1)) {
            Ok(str) => str,
            Err(e) => format!("Err: {}", e.to_string())
        }
    }

    // 从字节流读取n个字节
    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(n);
        for _ in 0..n {
            bytes.push(self.read_byte())
        }
        bytes
    }

    // 检查Header
    pub fn check_header(&mut self) {
        assert_eq!(self.read_bytes(4), LUA_SIGNATURE, "not a compiled chunk!");
        assert_eq!(self.read_byte(), LUA_VERSION, "version mismatch!");
        assert_eq!(self.read_byte(), LUA_FORMAT, "format mismatch!");
        assert_eq!(self.read_bytes(6), LUAC_DATA, "corrupted!");
        assert_eq!(self.read_byte(), C_INT_SIZE, "int size mismatch!");
        assert_eq!(self.read_byte(), C_SIZE_T_SIZE, "size_t size mismatch!");
        assert_eq!(self.read_byte(), INSTRUCTION_SIZE, "instruction size mismatch!");
        assert_eq!(self.read_byte(), LUA_INTEGER_SIZE, "lua integer size mismatch!");
        assert_eq!(self.read_byte(), LUA_NUMBER_SIZE, "lua number size mismatch!");
        assert_eq!(self.read_lua_integer(), LUAC_INT, "endianness mismatch!");
        assert_eq!(self.read_lua_number(), LUAC_NUM, "float format mismatch!")
    }

    pub fn read_proto(&mut self, parent_source: String) -> Prototype {
        let mut source = self.read_string();
        if source == "" {
            source = parent_source;
        }
        Prototype {
            source: source.clone(),
            line_defined: self.read_u32(),
            last_line_defined: self.read_u32(),
            num_params: self.read_byte(),
            is_varargs: self.read_byte(),
            max_stack_size: self.read_byte(),
            code: self.read_code(),
            constants: self.read_constants(),
            upvalues: self.read_upvalues(),
            protos: self.read_protos(source.clone()),
            line_info: self.read_line_info(),
            loc_vars: self.read_loc_vars(),
            upvalue_names: self.read_upvalue_names(),
        }
    }

    pub fn read_code(&mut self) -> Vec<u32> {
        let code_size = self.read_u32() as usize;
        let mut code: Vec<u32> = Vec::with_capacity(code_size);
        for _ in 0..code_size {
            code.push(self.read_u32());
        }
        code
    }

    pub fn read_constant(&mut self) -> Constant {
        match self.read_byte() {
            TAG_NIL => Constant::Nil,
            TAG_BOOLEAN => Constant::Boolean(self.read_byte() != 0),
            TAG_INTEGER => Constant::Integer(self.read_lua_integer()),
            TAG_NUMBER => Constant::Number(self.read_lua_number()),
            TAG_SHORT_STR => Constant::String(self.read_string()),
            TAG_LONG_STR => Constant::String(self.read_string()),
            _ => panic!("corrupted!")
        }
    }

    pub fn read_constants(&mut self) -> Vec<Constant> {
        let constants_size = self.read_u32() as usize;
        let mut constants: Vec<Constant> = Vec::with_capacity(constants_size);
        for _ in 0..constants_size {
            constants.push(self.read_constant());
        }
        constants
    }

    pub fn read_upvalues(&mut self) -> Vec<Upvalue> {
        let upvalues_size = self.read_u32() as usize;
        let mut upvalues: Vec<Upvalue> = Vec::with_capacity(upvalues_size);
        for _ in 0..upvalues_size {
            upvalues.push(
                Upvalue {
                    in_stack: self.read_byte(),
                    idx: self.read_byte(),
                }
            )
        }
        upvalues
    }

    pub fn read_protos(&mut self, parent_source: String) -> Vec<Prototype> {
        let size = self.read_u32() as usize;
        let mut protos: Vec<Prototype> = Vec::with_capacity(size);
        for _ in 0..size {
            protos.push(self.read_proto(parent_source.clone()));
        }
        protos
    }

    pub fn read_line_info(&mut self) -> Vec<u32> {
        let size = self.read_u32() as usize;
        let mut line_info: Vec<u32> = Vec::with_capacity(size);
        for _ in 0..size {
            line_info.push(self.read_u32());
        }
        line_info
    }

    pub fn read_loc_vars(&mut self) -> Vec<LocVar> {
        let size = self.read_u32() as usize;
        let mut loc_vars: Vec<LocVar> = Vec::with_capacity(size);
        for _ in 0..size {
            loc_vars.push(
                LocVar {
                    var_name: self.read_string(),
                    start_pc: self.read_u32(),
                    end_pc: self.read_u32(),
                }
            )
        }
        loc_vars
    }

    pub fn read_upvalue_names(&mut self) -> Vec<String> {
        let size = self.read_u32() as usize;
        let mut upvalue_names: Vec<String> = Vec::with_capacity(size);
        for _ in 0..size {
            upvalue_names.push(self.read_string());
        }
        upvalue_names
    }
}