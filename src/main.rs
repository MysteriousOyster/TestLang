use std::{
    cell::RefCell,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Cursor},
    path::Path,
    rc::Rc,
    sync::Arc,
};

use anyhow::bail;
use clap::{ArgAction, Parser};

mod util;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLArgs {
    path: String,
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

type Args = Vec<String>;

#[derive(Debug, PartialEq, Eq)]
enum Action {
    BeginFunc(String, Args),
    EndFunc,
    CallFunc(String, Args),
    CreateVar(String, Option<String>),
    Assign(String),
    InFunc(String),
    Import(String),
    BeginIf(Option<String>),
    EndIf,
    InIf(String),
    None,
}

/// Parses a single line into an action
fn parse_line(line: String) -> anyhow::Result<Action> {
    //dbg!(line.clone());
    let args = line.split_whitespace().collect::<Vec<&str>>();
    //dbg!(args.clone());
    let mut iter = args.into_iter();
    match iter.next() {
        Some(x) => match x {
            "##" => Ok(Action::None),
            "imp" => match iter.next() {
                Some(p) => Ok(Action::Import(p.to_string())),
                None => bail!("must have import path"),
            },
            "|" => match iter
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
            {
                x if x == "" => Ok(Action::None),
                x => Ok(Action::InFunc(x)),
            },
            ">" => match iter
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
            {
                x if x == "" => Ok(Action::None),
                x => Ok(Action::InIf(x)),
            },
            "def" => match iter.next() {
                Some(keyword) => Ok(Action::BeginFunc(
                    keyword.into(),
                    iter.map(ToString::to_string).collect(),
                )),
                None => bail!("defined function must have name"),
            },
            "fed" => match iter.next() {
                Some(_) => bail!("expected nothing after `fed`"),
                None => Ok(Action::EndFunc),
            },
            "var" => match iter.next() {
                Some(keyword) => Ok(Action::CreateVar(
                    keyword.to_string(),
                    match iter.collect::<String>() {
                        x if x == "" => None,
                        x => Some(x),
                    },
                )),
                None => bail!("must have var name"),
            },
            "=" => match iter.next() {
                Some(keyword) => Ok(Action::Assign(keyword.to_string())),
                None => bail!("must have var name"),
            },
            "bif" => Ok(Action::BeginIf(iter.next().map(ToString::to_string))),
            "eif" => Ok(Action::EndIf),
            _ => {
                // dbg!(x); dbg!(iter.clone());
                Ok(Action::CallFunc(
                    x.to_string(),
                    iter.map(ToString::to_string).collect(),
                ))
            }
        },
        None => Ok(Action::None),
    }
}

#[derive(Clone)]
struct Function {
    keyword: String,
    callback: Arc<dyn Fn(Args) -> String>,
}

impl Function {
    pub fn new(keyword: String, callback: Arc<dyn Fn(Args) -> String>) -> Self {
        Self { keyword, callback }
    }
}

#[derive(Clone, Debug)]
struct VariableRecord(String, Option<String>);

/// Read-Evaluate-Print Loop on a BufReader
fn repl<R: BufRead>(
    reader: R,
    additional_vars: Rc<RefCell<Vec<VariableRecord>>>,
    additional_funcs: Rc<RefCell<Vec<Function>>>,
    verbose: bool,
) -> anyhow::Result<String> {
    let registered_functions: Rc<RefCell<Vec<Function>>> = additional_funcs.clone();
    let registered_vars: Rc<RefCell<Vec<VariableRecord>>> = additional_vars.clone();
    let lines = reader.lines();
    let mut count = 1;
    let mut assigning: Option<usize> = None;
    let mut funcing: Option<(String, Args, Vec<String>)> = None;
    let mut last_res: Option<String> = None;
    let mut iffingcheck = false;
    let mut iffingskip = false;
    let mut countingif: Option<Vec<String>> = None;
    for line in lines {
        if verbose {
            print!("{count}: ");
        }
        count += 1;
        let line = line?;
        let action = parse_line(line.clone())?;
        match action {
            x if iffingskip => {
                if x == Action::EndIf {
                    iffingskip = false;
                }
            }
            Action::BeginIf(x) => {
                countingif = Some(vec![]);
                match x {
                    None => iffingcheck = true,
                    Some(x) => {
                        let vars = registered_vars.borrow();
                        let x = vars.iter().find(|a| a.0 == x).unwrap();
                        if x.1.clone().unwrap() != "yes" {
                            iffingskip = true;
                            countingif = None;
                        }
                    }
                }
            }
            Action::InIf(command) => {
                if let Some(acc) = &mut countingif {
                    acc.push(command);
                } else {
                    bail!("there is no if to be in here")
                }
            }
            Action::EndIf => {
                if let Some(countingif) = countingif.take() {
                    iffingskip = false;
                    iffingcheck = false;
                    let commands = countingif.join("\n");
                    let curses = Cursor::new(commands);
                    let reader = BufReader::new(curses);
                    last_res = Some(repl(
                        reader,
                        registered_vars.clone(),
                        registered_functions.clone(),
                        verbose,
                    )?);
                } else {
                    bail!("no if to end")
                }
            }
            Action::CallFunc(name, args) => {
                let args = args
                    .iter()
                    .map(|s| {
                        if let Some(v) = registered_vars.borrow().iter().find(|v| &v.0 == s) {
                            v.1.clone().unwrap_or_default()
                        } else {
                            s.clone()
                        }
                    })
                    .collect::<Vec<String>>();
                let reg = registered_functions.borrow();
                let func = match reg.iter().find(|f| *f.keyword == name) {
                    Some(x) => x,
                    None => {
                        bail!("function {name} not found")
                    }
                };
                let res = (func.callback)(args);
                last_res = Some(res.clone());
                let mut vars = registered_vars.borrow_mut();
                if let Some(index) = assigning.take() {
                    let var = vars.get_mut(index).unwrap();
                    var.1 = Some(res.clone());
                }
                if iffingcheck {
                    if res != "yes" {
                        iffingskip = true;
                        countingif = None;
                    }
                    iffingcheck = false;
                }
                if verbose {
                    print!("{res}");
                }
            }
            Action::CreateVar(keyword, init) => {
                registered_vars
                    .borrow_mut()
                    .push(VariableRecord(keyword, init));
            }
            Action::Assign(keyword) => {
                assigning = Some(
                    registered_vars
                        .borrow_mut()
                        .iter_mut()
                        .position(|v| v.0 == keyword)
                        .unwrap(),
                );
            }
            Action::BeginFunc(keyword, args) => {
                funcing = Some((keyword, args, vec![]));
            }
            Action::InFunc(command) => {
                if let Some(func_builder) = &mut funcing {
                    func_builder.2.push(command);
                } else {
                    bail!("there is no function to do that to :(")
                }
            }
            Action::EndFunc => {
                if let Some(func_builder) = funcing.take() {
                    let adjoined = func_builder.2.join("\n");
                    let registered_vars = registered_vars.clone();
                    let registered_funcs = registered_functions.clone();
                    registered_functions.borrow_mut().push(Function::new(
                        func_builder.0,
                        Arc::new(move |args| -> String {
                            let add_vars = registered_vars.clone();
                            add_vars
                                .borrow_mut()
                                .extend(func_builder.1.iter().enumerate().map(|(i, f)| {
                                    VariableRecord(f.clone(), args.get(i).map(Clone::clone))
                                }));
                            let curses = Cursor::new(adjoined.clone());
                            let reader = BufReader::new(curses);
                            repl(reader, add_vars, registered_funcs.clone(), verbose).unwrap()
                        }),
                    ));
                } else {
                    bail!(
                        "you cant exit out of a function if there is no function to begin with\n--Sun Tzu"
                    )
                }
            }
            Action::Import(p) => {
                let p = Path::new(&p);
                let f = fs::OpenOptions::new().read(true).open(p)?;
                let reader = BufReader::new(f);
                repl(
                    reader,
                    additional_vars.clone(),
                    additional_funcs.clone(),
                    verbose,
                )?;
            }
            Action::None => {} //a => unimplemented!("{a:?} not yet implemented"),
        }
        if verbose {
            println!();
        }
    }
    Ok(last_res.unwrap_or_default())
}

fn main() -> anyhow::Result<()> {
    let args = CLArgs::parse();
    let path = Path::new(&args.path);
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    repl(
        reader,
        Rc::new(RefCell::new(vec![])),
        Rc::new(RefCell::new(util::generate_default_functions())),
        args.verbose,
    )?;
    Ok(())
}
