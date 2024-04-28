use std::{env, fs};
use colored::{ColoredString, Colorize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let patch = fs::read_to_string(&args[1]).expect(format!("Wasn't able to read file {}", args[1]).as_str());

    let mut simplified_patch = ColoredString::from("");
    for line in patch.lines() {
        match line.chars().next().unwrap_or(' ') {
            '+' => {
                let mut s = line.to_string();
                s.remove(0);
                simplified_patch = format!("{} {}\n", simplified_patch, s.as_str()).green();
            },
            '-' => {}
            _ => simplified_patch = format!("{}{}\n", simplified_patch, line).white()
        }
    }

    println!("{}", simplified_patch)
}
