use std::{env, fs};
use crate::binarychunk::Constant;
use crate::vm::instructions::Instruction;
use vm::opcodes;

mod binarychunk;
mod vm;

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
/
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
    println!("{} <{}:{},{}> ({} instructions)",
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
        let i = &Instruction(*c);
        pc += 1;
        print!("\t{}\t[{}]\t{} \t", pc, line, i.op_name());
        print_operands(i);
        println!();
    }
}

fn print_operands(i: &Instruction) {
    match i.op_mode() {
        opcodes::IABC => {
            let (a, b, c) = i.abc();
            print!("{}", a);
            if i.b_mode() != opcodes::OP_ARG_N {
                if b > 0xFF {
                    print!(" {}", -1 - (b & 0xFF));
                }else {
                    print!(" {}", b);
                }
            }
            if i.c_mode() != opcodes::OP_ARG_N {
                if c > 0xFF {
                    //println!("调试信息：c>0xFF, c={}，c&0xFF={}", c, c & 0xFF);
                    print!(" {}", -1 - (c & 0xFF));
                }else {
                    print!(" {}", c);
                }
            }
        }
        opcodes::IABx => {
            let (a, bx) = i.a_bx();
            print!("{}", a);
            if i.b_mode() == opcodes::OP_ARG_K {
                print!(" {}", -1-bx);
            }else if i.b_mode() == opcodes::OP_ARG_N {
                print!(" {}", bx);
            }
        }
        opcodes::IAsBx => {
            let (a, sbx) = i.a_sbx();
            print!("{} {}", a, sbx);
        }
        opcodes::IAx => {
            let ax = i.ax();
            print!("{}", -1 - ax);
        }
        _ => {}
    }
}

//打印常量表、局部变量表和Upvalue表
fn print_detail(f: &binarychunk::Prototype) {
    println!("constants ({}):", f.constants.len());
    for (index, el) in f.constants.iter().enumerate() {
        println!("\t{}\t{}", index+1, constant_to_string(el));
    }

    println!("locals ({}):", f.loc_vars.len());
    for (index, loc_var) in f.loc_vars.iter().enumerate() {
        println!("\t{}\t{}\t{}\t{}\n",
                 index, loc_var.var_name, loc_var.start_pc + 1, loc_var.end_pc + 1);
    }

    println!("upvalues ({}):", f.upvalues.len());
    for (index, upval) in f.upvalues.iter().enumerate() {
        println!("\t{}\t{}\t{}\t{}\n",
                 index, upvalue_name(f, index), upval.in_stack, upval.idx);
    }
}

fn constant_to_string(constant: &Constant) -> String {
    match constant {
        Constant::Nil => format!("nil"),
        Constant::Boolean(b) => format!("\"{}\"", b),
        Constant::Number(n) => format!("\"{}\"", n),
        Constant::Integer(i) => format!("\"{}\"", i),
        Constant::String(s) => format!("\"{}\"", s),
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