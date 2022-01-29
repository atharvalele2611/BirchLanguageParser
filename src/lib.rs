//! lib.rs: contains the main logic for parsing and execution of birch language.
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Num(i64),
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Lt,
    Gt,
    Ifz,
    Dup,
    Pop,
    Rev,
    Swap,
    Cmds(Vec<Command>),
    Exec,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Command::Num(n) => n.fmt(f),
            Command::Add => f.write_str("add"),
            Command::Sub => f.write_str("sub"),
            Command::Mul => f.write_str("mul"),
            Command::Div => f.write_str("div"),
            Command::Rem => f.write_str("rem"),
            Command::Eq => f.write_str("eq"),
            Command::Lt => f.write_str("lt"),
            Command::Gt => f.write_str("gt"),
            Command::Ifz => f.write_str("ifz"),
            Command::Dup => f.write_str("dup"),
            Command::Pop => f.write_str("pop"),
            Command::Swap => f.write_str("swap"),
            Command::Rev => f.write_str("rev"),
            Command::Cmds(cmds) => {
                if cmds.is_empty() {
                    f.write_str("[ ]")
                } else {
                    f.write_str("[ ")?;
                    fmt_slice_rev(&cmds[..], f)?;
                    f.write_str(" ]")
                }
            }
            Command::Exec => f.write_str("exec"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prog(Vec<Command>);
impl Display for Prog {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_slice_rev(&self.0[..], f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgParseError;
impl FromStr for Prog {
    type Err = ProgParseError;
    fn from_str(s: &str) -> Result<Prog, ProgParseError> {
        // Your code here
        let mut vec: Vec<Command> = Vec::new();
        let mut command_stack: Vec<Vec<Command>> = Vec::new();
        let mut brackets: Vec<&str> = Vec::new();
        for w in s.split_whitespace() {
            match w.parse::<i64>() {
                Ok(n) => {
                    push_to_prog_vec(&mut command_stack, &mut vec, Command::Num(n));
                }
                Err(_) => match w {
                    "[" => {
                        brackets.push("[");
                        command_stack.push(Vec::new());
                    }
                    "]" => {
                        if brackets.is_empty() {
                            return Err(ProgParseError);
                        }
                        let mut sub_command = match command_stack.pop() {
                            Some(top) => top,
                            None => {
                                return Err(ProgParseError);
                            }
                        };
                        match command_stack.pop() {
                            Some(mut top) => {
                                sub_command.reverse();
                                top.push(Command::Cmds(sub_command));
                                command_stack.push(top);
                            }
                            None => {
                                sub_command.reverse();
                                vec.push(Command::Cmds(sub_command));
                            }
                        }
                        brackets.pop();
                    }
                    _ => match match_commands(&mut command_stack, &mut vec, w) {
                        Ok(w) => w,
                        Err(_) => return Err(ProgParseError),
                    },
                },
            }
        }
        if !brackets.is_empty() {
            return Err(ProgParseError);
        }
        vec.reverse();
        Ok(Prog(vec))
    }
}

/// matches the given string to the pattern and adds it to the appropriate stack
fn match_commands(
    command_stack: &mut Vec<Vec<Command>>,
    vec: &mut Vec<Command>,
    w: &str,
) -> Result<(), ProgParseError> {
    match w {
        "add" => {
            push_to_prog_vec(command_stack, vec, Command::Add);
        }
        "sub" => {
            push_to_prog_vec(command_stack, vec, Command::Sub);
        }
        "mul" => {
            push_to_prog_vec(command_stack, vec, Command::Mul);
        }
        "div" => {
            push_to_prog_vec(command_stack, vec, Command::Div);
        }
        "rem" => {
            push_to_prog_vec(command_stack, vec, Command::Rem);
        }
        "eq" => {
            push_to_prog_vec(command_stack, vec, Command::Eq);
        }
        "lt" => {
            push_to_prog_vec(command_stack, vec, Command::Lt);
        }
        "gt" => {
            push_to_prog_vec(command_stack, vec, Command::Gt);
        }
        "ifz" => {
            push_to_prog_vec(command_stack, vec, Command::Ifz);
        }
        "dup" => {
            push_to_prog_vec(command_stack, vec, Command::Dup);
        }
        "pop" => {
            push_to_prog_vec(command_stack, vec, Command::Pop);
        }
        "swap" => {
            push_to_prog_vec(command_stack, vec, Command::Swap);
        }
        "rev" => {
            push_to_prog_vec(command_stack, vec, Command::Rev);
        }
        "exec" => {
            push_to_prog_vec(command_stack, vec, Command::Exec);
        }
        _ => return Err(ProgParseError),
    }
    Ok(())
}

/// pushes the command to appropriate stack.
fn push_to_prog_vec(command_stack: &mut Vec<Vec<Command>>, vec: &mut Vec<Command>, cmd: Command) {
    match command_stack.pop() {
        Some(mut top) => {
            top.push(cmd);
            command_stack.push(top);
        }
        None => {
            vec.push(cmd);
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgExecError;

#[derive(Debug)]
struct CmdStack(Vec<Command>);

impl Display for CmdStack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_slice_rev(&self.0[..], f)
    }
}

impl CmdStack {
    fn new(prog: &Prog) -> Self {
        // Your code here
        return CmdStack(prog.0.clone());
    }

    fn push(&mut self, cmd: Command) {
        self.0.push(cmd)
    }

    fn pop(&mut self) -> Result<Command, ProgExecError> {
        match self.0.pop() {
            Some(cmd) => {
                return Ok(cmd);
            }
            _ => Err(ProgExecError),
        }
    }
    // Your code here; additional methods as necessary
}

#[derive(Debug, Clone)]
enum DataElem {
    Num(i64),
    Cmds(Vec<Command>),
}
impl Display for DataElem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataElem::Num(n) => n.fmt(f),
            DataElem::Cmds(cmds) => {
                if cmds.is_empty() {
                    f.write_str("[ ]")
                } else {
                    f.write_str("[ ")?;
                    fmt_slice_rev(&cmds[..], f)?;
                    f.write_str(" ]")
                }
            }
        }
    }
}

#[derive(Debug)]
struct DataStack(Vec<DataElem>);
impl Display for DataStack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_slice_rev(&self.0[..], f)
    }
}
impl DataStack {
    fn new() -> Self {
        DataStack(Vec::new())
    }
    fn push(&mut self, de: DataElem) {
        self.0.push(de)
    }
    fn pop(&mut self) -> Result<DataElem, ProgExecError> {
        match self.0.pop() {
            Some(de) => Ok(de),
            _ => Err(ProgExecError),
        }
    }
}

impl Prog {
    /// The exec function to handle the possible executions of different commands in the command stack.
    /// The execution stops when the command stack is empty.
    pub fn exec(&self, trace: bool) -> Result<i64, ProgExecError> {
        if trace {
            println!("prog: {}\n", self)
        }
        let mut cstk = CmdStack::new(self);
        let mut dstk = DataStack::new();
        let mut step: u64 = 0;
        loop {
            if trace {
                println!("step: {}\ncstk: {}\ndstk: {}\n", step, cstk, dstk);
            };

            let command = match cstk.pop() {
                Ok(c) => c,
                Err(_) => match dstk.pop()? {
                    DataElem::Num(n) => return Ok(n),
                    DataElem::Cmds(_) => return Err(ProgExecError),
                },
            };

            match command {
                Command::Num(n) => dstk.push(DataElem::Num(n)),
                Command::Cmds(cmds) => dstk.push(DataElem::Cmds(cmds)),
                Command::Add => {
                    match dstk.pop() {
                        Ok(DataElem::Num(n)) => match dstk.pop() {
                            Ok(DataElem::Num(m)) => {
                                let y = n + m;
                                dstk.push(DataElem::Num(y));
                            }
                            _ => {
                                return Err(ProgExecError);
                            }
                        },
                        _ => return Err(ProgExecError),
                    };
                }
                Command::Sub => {
                    match dstk.pop() {
                        Ok(DataElem::Num(n)) => match dstk.pop() {
                            Ok(DataElem::Num(m)) => {
                                let y = n - m;
                                dstk.push(DataElem::Num(y));
                            }
                            _ => {
                                return Err(ProgExecError);
                            }
                        },
                        _ => return Err(ProgExecError),
                    };
                }
                Command::Mul => {
                    match dstk.pop() {
                        Ok(DataElem::Num(n)) => match dstk.pop() {
                            Ok(DataElem::Num(m)) => {
                                let y = n * m;
                                dstk.push(DataElem::Num(y));
                            }
                            _ => {
                                return Err(ProgExecError);
                            }
                        },
                        _ => return Err(ProgExecError),
                    };
                }
                Command::Div => {
                    match dstk.pop() {
                        Ok(DataElem::Num(n)) => match dstk.pop() {
                            Ok(DataElem::Num(m)) => {
                                if m == 0 {
                                    return Err(ProgExecError);
                                }
                                let y = n / m;
                                dstk.push(DataElem::Num(y));
                            }
                            _ => {
                                return Err(ProgExecError);
                            }
                        },
                        _ => return Err(ProgExecError),
                    };
                }
                Command::Rem => {
                    match dstk.pop() {
                        Ok(DataElem::Num(n)) => match dstk.pop() {
                            Ok(DataElem::Num(m)) => {
                                if m == 0 {
                                    return Err(ProgExecError);
                                }
                                let y = n % m;
                                dstk.push(DataElem::Num(y));
                            }
                            _ => {
                                return Err(ProgExecError);
                            }
                        },
                        _ => return Err(ProgExecError),
                    };
                }
                Command::Eq => {
                    match dstk.pop() {
                        Ok(DataElem::Num(n)) => match dstk.pop() {
                            Ok(DataElem::Num(m)) => {
                                if n == m {
                                    dstk.push(DataElem::Num(1));
                                } else {
                                    dstk.push(DataElem::Num(0));
                                }
                            }
                            _ => return Err(ProgExecError),
                        },
                        _ => return Err(ProgExecError),
                    };
                }
                Command::Lt => {
                    match dstk.pop() {
                        Ok(DataElem::Num(n)) => match dstk.pop() {
                            Ok(DataElem::Num(m)) => {
                                if n < m {
                                    dstk.push(DataElem::Num(1));
                                } else {
                                    dstk.push(DataElem::Num(0));
                                }
                            }
                            _ => return Err(ProgExecError),
                        },
                        _ => return Err(ProgExecError),
                    };
                }
                Command::Gt => {
                    match dstk.pop() {
                        Ok(DataElem::Num(n)) => match dstk.pop() {
                            Ok(DataElem::Num(m)) => {
                                if n > m {
                                    dstk.push(DataElem::Num(1));
                                } else {
                                    dstk.push(DataElem::Num(0));
                                }
                            }
                            _ => return Err(ProgExecError),
                        },
                        _ => return Err(ProgExecError),
                    };
                }
                Command::Ifz => {
                    let elem1 = dstk.pop()?;
                    let elem2 = dstk.pop()?;
                    let elem3 = dstk.pop()?;

                    match elem1 {
                        DataElem::Num(n) => {
                            if n == 0 {
                                dstk.push(elem2);
                            } else {
                                dstk.push(elem3);
                            }
                        }
                        DataElem::Cmds(_) => {
                            dstk.push(elem3);
                        }
                    }
                }
                Command::Dup => match dstk.pop() {
                    Ok(DataElem::Num(n)) => {
                        if n >= 0 {
                            let length = dstk.0.len();
                            if n + 1 > length as i64 {
                                return Err(ProgExecError);
                            }
                            let index = length - (n + 1) as usize;
                            match dstk.0.get(index) {
                                Some(el) => {
                                    let e = (*el).clone();
                                    dstk.push(e);
                                }
                                None => return Err(ProgExecError),
                            }
                        } else {
                            match dstk.0.get(n.abs() as usize - 1) {
                                Some(el) => {
                                    let e = (*el).clone();
                                    dstk.push(e);
                                }
                                None => return Err(ProgExecError),
                            }
                        }
                    }
                    _ => {
                        return Err(ProgExecError);
                    }
                },
                Command::Pop => {
                    dstk.pop()?;
                }
                Command::Swap => {
                    let a = dstk.pop()?;
                    let b = dstk.pop()?;

                    dstk.push(a);
                    dstk.push(b);
                }
                Command::Rev => dstk.0.reverse(),
                Command::Exec => {
                    let cmds = match dstk.pop()? {
                        DataElem::Cmds(c) => c,
                        _ => return Err(ProgExecError),
                    };

                    for c in cmds {
                        cstk.push(c);
                    }
                }
            };
            step += 1;
        }
    }
}

/// A (private) helper function to display a slice in "reverse" order with
/// single spaces between elements.  Useful for displaying a `Vec<T>` being used
/// as a stack, so that the top element appears to the left and the bottom
/// element appears to the right.
fn fmt_slice_rev<T>(slc: &[T], f: &mut Formatter) -> fmt::Result
where
    T: Display,
{
    let mut first = true;
    for x in slc.iter().rev() {
        if first {
            first = false;
        } else {
            f.write_str(" ")?;
        };
        x.fmt(f)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
