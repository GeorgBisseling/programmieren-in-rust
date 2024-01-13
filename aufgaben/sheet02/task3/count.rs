fn main() {
    println!("{}", count("peter", 'e'));
}

fn count(s : &str, c : char) -> usize {
   return Iterator::count(s.chars().filter(|cc| {*cc == c}));
}