use std::fs;

struct Directory<'a> {
    name: &'a str,
    files: Vec<u32>,
    directories: Vec<Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn get_subdir_pos(&self, name: &str) -> Option<usize> {
        return self
            .directories
            .iter()
            .position(|subdir| subdir.name == name);
    }

    fn new_dir(name: &str) -> Directory {
        let new_dir = Directory {
            name,
            files: vec![],
            directories: vec![],
        };
        return new_dir;
    }

    fn add_subdir(&mut self, new_dir: Directory<'a>) -> usize {
        self.directories.push(new_dir);
        return self.directories.len() - 1;
    }

    fn get_size(&self) -> u32 {
        return self.files.iter().sum::<u32>()
            + self.directories.iter().map(|d| d.get_size()).sum::<u32>();
    }
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut root = Directory {
        name: "/",
        files: vec![],
        directories: vec![],
    };
    let mut wd = &mut root;
    let mut path: Vec<usize> = Vec::new();
    for line in lines.iter().skip(1) {
        if line.starts_with("$") {
            let directive = line[1..].trim();
            let args = directive.split(" ").collect::<Vec<&str>>();
            let command = args[0];
            if command == "cd" {
                let target = args[1];
                if target.starts_with("/") {
                    wd = &mut root;
                    if target == "/" {
                        path = Vec::new();
                    } else {
                        for dir_name in target.split("/").skip(1) {
                            let subdir_pos = wd.get_subdir_pos(dir_name).unwrap();
                            path.push(subdir_pos);
                            wd = &mut wd.directories[subdir_pos];
                        }
                    }
                } else if target == ".." {
                    path.pop();
                    wd = &mut root;
                    for pos in &path {
                        wd = &mut wd.directories[*pos];
                    }
                } else {
                    let subdir_pos = wd
                        .get_subdir_pos(target)
                        .expect(&format!("No subdir: {:?}", target));
                    path.push(subdir_pos);
                    wd = &mut wd.directories[subdir_pos];
                }
            }
        } else {
            let entries = line.split(" ").collect::<Vec<&str>>();
            if entries[0] == "dir" {
                let dir_name = entries[1].trim();
                let new_dir = Directory::new_dir(dir_name);
                wd.add_subdir(new_dir);
            } else {
                let file_size = entries[0].parse::<u32>().unwrap();
                wd.files.push(file_size);
            }
        }
    }

    // Part 1
    let answer = get_total_sizes_under(&root, 100_000);
    println!(
        "Total size of all directories under size 100,000: {}",
        answer
    );

    // Part 2
    let size = 70000000;
    let used_space = root.get_size();
    let free_space = size - used_space;
    let space_required = 30000000;
    let deletion_minimum = space_required - free_space;
    println!(
        "First smallest dir size meeting deletion requirement: {}",
        find_dir_size_closest_to_size(&root, deletion_minimum).unwrap()
    );
}

fn find_dir_size_closest_to_size(dir: &Directory, size: u32) -> Option<u32> {
    let dir_size = dir.get_size();
    let mut closest_size = None;
    if dir_size >= size {
        closest_size = Some(dir_size);
    }
    for subdir in &dir.directories {
        let subdir_closest = find_dir_size_closest_to_size(subdir, size);
        closest_size = closest_size.min(subdir_closest).or(closest_size);
    }
    return closest_size;
}

fn get_total_sizes_under(dir: &Directory, max_size: u32) -> u32 {
    let mut total: u32 = 0;
    let dir_size = dir.get_size();
    if dir_size <= max_size {
        total += dir_size;
    }
    for subdir in &dir.directories {
        total += get_total_sizes_under(subdir, max_size);
    }
    return total;
}
