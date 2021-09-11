use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let path = args.get(1).expect("expected path to file as the first arg");

    eprint!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    eprintln!("reading file {}", path);

    let file = std::fs::read_to_string(&path).unwrap();

    let lines: Vec<&str> = file.split('\n').collect();

    let mut fzf = Fzf::default();

    for x in lines {
        fzf.append(x.to_string());
    }

    let stdin = std::io::stdin();

    let mut input = String::new();
    loop {
        eprintln!("search : ");

        {
            let mut buff = String::new();
            stdin.read_line(&mut buff).unwrap();
            let buff = buff.trim();

            match buff {
                ":c" => input.clear(),
                ":q" => break,
                _ => {
                    if !input.is_empty() {
                        input.push(' ');
                    }
                    input.push_str(&buff);
                }
            }
        }
        eprint!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        eprintln!("searching: {}\n", input);

        let s = fzf.find(&input);
        for (x, ele) in s[0..(std::cmp::min(10, s.len()))].iter().enumerate() {
            eprintln!("\x1b[40m \x1b[37;1m{}: \x1b[0m {}", x, ele);
        }

        eprintln!("");
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
            if x == ' '{
                continue;
            }
            let vec = self.index.entry(x.to_ascii_lowercase()).or_default();
            vec.insert(i);
        }

        self.list.push(term);
    }

    fn find(&self, term: &T) -> Vec<&T> {
        let mut matched_terms = HashMap::<usize, usize>::new();

        for x in term.as_ref().chars() {
            if let Some(vec) = self.index.get(&x.to_ascii_lowercase()) {
                for ele in vec {
                    *(matched_terms.entry(*ele).or_default()) += 1;
                }
            }
        }

        let mut terms: Vec<(&usize, &usize)> = matched_terms.iter().collect();

        terms.sort_by(|first, second| second.1.cmp(first.1));

        terms
            .iter()
            .filter(|(_, matches)| **matches > 0)
            .map(|(ele, _)| &self.list[**ele])
            .collect()
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
