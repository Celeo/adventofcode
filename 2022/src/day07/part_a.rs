use std::collections::HashMap;

use itertools::Itertools;

pub fn run(text: &str) {
    let mut fs: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut pwd = String::new();

    for line in text.split_terminator('\n') {
        if line.starts_with("$ cd") {
            if line.chars().nth(5).unwrap() == '/' {
                pwd = String::from("/");
                continue;
            }
            let path = line.chars().skip(5).collect::<String>();
            if path == ".." {
                pwd = pwd.split('/').take(pwd.matches('/').count() - 1).join("/") + "/";
            } else {
                pwd = pwd + &path + "/";
            }
            continue;
        }
        if line.starts_with("dir") || line.starts_with('$') {
            continue;
        }
        let (size, file) = line.split_once(' ').unwrap();
        let in_path = fs.entry(pwd.clone()).or_insert_with(Vec::new);
        in_path.push((file.to_string(), size.parse().unwrap()));
    }

    let matching: Vec<_> = fs
        .keys()
        .filter(|&path| {
            // TODO need to count nested directories as well
            let in_dir = fs.get(path).unwrap().iter().map(|(_, s)| *s).sum::<usize>();
            in_dir > 100_000
        })
        .collect();

    dbg!(matching);

    todo!()
}
