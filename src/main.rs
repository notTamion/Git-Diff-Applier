use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let patch = fs::read_to_string(&args[1]).expect(format!("Wasn't able to read file {}", args[1]).as_str());

    let mut simplified_patch = String::new();
    for line in patch.lines() {
        match line.chars().next().unwrap_or(' ') {
            '+' => {
                let mut s = line.to_string();
                s.remove(0);
                simplified_patch.push_str(format!(" {}\n", s.as_str()).as_str())
            },
            '-' => {}
            _ => simplified_patch.push_str(format!("{}\n", line).as_str())
        }
    }

    println!("{}", simplified_patch)
}
