use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let path = args.get(1).expect("expected path to file as the first arg");

    print!("reading file {}", path);
    let file = std::fs::read_to_string(&path).unwrap();

    let lines: Vec<&str> = file.split('\n').collect();

    let mut fzf = Fzf::default();

    for x in lines {
        fzf.append(x.to_string());
    }

    let stdin = std::io::stdin();

    loop {
        let mut input = String::new();
        println!("search : ");
        stdin.read_line(&mut input).unwrap();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let s = fzf.find(&input);
        for ele in s {
            println!("{}", ele);
        }
    }
}

#[derive(Default)]
struct Fzf<T: AsRef<str>> {
    list: Vec<T>,
    index: HashMap<char, HashSet<usize>>,
}

impl<T: AsRef<str>> Fzf<T> {
    fn append(&mut self, term: T) {
        let i = self.list.len();

        for x in term.as_ref().chars() {
            let vec = self.index.entry(x).or_default();
            vec.insert(i);
        }
        self.list.push(term);
    }

    fn find(&self, term: &T) -> Vec<&T> {
        let mut matched_terms = HashMap::<usize, usize>::new();

        for x in term.as_ref().chars() {
            if let Some(vec) = self.index.get(&x) {
                for ele in vec {
                    *(matched_terms.entry(*ele).or_default()) += 1;
                }
            }
        }

        let mut terms: Vec<(&usize, &usize)> = matched_terms.iter().collect();

        terms.sort_by(|first, second| second.1.cmp(first.1));

        terms.iter().map(|(ele, _)| &self.list[**ele]).collect()
    }
}

impl<T: AsRef<str>> Deref for Fzf<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl<T: AsRef<str>> DerefMut for Fzf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}
