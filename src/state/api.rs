use std::process::id;
use crate::api;
use crate::api::lua_state::LuaType;
use crate::state::lua_value::LuaValue;
use super::lua_state::LuaState;
use crate::api::consts::*;

impl api::lua_state::LuaState for LuaState {
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
        if n > 0 {
            for i in 0..n {
                self.stack.pop();
            }
        }else if n < 0 {
            self.stack.push(LuaValue::Nil);
        }
    }

    fn type_name(&self, tp: LuaType) -> String {
        let str = match tp {
            LuaType(LUA_TNONE) => "no value",
            LuaType
        }
    }

    fn get_type(&self, idx: i32) -> LuaType {
        todo!()
    }

    fn is_none(&self, idx: i32) -> bool {
        todo!()
    }

    fn is_nil(&self, idx: i32) -> bool {
        todo!()
    }

    fn is_none_or_nil(&self, idx: i32) -> bool {
        todo!()
    }

    fn is_boolean(&self, idx: i32) -> bool {
        todo!()
    }

    fn is_integer(&self, idx: i32) -> bool {
        todo!()
    }

    fn is_number(&self, idx: i32) -> bool {
        todo!()
    }

    fn is_string(&self, idx: i32) -> bool {
        todo!()
    }

    fn to_boolean(&self, idx: i32) -> bool {
        todo!()
    }

    fn to_integer(&self, idx: i32) -> i64 {
        todo!()
    }

    fn to_integer_x(&self, idx: i32) -> (i64, bool) {
        todo!()
    }

    fn to_number(&self, _: i32) -> f64 {
        todo!()
    }

    fn to_number_x(&self, _: i32) -> (f64, bool) {
        todo!()
    }

    fn to_string(&self, _: i32) -> String {
        todo!()
    }

    fn to_string_x(&self, _: i32) -> (String, bool) {
        todo!()
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