use advent_of_code_2022::*;
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day07.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum FileSystem {
    File(String, usize),
    Directory(String, Vec<FileSystem>),
}

impl FileSystem {
    fn name(&self) -> &str {
        match self {
            FileSystem::File(name, _) => name,
            FileSystem::Directory(name, _) => name,
        }
    }

    fn size(&self) -> usize {
        match self {
            FileSystem::File(_, size) => *size,
            FileSystem::Directory(_, children) => children.iter().map(|child| child.size()).sum(),
        }
    }
}

fn filesystem(input: &str) -> Result<FileSystem> {
    let mut files = Vec::new();
    let mut stack: Vec<(String, Vec<FileSystem>)> = Vec::new();
    let mut name = "/".to_string();
    for line in input.trim().lines() {
        if line == "$ cd .." {
            let node = FileSystem::Directory(name, files);
            (name, files) = stack.pop().unwrap();
            files.push(node);
        } else if line.starts_with("$ cd") {
            stack.push((name, files));
            name = line[5..].trim().to_string();
            files = Vec::new();
        } else if line == "$ ls" {
            continue;
        } else if line.starts_with("dir") {
            continue;
        } else {
            let (size, name) = line.split_once(" ").unwrap();
            files.push(FileSystem::File(name.to_string(), size.parse::<usize>().unwrap()));
        }
    };

    let mut node = FileSystem::Directory(name, files.clone());
    for (name, f) in stack.iter().rev() {
        files = f.clone();
        files.push(node);
        node = FileSystem::Directory(name.clone(), files.clone());
    };

    Ok(FileSystem::Directory("/".to_string(), files))
}

fn walk_tree<F>(node: &FileSystem, f: &mut F)
    where F: FnMut(&FileSystem) {
        f(node);
        match node {
            FileSystem::File(_, _) => (),
            FileSystem::Directory(_, children) => {
                for child in children {
                    walk_tree(child, f);
                }
            }
        };
}

fn part1(input: &str) -> Result<usize> {
    let fs = filesystem(input)?;
    let mut size = 0;
    walk_tree(&fs, &mut |node: &FileSystem| {
        if node.size() >= 100000 {
            return;
        }

        match node {
            FileSystem::File(_, _) => (),
            FileSystem::Directory(_, _) => size += node.size(),
        };
    });
    Ok(size)
}

const FS_SIZE: usize = 70000000;
const FS_UPDATE: usize = 30000000;

fn part2(input: &str) -> Result<usize> {
    let fs = filesystem(input)?;
    let space_needed = FS_UPDATE - (FS_SIZE - fs.size());

    let mut size = usize::MAX;
    walk_tree(&fs, &mut |node: &FileSystem| {
        if node.size() < space_needed {
            return;
        }

        match node {
            FileSystem::File(_, _) => (),
            FileSystem::Directory(_, _) => {
                if size > node.size() {
                    size = node.size();
                }
            }
        };
    });
    Ok(size)
}

fn main() -> Result<()> {
    println!("part 1: {0}", part1(INPUT)?);
    println!("part 2: {0}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = make_input(
            r###"
$ cd /
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
7214296 k
            "###
        );

        assert_eq!(95437, part1(&input)?);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = make_input(
            r###"
$ cd /
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
7214296 k
            "###
        );

        assert_eq!(24933642, part2(&input)?);

        Ok(())
    }
}
