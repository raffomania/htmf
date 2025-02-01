pub fn write_escaped_html<W>(writer: &mut W, input: &str)
where
    W: std::fmt::Write,
{
    for char in input.chars() {
        #[allow(unused_must_use)]
        match char {
            '<' => writer.write_str("&lt;"),
            '>' => writer.write_str("&gt;"),
            '&' => writer.write_str("&amp;"),
            '"' => writer.write_str("&quot;"),
            '\'' => writer.write_str("&#x27;"),
            _ => writer.write_char(char),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn escape_string(input: &str) -> String {
        let mut res = String::new();
        write_escaped_html(&mut res, input);
        res
    }

    #[test]
    fn base() {
        let escaped = escape_string("<>&\"'");
        assert_eq!("&lt;&gt;&amp;&quot;&#x27;", escaped);
    }

    #[test]
    fn no_changes_to_other_text() {
        let input =
            "This is some standard text, and tailwind classes: mx-8 dark:bg-slate-800 top-[117px]";
        let escaped = escape_string(input);
        assert_eq!(input, escaped);
    }
}
