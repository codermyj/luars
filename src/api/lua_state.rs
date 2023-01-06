pub struct LuaType(i32);

pub trait LuaState {
    fn get_top() -> i32;
    fn abs_index(idx: i32) -> i32;
    fn check_stack(n: i32) -> bool;
}