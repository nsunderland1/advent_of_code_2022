use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::not_line_ending,
    combinator::{map, value},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[allow(unused)]
use crate::prelude::*;

#[derive(Debug, Clone)]
enum FsEntry {
    Dir(Directory),
    File(File),
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    contents: Vec<FsEntry>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
enum Command {
    Ls(Vec<FsEntry>),
    Cd(Path),
}

#[derive(Debug, Clone)]
enum Path {
    Root,
    Up,
    Dir(String),
}

fn parse_file(input: &str) -> IResult<&str, File> {
    let (input, (size, name)) =
        separated_pair(nom::character::complete::u64, tag(" "), not_line_ending)(input)?;

    Ok((
        input,
        File {
            name: name.to_owned(),
            size: size as usize,
        },
    ))
}

fn parse_dir(input: &str) -> IResult<&str, Directory> {
    let (input, name) = preceded(tag("dir "), not_line_ending)(input)?;

    Ok((
        input,
        Directory {
            name: name.to_owned(),
            contents: Vec::new(),
        },
    ))
}

fn parse_path(input: &str) -> IResult<&str, Path> {
    alt((
        value(Path::Root, tag("/")),
        value(Path::Up, tag("..")),
        map(not_line_ending, |dir: &str| Path::Dir(dir.to_owned())),
    ))(input)
}

fn parse_fs_entry(input: &str) -> IResult<&str, FsEntry> {
    alt((map(parse_dir, FsEntry::Dir), map(parse_file, FsEntry::File)))(input)
}

fn parse_fs_entries(input: &str) -> IResult<&str, Vec<FsEntry>> {
    separated_list0(tag("\n"), parse_fs_entry)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    preceded(
        tag("$ "),
        alt((
            map(preceded(tag("ls\n"), parse_fs_entries), Command::Ls),
            map(preceded(tag("cd "), parse_path), Command::Cd),
        )),
    )(input)
}

fn parse_log(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list0(tag("\n"), parse_command)(input)
}

fn build_filesystem(log: Vec<Command>) -> Directory {
    let mut directories = HashMap::default();

    let mut current_path = vec!["".to_owned()];

    for command in log {
        match command {
            Command::Ls(entries) => {
                let current_dir = current_path.join("/");
                directories.insert(current_dir, entries);
            }
            Command::Cd(path) => match path {
                Path::Root => current_path = vec!["".to_owned()],
                Path::Up => {
                    current_path.pop().unwrap();
                }
                Path::Dir(path) => current_path.push(path),
            },
        }
    }

    fn recursively_build(table: &HashMap<String, Vec<FsEntry>>, path: String) -> Directory {
        let mut entries = Vec::new();

        for entry in table.get(&path).unwrap().iter() {
            match entry {
                FsEntry::Dir(directory) => {
                    entries.push(FsEntry::Dir(recursively_build(
                        table,
                        [path.as_str(), directory.name.as_str()].join("/"),
                    )));
                }
                FsEntry::File(file) => entries.push(FsEntry::File(file.clone())),
            }
        }

        Directory {
            name: path,
            contents: entries,
        }
    }

    recursively_build(&directories, "".to_owned())
}

const LIMIT: usize = 100000;

fn count_at_most_of_size(directory: &Directory) -> (usize /* my size */, usize /* total */) {
    let mut my_size = 0;
    let mut total = 0;

    for entry in &directory.contents {
        match entry {
            FsEntry::Dir(subdir) => {
                let (subsize, subtotal) = count_at_most_of_size(subdir);
                my_size += subsize;
                total += subtotal;
            }
            FsEntry::File(file) => {
                my_size += file.size;
            }
        }
    }

    if my_size <= LIMIT {
        total += my_size
    }

    (my_size, total)
}

fn smallest_big_enough(directory: &Directory, target: usize) -> (usize, usize) {
    let mut my_size = 0;
    let mut best = usize::MAX;

    for entry in &directory.contents {
        match entry {
            FsEntry::Dir(subdir) => {
                let (subsize, subbest) = smallest_big_enough(&subdir, target);
                my_size += subsize;
                if subbest >= target {
                    best = best.min(subbest);
                }
            }
            FsEntry::File(file) => {
                my_size += file.size;
            }
        }
    }

    if my_size >= target {
        best = best.min(my_size);
    }

    (my_size, best)
}

pub fn run(input: &str) -> (Solution, Solution) {
    let (_, log) = parse_log(input).unwrap();

    let filesystem = build_filesystem(log);

    let (fs_size, result1) = count_at_most_of_size(&filesystem);
    let free_space = 70000000 - fs_size;
    let need_to_delete = 30000000usize.saturating_sub(free_space);

    let result2 = smallest_big_enough(&filesystem, need_to_delete).1;

    (result1.into(), result2.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_expected_output, get_input};

    #[test]
    fn verify() {
        const DAY: u32 = 7;
        let input = get_input(DAY);
        let output = run(&input);
        let expected_output = get_expected_output(DAY);
        assert_eq!(output, expected_output);
    }
}
