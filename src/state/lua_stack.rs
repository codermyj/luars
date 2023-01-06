use crate::state::lua_value::LuaValue;

pub struct LuaStack {
    slots: Vec<LuaValue>,
    top: i32
}

impl LuaStack {
    fn new(size: i32) -> LuaStack {
        LuaStack {
            slots: Vec::with_capacity(size as usize),
            top: 0,
        }
    }

    fn check(&mut self, n: i32) {
        let free = self.slots.len() as i32 - self.top;
        for i in free..n {
            self.slots.push(LuaValue::Nil);
        }
    }

    fn push(&mut self, val: LuaValue) {
        assert!(self.top < self.slots.len() as i32, "stack overflow!");
        self.slots.push(val);
        self.top += 1;
    }

    fn pop(&mut self) -> LuaValue {
        assert!(self.top >= 1, "stack underflow!");
        let pop_value = self.slots.pop()?;
        self.top -= 1;
        pop_value
    }

    fn abs_index(&self, idx: i32) -> i32 {
        if idx > 0 {
            idx
        }else {
            self.top + idx + 1
        }
    }

    fn is_valid(&self, idx: i32) -> bool {
        let abs_idx = self.abs_index(idx);
        (abs_idx <= self.top) && (abs_idx > 0)
    }

    fn get(&self, idx: i32) -> LuaValue {
        if self.is_valid(idx) {
            self.slots[idx as usize - 1].clone()
        }else {
            LuaValue::Nil
        }
    }

    fn set(&mut self, idx: i32, val: LuaValue) {
        if self.is_valid(idx) {
            self.slots[idx - 1] = val;
            return;
        }
        panic!("invalid index");
    }
}