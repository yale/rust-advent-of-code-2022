use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Debug)]
enum Command { Cd(String), Ls }

#[derive(Debug)]
struct ExecutedCommand {
    input: Command,
    output: Vec<String>
}

impl ExecutedCommand {
    fn parse(s: String) -> Self {
        let mut lines = s.split("\n");
        // There will always be at least one line
        let cmd_str = lines.next().expect("Expected a command");

        let cmd = if cmd_str.starts_with("cd") {
            let arg = cmd_str.split(" ").last().expect("Expected an argument to cd");
            Command::Cd(arg.to_string())
        } else {
            Command::Ls
        };

        Self { input: cmd, output: lines.map(str::to_string).collect() }
    }
}

#[derive(Debug)]
struct FsNode {
    name: String,
    size: usize,
    idx: usize,
    parent: Option<usize>,
    children: Vec<usize>
}

#[derive(Debug, Default)]
struct FsTree {
    data: Vec<FsNode>,
}

// impl FsNode {
//     fn new(name: String, size: usize, idx: usize, val: T) -> Self {
//         Self {
//             idx,
//             val,
//             parent: None,
//             children: vec![],
//         }
//     }
// }

impl FsTree {
    fn insert(&mut self, name: String, size: usize, parent: Option<usize>) -> usize {
        let idx = self.data.len();
        let node = FsNode {
            idx,
            name,
            size,
            parent,
            children: vec![]
        };

        &self.data.push(node);
        idx
    }

    fn get(&self,
}

fn main() {
    let contents = include_str!("input.txt").trim();
    let commands: Vec<ExecutedCommand> = contents
        .split("$")
        .map(str::trim)
        .map(str::to_string)
        .map(ExecutedCommand::parse)
        .collect();
    dbg!(&commands);

    for cmd in commands {
        match cmd.input {
            Command::Cd(path) => {
                todo!();
            },
            Command::Ls => {
                for line in cmd.output {
                    if line.starts_with("dir") {
                        let dirname = line.split(" ").last().expect("expected dir to contain name").to_string();
                        todo!();
                    } else {
                        let mut cmd_lines = line.split(" ");
                        let size: usize = cmd_lines.next().expect("expected dir to contain pathname").parse().expect("expected size to be num");
                        let filename = cmd_lines.last().expect("expected dir to contain pathname").to_string();
                        todo!();
                    }
                }
            }
        }
    }

    println!("Hello, world!");
}
