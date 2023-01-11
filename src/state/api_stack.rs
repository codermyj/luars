use std::process::id;
use crate::api;
use crate::api::lua_state::LuaType;
use super::lua_state::LuaState;

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
        todo!()
    }

    fn set_top(&mut self, idx: i32) {
        todo!()
    }

    fn type_name(&self, tp: LuaType) -> String {
        todo!()
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
        todo!()
    }

    fn push_boolean(b: bool) {
        todo!()
    }

    fn push_integer(i: i64) {
        todo!()
    }

    fn push_number(n: f64) {
        todo!()
    }

    fn push_string(s: String) {
        todo!()
    }
}