use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Item {
    File(String, usize),
    Directory(String, Vec<Item>),
}

fn insert(parent: &mut Item, path: &str, new: &Item) {
    if path.is_empty() {
        if let Item::Directory(_, dir_items) = parent {
            dir_items.push(new.clone());
        }
        return;
    }
    let mut path_parts = path.split('/');
    let next_dir = path_parts.nth(1).unwrap();
    let remaining_path = {
        let s = path_parts.collect::<String>();
        if s.is_empty() {
            s
        } else {
            format!("/{s}")
        }
    };
    if let Item::Directory(_, dir_items) = parent {
        if let Some(next) = dir_items.iter_mut().find(|d| match d {
            Item::Directory(dir_name, _) => dir_name == next_dir,
            Item::File(..) => false,
        }) {
            insert(next, &remaining_path, new);
        } else {
            let next = Item::Directory(next_dir.to_owned(), Vec::new());
            dir_items.push(next);
            insert(dir_items.last_mut().unwrap(), &remaining_path, new);
        }
    }
}

fn construct(text: &str) -> Item {
    let mut root = Item::Directory(String::new(), Vec::new());
    let mut pwd = String::new();

    for line in text.split_terminator('\n') {
        if line.starts_with("$ cd") {
            if line.chars().nth(5).unwrap() == '/' {
                pwd = String::new();
                continue;
            }
            let path = line.chars().skip(5).collect::<String>();
            if path == ".." {
                let mut v = pwd.split('/').collect::<Vec<_>>();
                let _ = v.pop();
                pwd = v.join("/");
            } else {
                pwd = pwd + "/" + &path;
            }
            continue;
        }
        if line.starts_with("dir") || line.starts_with('$') {
            continue;
        }
        let (size, file) = line.split_once(' ').unwrap();
        insert(
            &mut root,
            &pwd,
            &Item::File(file.to_string(), size.parse().unwrap()),
        );
    }

    root
}

fn find_size(root: &Item, max_size: usize) -> Vec<usize> {
    let mut sizes: HashMap<String, usize> = HashMap::new();

    if let Item::Directory(_, dir_items) = root {
        //
    }

    sizes
        .iter()
        .filter(|(_key, &value)| value <= max_size)
        .map(|(_key, value)| *value)
        .collect()
}

pub fn run(text: &str) {
    let fs = construct(text);
    let dirs = find_size(&fs, 100_000);
}
