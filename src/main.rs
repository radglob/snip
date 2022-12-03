use std::env;
use std::fs;
use std::process::exit;
use std::io::Error;

fn usage() {
    println!("Usage: snip <filename> <start> <end>");
}

fn count_leading_whitespace(s: &str) -> u32 {
    let mut count = 0;
    let mut chars = s.chars();
    while let Some(' ') = chars.next() {
        count += 1;
    }
    return count;
}

fn get_lines(filename: &str) -> Result<Vec<String>, Error> {
    let contents = fs::read_to_string(filename)?;
    let lines: Vec<String> = contents.lines().map(str::to_string).collect();
    Ok(lines)
}

fn parse_bounds(bounds_str: &str) -> (usize, usize) {
    let line_numbers: Vec<usize> = bounds_str.split(" ").map(|s| { s.parse::<usize>().expect("There was an issue parsing the number") }).collect();
    assert!(line_numbers[0] < line_numbers[1]);
    (line_numbers[0], line_numbers[1])
}

fn calculate_min_whitespace(lines: &[String]) -> usize {
    let leading_whitespaces = lines.into_iter().map(|s| count_leading_whitespace(s));
    match leading_whitespaces.min() {
        Some(min) => min as usize,
        None => 0
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        usage();
        exit(128);
    } else {
        let lines = get_lines(&args[0]).expect("There was an issue parsing lines from the file");
        let (start_line, end_line) = parse_bounds(&args[1]);
        let selected_lines = &lines[start_line..end_line];
        let padding = calculate_min_whitespace(selected_lines);
        let trimmed_lines = selected_lines.into_iter().map(|s| &s[padding..]);
        for line in trimmed_lines {
            println!("{}", line);
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bounds() {
        assert_eq!(parse_bounds("0 9"), (0, 9));
    }

    #[test]
    #[should_panic]
    fn test_parse_bounds_bad_order() {
        parse_bounds("20 1");
    }

    #[test]
    #[should_panic]
    fn test_parse_bounds_negative_numbers() {
        parse_bounds("0 -20");
    }

    #[test]
    fn test_count_leading_whitespace() {
        assert_eq!(count_leading_whitespace("    a"), 4);
        assert_eq!(count_leading_whitespace(""), 0);
        assert_eq!(count_leading_whitespace("apples    "), 0);
    }

    #[test]
    fn test_calculate_min_whitespace() {
        let lines = vec![
            String::from("apples and bananas"),
            String::from("   are my favorite fruits"),
            String::from(" and everyone knows this ")
        ];
        assert_eq!(calculate_min_whitespace(&lines), 0);
    }
}
