pub fn no_comments(string: &str) -> String {
    use std::fmt::Write;

    let re = fancy_regex::Regex::new(r"(.*)(?!#>)#.*$").unwrap();

    let do_line = |line: &str| -> (bool, String) {
        let caps = re.captures(line);
        if let Ok(Some(caps)) = caps {
            let mat = caps.get(1).unwrap();
            (
                true,
                line[mat.start()..mat.end()].trim_end().to_string(),
            )
        } else {
            (false, line.trim_end().to_string())
        }
    };

    let mut output = String::new();
    for line in string.lines() {
        let mut line = line.to_string();
        loop {
            let (again, result) = do_line(line.as_str());
            if again {
                // Repeat the call to do_line again to make sure that all
                // #-comments are removed (there could be
                // multiple on one line)
                line = result;
            } else {
                writeln!(&mut output, "{result}").unwrap();
                break;
            }
        }
    }
    output
}
