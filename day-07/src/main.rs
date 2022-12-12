use std::rc::Rc;

#[derive(Clone, Debug)]
enum FileSystemEntry {
    Dir(Dir),
    File(File),
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Default, Clone, Debug)]
struct Dir {
    name: String,
    entries: Vec<Rc<FileSystemEntry>>,
}

impl Dir {
    fn size(&self) -> usize {
        let mut sum = 0;
        for e in self.entries.iter() {
            sum += match &**e {
                FileSystemEntry::File(f) => f.size,
                FileSystemEntry::Dir(d) => d.size(),
            }
        }
        sum
    }

    fn new_dir(&mut self, name: &str) -> Rc<FileSystemEntry> {
        let new_dir = Dir {
            name: String::from(name),
            ..Dir::default()
        };
        let b = Rc::new(FileSystemEntry::Dir(new_dir));
        self.entries.push(b.clone());
        b
    }
}

fn main() {
    let input = include_str!("sample_input.txt");
    //    let mut dir_contents = HashMap::new();
    //   let mut file_sizes = HashMap::new();
    let mut cur_path: Vec<Rc<FileSystemEntry>> = Vec::new();
    let mut fs = Dir::default();
    let mut cwd: Option<Rc<FileSystemEntry>> = None;
    for line in input.lines() {
        if line.starts_with("$") {
            // command
            let cmd = &line[2..];
            match cmd {
                "cd" => {
                    if let Some((_cd, dir_name)) = cmd.split_once(' ') {
                        let new_dir = if cwd.is_none() {
                            fs.new_dir(dir_name)
                        } else {
                            match &cwd.unwrap().clone() {
                                FileSystemEntry::Dir(d) => d.new_dir(dir_name),
                                _ => panic!("cwd should only be a Dir"),
                            }
                        };
                        cur_path.push(new_dir.clone());
                        cwd = Some(new_dir);
                    }
                }
                "ls" => {
                    // we don't need to do anything to process this command
                }
                _ => {
                    panic!("Unrecognized command: {}", cmd);
                }
            }
        }
    }
}
