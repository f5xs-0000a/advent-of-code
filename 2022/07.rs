use std::io::BufRead as _;
use std::collections::HashMap;

#[derive(Debug)]
enum DirEntry {
    Directory(Box<Folder>),
    File(usize),
}

#[derive(Debug)]
struct Folder(HashMap<String, DirEntry>);

impl Folder {
    fn solve_day_1(&self) -> usize {
        let mut total_size = 0;
        self.solve_inner(&mut |size| if size < 100000 { total_size += size });
        total_size
    }

    fn solve_day_2(&self, total_used: usize) -> usize {
        let total_free = 70_000_000 - total_used;
        let minimum_removable = 30_000_000 - total_free;

        let mut best_size = usize::MAX;
        self.solve_inner(&mut |size| {
            if minimum_removable <= size {
                if size < best_size {
                    best_size = size;
                }
            }
        });

        best_size
    }

    fn solve_inner(&self, size_update: &mut impl FnMut(usize)) -> usize {
        use DirEntry::*;

        let mut current_size = 0;
        for (_, entry) in self.0.iter() {
            current_size += match entry {
                Directory(folder) => folder.solve_inner(size_update),
                File(file_size) => *file_size,
            };
        }

        size_update(current_size);
        current_size
    }

    fn get_folder_size(&self) -> usize {
        use DirEntry::*;

        let mut folder_size = 0;

        for (_, entry) in self.0.iter() {
            folder_size += match entry {
                Directory(folder) => folder.get_folder_size(),
                File(file_size) => *file_size
            };
        }

        folder_size
    }
}

fn find_directory<'a>(
    directory_tree: &'a mut Folder,
    path: &[String])
-> &'a mut Folder {
    let mut directory = directory_tree;

    for folder in path.iter().skip(1) {
        match directory.0.get_mut(folder) {
            Some(DirEntry::Directory(ref mut f)) => directory = f,
            _ => unimplemented!(),
        }
    }

    directory
}


fn main() {
    // this isn't the optimal way to represent where you are right now in
    let mut current_path = vec![];

    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    let mut new_dirs: Vec<String> = vec![];
    let mut new_files: Vec<(usize, String)> = vec![];
    let mut directory_tree = Folder(HashMap::new());
    while {
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => false,
            Err(_) => false,
            _ => true
        }
    } {
        let mut words = buffer.split_whitespace();
        
        match words.next().unwrap() {
            "$" => {
                // if the contents of new dirs and new files are non empty,
                // append them into the directory
                {
                    let ref mut appending_directory = find_directory(
                        &mut directory_tree,
                        &current_path,
                    );
                    
                    for new_dir in new_dirs.drain(..) {
                        appending_directory.0.insert(
                            new_dir,
                            DirEntry::Directory(
                                Box::new(Folder(HashMap::new()))
                            ),
                        );
                    }

                    for (file_size, new_file) in new_files.drain(..) {
                        appending_directory.0.insert(
                            new_file,
                            DirEntry::File(file_size)
                        );
                    }
                }

                match words.next().unwrap() {
                    "cd" => match words.next().unwrap() {
                        ".." => { current_path.pop(); },
                        x => current_path.push(x.to_owned()),
                    }
                    "ls" => {}, // do nothing
                    _ => unreachable!(),
                };
            }
            "dir" => new_dirs.push(words.next().unwrap().to_owned()),
            num => {
                let num = num.parse::<usize>().unwrap();
                new_files.push((num, words.next().unwrap().to_owned()));
            },
        }
    }
        
    eprintln!("Day 7.1: {}", directory_tree.solve_day_1());
    let folder_size = directory_tree.get_folder_size();
    eprintln!("Day 7.2: {}", directory_tree.solve_day_2(folder_size));
}
