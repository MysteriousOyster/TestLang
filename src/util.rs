use std::{fs, path::Path, sync::Arc};

use input_macro::input;

use crate::{Args, Function};

fn add(args: Args) -> String {
    let first = args[0].parse::<i64>().unwrap();
    let second = args[1].parse::<i64>().unwrap();
    let result = first + second;
    result.to_string()
}
fn print(args: Args) -> String {
    for arg in args {
        print!("{arg} ");
    }
    "".into()
}
fn println(args: Args) -> String {
    print(args);
    println!();
    "".into()
}
fn file_delete(args: Args) -> String {
    let res = fs::remove_file(&Path::new(&args[0]));
    match res {
        Ok(_) => "Done!".into(),
        Err(e) => format!("{e}"),
    }
}

fn equals(args: Args) -> String {
    if args[0] == args[1] {
        "yes"
    } else {
        "no"
    }.into()
}

fn greater_than(args: Args) -> String {
    if args[0].parse::<i64>().unwrap() > args[1].parse::<i64>().unwrap() {
        "yes"
    } else {
        "no"
    }.into()
}

fn less_than(args: Args) -> String {
    if args[0].parse::<i64>().unwrap() < args[1].parse::<i64>().unwrap() {
        "yes"
    } else {
        "no"
    }.into()
}

fn and(args: Args) -> String {
    if args[0] == "yes" && args[1] == "yes" {
        "yes"
    } else {
        "no"
    }.into()
}

fn or(args: Args) -> String {
    if args[0] == "yes" || args[1] == "yes" {
        "yes"
    } else {
        "no"
    }.into()
}

fn not(args: Args) -> String {
    if args[0] == "yes" {
        "no"
    } else {
        "yes"
    }.into()
}

fn input(args: Args) -> String {
    match args.join(" ") {
        x if x == "" => input!(),
        x => input!("{x}")
    }
}

fn echo(args: Args) -> String {
    args[0].clone()
}

pub fn generate_default_functions() -> Vec<Function> {
    vec![
        Function::new("add".into(), Arc::new(add)),
        Function::new("print".into(), Arc::new(print)),
        Function::new("println".into(), Arc::new(println)),
        Function::new("file.delete".into(), Arc::new(file_delete)),
        Function::new("eq".into(), Arc::new(equals)),
        Function::new("gt".into(), Arc::new(greater_than)),
        Function::new("lt".into(), Arc::new(less_than)),
        Function::new("input".into(), Arc::new(input)),
        Function::new("&&".into(), Arc::new(and)),
        Function::new("||".into(), Arc::new(or)),
        Function::new("!!".into(), Arc::new(not)),
        Function::new("ech".into(), Arc::new(echo))
    ]
}
