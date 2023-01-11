use crate::api::lua_state::LuaState;
use crate::state::lua_value::LuaValue;
use crate::state::lua_value::LuaValue::Nil;

pub struct LuaStack {
    pub slots: Vec<LuaValue>,
    pub top: i32
}

impl LuaStack {
    pub fn new(size: i32) -> LuaStack {
        let mut v = Vec::with_capacity(size as usize);
        v.fill(Nil);
        LuaStack {
            slots: v,
            top: 0,
        }
    }

   pub fn check(&mut self, n: i32) {
        let free = self.slots.len() as i32 - self.top;
        for i in free..n {
            self.slots.push(LuaValue::Nil);
        }
    }

    pub fn push(&mut self, val: LuaValue) {
        assert!(self.top < self.slots.len() as i32, "stack overflow!");
        self.slots.push(val);
        self.top += 1;
    }

    pub fn pop(&mut self) -> LuaValue {
        assert!(self.top >= 1, "stack underflow!");
        let pop_value = self.slots.pop()?;
        self.top -= 1;
        pop_value
    }

    pub fn abs_index(&self, idx: i32) -> i32 {
        if idx > 0 {
            idx
        }else {
            self.top + idx + 1
        }
    }

    pub fn is_valid(&self, idx: i32) -> bool {
        let abs_idx = self.abs_index(idx);
        (abs_idx <= self.top) && (abs_idx > 0)
    }

    pub fn get(&self, idx: i32) -> LuaValue {
        if self.is_valid(idx) {
            self.slots[idx as usize - 1].clone()
        }else {
            LuaValue::Nil
        }
    }

    pub fn set(&mut self, idx: i32, val: LuaValue) {
        if self.is_valid(idx) {
            self.slots[idx - 1] = val;
            return;
        }
        panic!("invalid index");
    }

    pub fn reverse(&mut self, mut from: i32, mut to: i32) {
        //let from = from as usize;
        while from < to {
            (self.slots[from], self.slots[to]) = (self.slots[to], self.slots[from])
            from += 1;
            to -= 1;
        }
    }
}