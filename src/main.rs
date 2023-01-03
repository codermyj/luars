use std::{env, fs};

mod binarychunk;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match fs::read(&args[1]) {
            Ok(data) => {
                let proto = binarychunk::undump(data);

            }
            Err(e) => {
                panic!(e);
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
}

fn print_header(f: &binarychunk::Prototype) {
    let fn_type = "main";

}