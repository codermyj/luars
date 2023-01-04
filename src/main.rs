use std::{env, fs};
use crate::binarychunk::Constant;

mod binarychunk;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match fs::read(&args[1]) {
            Ok(data) => {
                let proto = binarychunk::undump(data);
                list(&proto);

            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

/*
/ Todo
 */
fn list(f: &binarychunk::Prototype) {
    print_header(f);
    print_code(f);
    print_detail(f);
    for p in &f.protos {
        list(p);
    }
}

fn print_header(f: &binarychunk::Prototype) {
    let mut fn_type = "main";
    if f.line_defined > 0 {
        fn_type = "function";
    }
    let mut vararg_flag = "";
    if f.is_varargs > 0 {
        vararg_flag = "+";
    }
    println!("\n{} <{}:{},{}> ({} instructions)",
             fn_type, f.source, f.line_defined, f.last_line_defined, f.code.len());

    print!("{}{} params, {} slots, {} upvalues, ", f.num_params, vararg_flag, f.max_stack_size, f.upvalues.len());
    println!("{} locals, {} constants, {} functions", f.loc_vars.len(), f.constants.len(), f.protos.len());
}

fn print_code(f: &binarychunk::Prototype) {
    let mut pc = 0;
    for c in &f.code {
        let mut line = String::from("-");
        if f.line_info.len() > 0 {
            line = format!("{}", f.line_info[pc]);
        }
        pc += 1;
        println!("\t{}\t[{}]\t0x{:0>8x}", pc, line, c);
    }
}

//打印常量表、局部变量表和Upvalue表
fn print_detail(f: &binarychunk::Prototype) {
    println!("constants ({})", f.constants.len());
    for (index, el) in f.constants.iter().enumerate() {
        println!("\t{}\t{}", index+1, constant_to_string(el));
    }
}

fn constant_to_string(constant: &Constant) -> String {
    match constant {
        Constant::Nil => format!("nil"),
        Constant::Boolean(b) => format!("{}", b),
        Constant::Number(n) => format!("{}", n),
        Constant::Integer(i) => format!("{}", i),
        Constant::String(s) => format!("{}", s),
        //_ => format!("?")
    }
}

fn upvalue_name(f: &binarychunk::Prototype, idx: usize) -> String {
    if f.upvalue_names.len() > 0 {
        match f.upvalue_names.get(idx) {
            Some(s) => {
                return String::from(s);
            }
            None => {
                return String::from("-");
            }
        }
    }
    String::from("-")
}