use crate::binarychunk::Constant::Nil;
use crate::state::lua_stack::LuaStack;

pub struct LuaState{
   pub stack: LuaStack
}

impl LuaState {
    fn new() -> LuaState {
        LuaState {
            stack: LuaStack::new(20)
        }
    }
}