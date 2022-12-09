use crate::commit_line::format_commit_line;
    static ref STATIC_HEADER_PREFIXES: Vec<(&'static str, &'static str)> = vec![
        ("diff ", FAINT),
        ("index ", FAINT),
        ("Binary files ", BOLD),
        ("copy from ", FAINT),
        ("copy to ", BOLD),
        ("rename from ", FAINT),
        ("rename to ", BOLD),
        ("similarity index ", FAINT),
        ("new file mode ", FAINT),
        ("deleted file mode ", FAINT),
        ("--- /dev/null", FAINT),
        ("+++ /dev/null", FAINT),
    ];
    static ref ANSI_COLOR_REGEX: Regex = Regex::new("\x1b\\[[0-9;]*[^0-9;]").unwrap();
    diff_seen: bool,
            diff_seen: false,

    fn without_ansi_escape_codes(input: &'_ str) -> std::borrow::Cow<'_, str> {
        return ANSI_COLOR_REGEX.replace_all(input, "");
    }

        let line = LineCollector::without_ansi_escape_codes(&line);

        if line.starts_with("diff") {
            self.diff_seen = true;
        }
        if line.starts_with("commit") {
            self.consume_plain_line(&format_commit_line(&line, self.diff_seen));
            return;
        }

        if line.starts_with("--- ") || line.starts_with("+++ ") {

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_non_sgr() {
        assert_eq!(
            LineCollector::without_ansi_escape_codes("hel\x1b[0Klo"),
            "hello"
        );
    }

    #[test]
    fn test_sgr() {
        assert_eq!(
            LineCollector::without_ansi_escape_codes("hel\x1b[33mlo"),
            "hello"
        );
    }

    #[test]
    fn test_multi_sgr() {
        assert_eq!(
            LineCollector::without_ansi_escape_codes("hel\x1b[33;34mlo"),
            "hello"
        );
    }
}