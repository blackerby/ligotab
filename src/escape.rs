pub(crate) trait Escape {
    fn escape_brackets(&self) -> String;
    fn escape_pipe(&self) -> String;
}

impl Escape for String {
    fn escape_pipe(&self) -> Self {
        self.replace("|", "\\|")
    }

    fn escape_brackets(&self) -> Self {
        self.replace("{", "\\{").replace("}", "\\}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_pipe() {
        let original = "There | are | pipes | here";
        let got = original.to_string().escape_pipe();
        let want = "There \\| are \\| pipes \\| here";

        assert_eq!(got.as_str(), want);
    }

    #[test]
    fn test_escape_brackets() {
        let original = "There { are } brackets here";
        let got = original.to_string().escape_brackets();
        let want = "There \\{ are \\} brackets here";

        assert_eq!(got.as_str(), want);
    }
}
