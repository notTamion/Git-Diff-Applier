use std::{env, fs};
use colored::{ColoredString, Colorize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut patch;
    if args[1].starts_with("https://") {
        patch = reqwest::blocking::get(format!("{}.diff", &args[1])).unwrap().text().unwrap();
    } else {
        patch = fs::read_to_string(&args[1]).expect(format!("Wasn't able to read file {}", args[1]).as_str());
    }

    let mut runs = 1;
    if args.len() == 3 {
        runs = args[2].parse().unwrap();
    }

    let mut simplified_patch= String::from("");
    while runs > 1 {
        for line in patch.lines() {
            if line.len() == 0 {
                simplified_patch = format!("{}\n", simplified_patch);
                continue
            }
            match line.chars().next().unwrap() {
                '+' => {
                    let mut s = line.to_string();
                    s.remove(0);
                    simplified_patch = format!("{}{}\n", simplified_patch, s);
                },
                ' ' => {
                    let mut s = line.to_string();
                    s.remove(0);
                    simplified_patch = format!("{}{}\n", simplified_patch, s);
                },
                '-' => {}
                _ => simplified_patch = format!("{}{}\n", simplified_patch, line)
            }
        }
        patch = simplified_patch.to_string();
        runs = runs-1;
    }
    let mut colored_patch = ColoredString::from("");
    for line in patch.lines() {
        if line.len() == 0 {
            colored_patch = format!("{}\n", colored_patch).into();
            continue
        }
        match line.chars().next().unwrap() {
            '+' => {
                let mut s = line.to_string();
                s.remove(0);
                colored_patch = format!("{}{}\n", colored_patch, s.green()).into();
            },
            ' ' => {
                let mut s = line.to_string();
                s.remove(0);
                colored_patch = format!("{}{}\n", colored_patch, s.white()).into();
            },
            '-' => {}
            _ => colored_patch = format!("{}{}\n", colored_patch, line.white()).into()
        }
    }
    println!("{}", colored_patch)
}