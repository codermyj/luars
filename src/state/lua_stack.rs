use crate::state::lua_value::LuaValue;
use crate::state::lua_value::LuaValue::Nil;

pub struct LuaStack {
    pub slots: Vec<LuaValue>,
    pub top: i32
}

impl LuaStack {
    pub fn new(size: i32) -> LuaStack {
        let mut v = vec![Nil; size as usize];
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
        self.slots[self.top as usize] = val;
        self.top += 1;
    }

    pub fn pop(&mut self) -> LuaValue {
        assert!(self.top >= 1, "stack underflow!");
        self.top -= 1;
        let val = self.slots[self.top as usize].clone();
        self.slots[self.top as usize] = Nil;
        val
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
        (abs_idx <= self.top as i32) && (abs_idx > 0)
    }

    pub fn get(&self, idx: i32) -> LuaValue {
        let abs_idx = self.abs_index(idx);
        if self.is_valid(abs_idx) {
            self.slots[abs_idx as usize - 1].clone()
        }else {
            LuaValue::Nil
        }
    }

    pub fn set(&mut self, idx: i32, val: LuaValue) {
        let abs_idx = self.abs_index(idx);
        if self.is_valid(abs_idx) {
            self.slots[abs_idx as usize - 1] = val;
            return;
        }
        panic!("invalid index");
    }

    pub fn reverse(&mut self, mut from: i32, mut to: i32) {
        //let from = from as usize;
        while from < to {
            (self.slots[from as usize], self.slots[to as usize]) = (self.slots[to as usize].clone(), self.slots[from as usize].clone());
            from += 1;
            to -= 1;
        }
    }
}