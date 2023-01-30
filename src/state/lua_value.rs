use crate::api::consts::*;

#[derive(Clone, Debug)]
pub enum LuaValue {
    Nil,
    Bool(bool),
    Integer(i64),
    Number(f64),
    String(String),
}

impl LuaValue {
    pub fn type_of(&self) -> i8 {
        match self {
            &LuaValue::Nil => LUA_TNIL,
            &LuaValue::Bool(_) => LUA_TBOOLEAN,
            &LuaValue::Integer(_) => LUA_TNUMBER,
            &LuaValue::Number(_) => LUA_TNUMBER,
            &LuaValue::String(_) => LUA_TSTRING,
        }
    }
}