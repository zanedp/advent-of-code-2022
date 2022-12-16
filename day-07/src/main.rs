use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Clone, Debug, Default)]
struct FileSystemEntry {
    name: String,
    /// 0 means this is a directory
    size: usize,
    children: HashMap<String, FsHandle>,
}

type FsHandle = Rc<RefCell<FileSystemEntry>>;

impl FileSystemEntry {
    pub fn is_dir(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        let sum = Cell::new(0usize);
        self.visit(|fs: &FileSystemEntry| {
            sum.set(sum.get() + fs.size);
        });
        sum.get()
    }

    pub fn add_file(&mut self, name: &str, size: usize) {
        self.children
            .entry(String::from(name))
            .or_insert(Rc::new(RefCell::new(Self {
                name: String::from(name),
                size,
                ..Self::default()
            })));
    }

    pub fn add_or_get_dir(&mut self, name: &str) -> FsHandle {
        let new_dir = Rc::new(RefCell::new(Self {
            name: String::from(name),
            ..Self::default()
        }));
        Rc::clone(self.children.entry(String::from(name)).or_insert(new_dir))
    }

    pub fn visit<F>(&self, mut func: F)
    where
        F: FnMut(&FileSystemEntry) + Copy,
    {
        self.children
            .values()
            .for_each(|x| x.borrow_mut().visit::<F>(func));
        func(self);
    }

    fn print_tree_helper(&self, depth: usize) {
        for _ in 0..depth {
            print!(" ");
        }
        print!("{}", self.name);
        if self.is_dir() {
            print!("/");
            println!();
            self.children
                .values()
                .for_each(|x| Rc::clone(x).borrow().print_tree_helper(depth + 4));
        } else {
            println!();
        }
    }

    #[allow(dead_code)]
    pub fn print_tree(&self) {
        self.print_tree_helper(0)
    }
}

#[allow(dead_code)]
fn path_to_string(cur_path: &Vec<FsHandle>) -> String {
    cur_path
        .iter()
        .map(|x| Rc::clone(x).borrow_mut().name.clone())
        .collect::<Vec<_>>()
        .join("/")
}

fn main() {
    let input = include_str!("input.txt");
    //    let mut dir_contents = HashMap::new();
    //   let mut file_sizes = HashMap::new();
    let mut cwd = Rc::new(RefCell::new(FileSystemEntry {
        name: "<fs>".to_string(),
        ..FileSystemEntry::default()
    }));
    let fs = Rc::clone(&cwd);
    let mut cur_path: Vec<FsHandle> = Vec::new();
    cur_path.push(Rc::clone(&cwd));
    for line in input.lines() {
        if line.starts_with("$") {
            // command
            let cmd = &line[2..];
            if cmd.starts_with("cd") {
                if let Some((_cd, dir_name)) = cmd.split_once(' ') {
                    match dir_name {
                        "/" => {
                            continue;
                        }
                        ".." => {
                            // going up to parent dir
                            cur_path.pop();
                            let new_wd = cur_path.last().unwrap();
                            cwd = Rc::clone(new_wd);
                        }
                        _ => {
                            // changing into a subdirectory
                            let new_wd = cwd.borrow_mut().add_or_get_dir(dir_name);
                            cur_path.push(Rc::clone(&new_wd));
                            cwd = new_wd;
                        }
                    }
                }
            } else if cmd.starts_with("ls") {
            }
            // we don't need to do anything to process this command
            else {
                panic!("Unrecognized command: {}", cmd);
            }
        } else {
            // must be output of a command
            let (left, right) = line.split_once(" ").unwrap();
            match left {
                "dir" => {
                    let name = right.to_string();
                    cwd.borrow_mut().add_or_get_dir(&name);
                }
                _ => {
                    let size = left.parse().unwrap();
                    let name = right;
                    cwd.borrow_mut().add_file(name, size);
                }
            }
        }
    }

    let dir_sizes = RefCell::new(Vec::new());
    fs.borrow().visit(|fs| {
        if fs.is_dir() {
            dir_sizes
                .borrow_mut()
                .push((fs.name.to_string(), fs.size()));
        }
    });
    //println!("{:#?}", dir_sizes);

    let sum = dir_sizes
        .into_inner()
        .into_iter()
        .filter(|(_name, size)| *size < 100000)
        //.inspect(|(name, size)| println!("{name} {size}"))
        .fold(0, |a, e| a + e.1);

    println!("sum: {sum}");

    //Rc::clone(&fs).borrow().print_tree();
    //println!("total size = {}", fs.size());
    //println!("{:#?}", fs);
}
