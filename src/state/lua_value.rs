use crate::api::consts::*;

#[derive(Copy, Clone)]
pub enum LuaValue {
    Nil,
    Bool(bool),
    Integer(i64),
    Number(f64),
    String(String),
}

impl LuaValue {
    pub fn type_of(&self) -> u8 {
        match self {
            &LuaValue::Nil => LUA_TNIL,
            &LuaValue::Bool(b) => LUA_TBOOLEAN,
            &LuaValue::Integer(i) => LUA_TNUMBER,
            &LuaValue::Number(f) => LUA_TNUMBER,
            &LuaValue::String(s) => LUA_TSTRING,
        }
    }
}