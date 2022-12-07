use crate::Problem;

struct AocFileSystem {
    // Root file
    folders: Vec<AocFolder>,
}

struct AocFolder {
    name: String,
    files: Vec<AocFile>,
    folders: Vec<AocFolder>,
    size: u32,
    parent: Option<Box<AocFolder>>, // None == FS Root
}

struct AocFile {
    name: String,
    size: u32,
}

fn interpret_input(list: Vec<&str>) {
    for i in list {
        println!("{:?}", i);
        if i.starts_with('$') {
            println!("Is command")
        } else if i.starts_with("dir") {
            println!("Is dir")
        } else {
            println!("Is file")
        }
    }
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
