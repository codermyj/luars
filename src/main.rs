mod state;
mod binarychunk;
mod api;

use api::lua_state::{LuaType, TraitLuaState};
use state::lua_state::LuaState;
use api::consts::*;


fn main() {
    let mut ls = LuaState::new();
    ls.push_boolean(true); print_stack(&mut ls);
    ls.push_integer(10);    print_stack(&mut ls);
    ls.push_nil();  print_stack(&mut ls);
    ls.push_string(String::from("hello")); print_stack(&mut ls);
    ls.push_value(-4); print_stack(&mut ls); //println!("{}", ls.stack.top);
    ls.replace(3); print_stack(&mut ls); //println!("{}", ls.stack.top);
    ls.set_top(6); print_stack(&mut ls); //println!("{}", ls.stack.top);
    ls.remove(-3); print_stack(&mut ls);
    ls.set_top(-5); print_stack(&mut ls);
}

fn print_stack(ls: &mut LuaState) {
    let top = ls.get_top();
   // println!("{:?}", ls.stack.slots);
    for i in 1..=top {
        let t = ls.get_type(i);
        match t {
            LuaType(LUA_TBOOLEAN) => {
                print!("[{}]", ls.to_boolean(i));
            },
            LuaType(LUA_TNUMBER) => {
                print!("[{}]", ls.to_number(i));
            }
            LuaType(LUA_TSTRING) => {
                print!("[{}]", ls.to_string(i));
            }
            _ => {
                print!("[{}]", ls.type_name(t));
            }
        }
    }
    println!();
}