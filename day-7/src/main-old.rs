use trees::{Tree, Node};

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

struct FsNode {
    name: String,
    is_dir: bool,
    size: Option<usize>
}

impl FsNode {
    fn new(name: String, is_dir: bool, size: Option<usize>) -> Self {
        Self {
            name,
            is_dir,
            size
        }
    }

    fn new_dir(name: String) -> Self {
        Self::new(name, true, None)
    }

    fn new_file(name: String, size: usize) -> Self {
        Self::new(name, false, Some(size))
    }
}

struct FileSystem<'a> {
    tree: Tree<FsNode>,
    pwd: Box<&'a Node<FsNode>>
}

impl<'a> FileSystem<'a> {
    fn new() -> Self {
        let root = FsNode::new_dir("/".to_string());
        let tree = Tree::new(root);

        Self {
            pwd: Box::new(tree.root()),
            tree,
        }
    }

    fn cd(self: &Self, name: String) {
        if name == ".." {
            self.pwd = Box::new(self.pwd.parent().unwrap());
        } else if name == "/" {
            self.pwd = Box::new(self.tree.root());
        } else if let Some(dir) = self.pwd.iter().find(|n| n.data().name == name) {
            self.pwd = Box::new(dir);
        }
    }

    fn mkdir(self: &Self, name: String) {
        let fs_node = FsNode::new_dir(name);
        self.pwd.push_back(Tree::new(fs_node));
    }

    fn touch(self: &Self, name: String, size: usize) {
        let fs_node = FsNode::new_file(name, size);
        self.pwd.push_back(Tree::new(fs_node));
    }
}

fn interpret_command<'a>(fs: &'a FileSystem, cmd: ExecutedCommand) -> Result<(), String> {
    match cmd.input {
        Command::Cd(path) => Ok(fs.cd(path.clone())),
        Command::Ls => {
            for line in cmd.output {
                if line.starts_with("dir") {
                    let dirname = line.split(" ").last().expect("expected dir to contain name").to_string();
                    fs.mkdir(dirname);
                } else {
                    let mut cmd_lines = line.split(" ");
                    let size: usize = cmd_lines.next().expect("expected dir to contain pathname").parse().expect("expected size to be num");
                    let filename = cmd_lines.last().expect("expected dir to contain pathname").to_string();
                    fs.touch(filename, size)
                }
            }
            Ok(())
        }
    }
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

    let fs = FileSystem::new();

    for cmd in commands {
        interpret_command(&fs, cmd).expect("could not intepret command");
    }

    println!("Hello, world!");
}
