use crate::Problem;

struct AocFileSystem {
    // Root file
    // Maybe Implement Buildup fn here
    // Calculate (call from AocFolders)sizes here after
    folders: Vec<AocFolder>,
}

struct AocFolder {
    name: String,
    files: Vec<AocFile>,
    folders: Vec<AocFolder>,
    // size: u32, || use size fn
    parent: Option<Box<AocFolder>>, // None == FS Root
}

impl AocFolder {
    fn new(
        name: String,
        files: Vec<AocFile>,
        folders: Vec<AocFolder>,
        // size: u32,
        parent: Option<Box<AocFolder>>,
    ) -> AocFolder {
        AocFolder {
            name,
            files,
            folders,
            // size,
            parent,
        }
    }

    fn size(&self) -> u32 {
        let mut total = 0;
        for f in &self.files {
            total += f.size
        }
        total
    }
}

#[derive(Debug)]
struct AocFile {
    name: String,
    size: u32,
}

impl AocFile {
    fn new(name: String, size: u32) -> AocFile {
        AocFile { name, size }
    }
}

fn interpret_input(list: Vec<&str>) {
    for i in list {
        println!("{:?}", i);
        if i.starts_with('$') {
            println!("Is command");
            interpret_command(i);
        } else if i.starts_with("dir") {
            println!("Is dir")
        } else {
            println!("Is file");
            interpret_file_entry(i)
        }
    }
}

fn interpret_command(aoc_cmd: &str) {
    let cmd = aoc_cmd
        .strip_prefix('$')
        .unwrap()
        .trim()
        .split(' ')
        .collect::<Vec<_>>();
    if cmd[0] == "cd" {
        move_command(cmd[1])
    } else {
        println!("ls --> NEXT ENTRY")
    }
}

fn move_command(to: &str) {
    if to == ".." {
        println!("FS MOVE PARENT")
    } else {
        println!("FS MOVE INTO {}", to)
    }
}

fn interpret_file_entry(entry: &str) {
    let e = entry.split(' ').collect::<Vec<_>>();
    let f = AocFile::new(e[1].to_string(), e[0].parse::<u32>().unwrap());

    println!("{:?}", f)
}

pub struct DaySeven {}

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let i = input.split('\n').collect::<Vec<_>>();

        interpret_input(i);
        "".to_string()
    }
    fn part_two(&self, input: &str) -> String {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let day = DaySeven {};
        assert_eq!(day.part_one(input), "95437")
    }
    #[test]
    fn test_part_two() {}
}
