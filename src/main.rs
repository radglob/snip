use std::env;
use std::fs;
use std::process::exit;

fn usage() {
    println!("snip <filename> <start>-<end>");
}

fn count_leading_whitespace(s: &str) -> u32 {
    let mut count = 0;
    let mut chars = s.chars();
    while let Some(' ') = chars.next() {
        count += 1;
    }
    return count;
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        usage();
        exit(128);
    } else {
        let line_numbers: Vec<usize> = args[1].split("-").map(|s| { s.parse::<usize>().expect("There was an issue parsing the number") }).collect();
        let (start_line, end_line) = (line_numbers[0] - 1, line_numbers[1] - 1);
        let filename = &args[0];
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let lines: Vec<&str> = contents.lines().collect();
        let selected_lines = &lines[start_line..end_line];
        let leading_whitespaces = selected_lines.into_iter().map(|s| count_leading_whitespace(s));
        let min_leading_whitespace: usize = match leading_whitespaces.min() {
            Some(min) => min as usize,
            None => 0
        };
        let trimmed_lines = selected_lines.into_iter().map(|s| &s[min_leading_whitespace..]);
        for line in trimmed_lines {
            println!("{}", line);
        };
    }
}

#[cfg(test)]
mod tests {
    fn snipping_file_correctly() {}
}
