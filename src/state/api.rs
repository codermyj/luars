use crate::api;
use crate::api::lua_state::LuaType;
use crate::state::lua_value::LuaValue;
use super::lua_state::LuaState;
use crate::api::consts::*;

impl api::lua_state::TraitLuaState for LuaState {
    fn get_top(&self) -> i32 {
        self.stack.top
    }

    fn abs_index(&self, idx: i32) -> i32 {
        self.stack.abs_index(idx)
    }

    fn check_stack(&mut self, n: i32) -> bool {
        self.stack.check(n);
        true
    }

    ///从栈顶弹出n个值
    fn pop(&mut self, n: i32) {
        for _ in 0..n {
            self.stack.pop();
        }
    }

    fn copy(&mut self, from_idx: i32, to_idx: i32) {
        let val = self.stack.get(from_idx);
        self.stack.set(to_idx, val);
    }


    fn push_value(&mut self, idx: i32) {
        let val = self.stack.get(idx);
        self.stack.push(val);
    }

    ///将栈顶值弹出，写入指定位置
    fn replace(&mut self, idx: i32) {
        let val = self.stack.pop();
        self.stack.set(idx, val);
    }

    ///将栈顶值弹出，插入指定位置，旋转操作rotate的特例
    fn insert(&mut self, idx: i32) {
        self.rotate(idx, 1);
    }

    fn remove(&mut self, idx: i32) {
        self.rotate(idx, -1);
        self.pop(1);
    }

    fn rotate(&mut self, idx: i32, n: i32) {
        let start = self.abs_index(idx) - 1;
        let top = self.stack.top - 1;
        let length = top - start + 1;
        let offset = n.abs() % length;

        let m;
        if n > 0 {
            m = offset - 1;
        }else {
            m = length - offset;
        }

        self.stack.reverse(start, top);
        self.stack.reverse(start, start + m - 1);
        self.stack.reverse(start + m, top);
    }

    fn set_top(&mut self, idx: i32) {
        let new_top = self.abs_index(idx);

        let n = self.stack.top - new_top;
        //println!("相差{}", n);
        if n > 0 {
            for _ in 0..n {
                self.stack.pop();
            }
        }else if n < 0 {
            for _ in 0..-n {
                self.stack.push(LuaValue::Nil);
            }

        }
    }

    fn type_name(&self, tp: LuaType) -> String {
        let str = match tp {
            LuaType(LUA_TNONE) => "no value",
            LuaType(LUA_TNIL) => "nil",
            LuaType(LUA_TBOOLEAN) => "bool",
            LuaType(LUA_TNUMBER) => "number",
            LuaType(LUA_TSTRING) => "string",
            LuaType(LUA_TTABLE) => "table",
            LuaType(LUA_TFUNCTION) => "function",
            LuaType(LUA_TTHREAD) => "thread",
            _ => "userdata",
        };
        str.to_string()
    }

    fn get_type(&self, idx: i32) -> LuaType {
        if self.stack.is_valid(idx) {
            let val = self.stack.get(idx);
            return LuaType(val.type_of());
        }
        return LuaType(LUA_TNONE);
    }

    fn is_none(&self, idx: i32) -> bool {
        self.get_type(idx) == LuaType(LUA_TNONE)
    }

    fn is_nil(&self, idx: i32) -> bool {
        self.get_type(idx) == LuaType(LUA_TNIL)
    }

    fn is_none_or_nil(&self, idx: i32) -> bool {
        self.get_type(idx).0 <= LUA_TNIL
    }

    fn is_boolean(&self, idx: i32) -> bool {
        self.get_type(idx) == LuaType(LUA_TBOOLEAN)
    }

    fn is_integer(&self, idx: i32) -> bool {
        let val = self.stack.get(idx);

        if let LuaValue::Integer(i) = val {
            return true
        }

        false
    }

    fn is_number(&self, idx: i32) -> bool {
        self.to_number_x(idx).1
    }

    fn is_string(&self, idx: i32) -> bool {
        let t = self.get_type(idx);
        (t == LuaType(LUA_TSTRING)) || (t == LuaType(LUA_TNUMBER))
    }

    fn to_boolean(&self, idx: i32) -> bool {
        let val = self.stack.get(idx);
        convert_to_boolean(val)
    }

    fn to_integer(&self, idx: i32) -> i64 {
        let (val, ok) = self.to_integer_x(idx);
        return val
    }

    /// 暂时只进行类型判断，后续补充
    fn to_integer_x(&self, idx: i32) -> (i64, bool) {
        let val = self.stack.get(idx);
        if let LuaValue::Integer(i) = val {
            return (i, true)
        }

        (0, false)
    }

    fn to_number(&self, idx: i32) -> f64 {
        self.to_number_x(idx).0
    }

    fn to_number_x(&self, idx: i32) -> (f64, bool) {
        let val = self.stack.get(idx);
        match val {
            LuaValue::Number(x) => (x, true),
            LuaValue::Integer(x) => (x as f64, true),
            _ => (0.0, false)
        }
    }

    fn to_string(&mut self, idx: i32) -> String {
        let (s, ok) = self.to_string_x(idx);
        s
    }

    fn to_string_x(&mut self, idx: i32) -> (String, bool) {
        let val = self.stack.get(idx);

        match val {
            LuaValue::String(v) => (v, true),
            LuaValue::Integer(v)  => {
                let s = format!("{}", v);
                self.stack.set(idx, LuaValue::String(s.clone()));
                (s, true)
            },
            LuaValue::Number(v) => {
                let s = format!("{}", v);
                self.stack.set(idx, LuaValue::String(s.clone()));
                (s, true)
            }
            _ => (String::from(""), false)
        }
    }

    fn push_nil(&mut self) {
        self.stack.push(LuaValue::Nil);
    }

    fn push_boolean(&mut self, b: bool) {
        self.stack.push(LuaValue::Bool(b));
    }

    fn push_integer(&mut self,i: i64) {
        self.stack.push(LuaValue::Integer(i));
    }

    fn push_number(&mut self,n: f64) {
        self.stack.push(LuaValue::Number(n));
    }

    fn push_string(&mut self,s: String) {
        self.stack.push(LuaValue::String(s));
    }
}

fn convert_to_boolean(val: LuaValue) -> bool {
    match val {
        LuaValue::Nil => false,
        LuaValue::Bool(b) => b,
        _ => true
    }
}