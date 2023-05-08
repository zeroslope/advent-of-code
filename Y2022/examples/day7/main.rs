use regex::Regex;
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{BufRead, Cursor},
    rc::{Rc, Weak},
};

struct File {
    name: String,
    size: u64,
    parent: Weak<RefCell<Folder>>,
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("name", &self.name)
            .field("size", &self.size)
            .finish()
    }
}

struct Folder {
    name: String,
    size: u64,
    folders: Vec<Rc<RefCell<Folder>>>,
    files: Vec<Rc<RefCell<File>>>,
    parent: Weak<RefCell<Folder>>,
}

impl Debug for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Folder")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("folders", &self.folders)
            .field("files", &self.files)
            .finish()
    }
}

impl Folder {
    fn root() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(Folder {
                name: "/".to_owned(),
                size: 0,
                folders: vec![],
                files: vec![],
                parent: me.clone(),
            })
        })
    }

    fn new(name: String, parent: Weak<RefCell<Folder>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Folder {
            name,
            size: 0,
            folders: vec![],
            files: vec![],
            parent,
        }))
    }
}

fn calc_folder_size(cur: Rc<RefCell<Folder>>) {
    for folder in cur.borrow().folders.iter().cloned() {
        calc_folder_size(folder);
    }

    let mut size = 0;
    for folder in cur.borrow().folders.iter() {
        size += folder.borrow().size;
    }
    for file in cur.borrow().files.iter() {
        size += file.borrow().size;
    }

    cur.borrow_mut().size = size;
}

fn get_target_size(cur: Rc<RefCell<Folder>>, size: &mut u64) {
    let cur_size = cur.borrow().size;
    if cur_size <= 100000 {
        *size += cur_size;
    }
    for folder in cur.borrow().folders.iter().cloned() {
        get_target_size(folder, size);
    }
}

fn get_delete_size(cur: Rc<RefCell<Folder>>, size: &mut u64, need_size: u64) {
    let cur_size = cur.borrow().size;
    if cur_size > need_size && cur_size < *size {
        *size = cur_size;
    }
    for folder in cur.borrow().folders.iter().cloned() {
        get_delete_size(folder, size, need_size)
    }
}

fn main() {
    let ls = Regex::new(r"\$ ls").unwrap();
    let cd = Regex::new(r"\$ cd (.+)").unwrap();

    let f = include_str!("./input.txt");
    let cursor = Cursor::new(f);
    let mut lines = cursor.lines().map(|l| l.unwrap()).peekable();
    // skip "$ cd /"
    lines.next();

    let root = Folder::root();
    let mut cur = Rc::clone(&root);
    while lines.peek().is_some() {
        let cmd = lines.next().unwrap();
        if ls.is_match(&cmd) {
            while {
                let line = lines.peek().map(|l| l.as_bytes().first());
                line.is_some() && line != Some(Some(&b'$'))
            } {
                let info = lines.next().unwrap();
                let mut info = info.split_ascii_whitespace();
                let first = info.next().unwrap();
                let second = info.next().unwrap();
                if first == "dir" {
                    let folder = Folder::new(second.to_owned(), Rc::downgrade(&cur));
                    cur.borrow_mut().folders.push(folder);
                } else {
                    let file = Rc::new(RefCell::new(File {
                        name: second.to_owned(),
                        size: first.parse::<u64>().unwrap(),
                        parent: Rc::downgrade(&cur),
                    }));
                    cur.borrow_mut().files.push(file);
                }
            }
        } else if let Some(caps) = cd.captures(&cmd) {
            let dir = caps.get(1).unwrap().as_str();
            if dir == ".." {
                let parent = cur.borrow().parent.upgrade().unwrap();
                cur = parent;
            } else {
                let children = cur
                    .borrow()
                    .folders
                    .iter()
                    .find(|ch| ch.borrow().name == dir)
                    .cloned()
                    .unwrap();
                cur = children;
            }
        }
    }

    calc_folder_size(root.clone());

    let mut part1 = 0;
    get_target_size(root.clone(), &mut part1);
    dbg!(part1);

    let need_size = 30000000 - (70000000 - root.borrow().size);
    let mut part2 = 70000000;
    get_delete_size(root.clone(), &mut part2, need_size);
    dbg!(part2);
}
