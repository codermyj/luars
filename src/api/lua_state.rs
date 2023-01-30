#[derive(PartialEq)]
pub struct LuaType(pub i8);


pub trait TraitLuaState {
    /// 基础栈操作
    fn get_top(&self) -> i32;
    fn abs_index(&self, idx: i32) -> i32;
    fn check_stack(&mut self, n: i32) -> bool;
    fn pop(&mut self, n: i32);
    fn copy(&mut self, from_idx: i32, to_idx: i32);
    fn push_value(&mut self, idx: i32);
    fn replace(&mut self, idx: i32);
    fn insert(&mut self, idx: i32);
    fn remove(&mut self, idx: i32);
    fn rotate(&mut self, idx: i32, n: i32);
    fn set_top(&mut self, idx: i32);
    /// 读取数据函数
    fn type_name(&self, tp: LuaType) -> String;
    fn get_type(&self, idx: i32) -> LuaType;
    fn is_none(&self, idx: i32) -> bool;
    fn is_nil(&self, idx: i32) -> bool;
    fn is_none_or_nil(&self, idx: i32) -> bool;
    fn is_boolean(&self, idx: i32) -> bool;
    fn is_integer(&self, idx: i32) -> bool;
    fn is_number(&self, idx: i32) -> bool;
    fn is_string(&self, idx: i32) -> bool;
    fn to_boolean(&self, idx: i32) -> bool;
    fn to_integer(&self, idx: i32) -> i64;
    fn to_integer_x(&self, idx: i32) -> (i64, bool);
    fn to_number(&self, idx: i32) -> f64;
    fn to_number_x(&self, idx: i32) -> (f64, bool);
    fn to_string(&mut self, idx: i32) -> String;
    fn to_string_x(&mut self, idx: i32) -> (String, bool);
    /// push函数
    fn push_nil(&mut self);
    fn push_boolean(&mut self, b: bool);
    fn push_integer(&mut self,i: i64);
    fn push_number(&mut self,n: f64);
    fn push_string(&mut self,s: String);
}