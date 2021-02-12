use std::fmt;

pub struct Formatter<'a, T> {
    f: &'a mut T,
    level: u32,
    indentation: String,
}

impl<'a, T: fmt::Write> fmt::Write for Formatter<'a, T> {
    fn write_str(&mut self, input: &str) -> fmt::Result {
        let input = match input.strip_prefix('\n') {
            Some(s) => s,
            None => return self.f.write_str(input),
        };

        let min = input
            .split('\n')
            .map(|line| line.chars().take_while(char::is_ascii_whitespace).count())
            .filter(|count| *count > 0)
            .min()
            .unwrap_or_default();

        let input = input.trim_end_matches(|c| char::is_ascii_whitespace(&c));

        for line in input.split('\n') {
            if line.len().saturating_sub(min) > 0 {
                for _ in 0..self.level {
                    self.f.write_str(&self.indentation)?;
                }
            }

            if line.len() >= min {
                self.f.write_str(&line[min..])?;
            } else {
                self.f.write_str(&line)?;
            }
            self.f.write_char('\n')?;
        }

        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.write_str(&args.to_string())
    }
}

impl<'a, T: fmt::Write> Formatter<'a, T> {
    pub fn new<S: Into<String>>(f: &'a mut T, indentation: S) -> Self {
        Self {
            f,
            level: 0,
            indentation: indentation.into(),
        }
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    pub fn inc(&mut self, increment: i32) {
        if increment < 0 {
            self.level = self.level.saturating_sub(-increment as u32);
        } else {
            self.level = self.level.saturating_add(increment as u32);
        }
    }

    pub fn indent(&mut self) {
        self.level = self.level.saturating_add(1);
    }

    pub fn dedent(&mut self) {
        self.level = self.level.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn dedent() {
        let mut s = String::new();
        let mut f = Formatter::new(&mut s, "    ");
        write!(
            f,
            r#"
            struct Foo;

            impl Foo {{
                fn foo() {{
                    todo!()
                }}
            }}
            "#,
        )
        .unwrap();
        assert_eq!(
            s,
            "struct Foo;\n\nimpl Foo {\n    fn foo() {\n        todo!()\n    }\n}\n"
        );

        let mut s = String::new();
        let mut f = Formatter::new(&mut s, "    ");
        write!(
            f,
            r#"
            struct Foo;

            impl Foo {{
                fn foo() {{
                    todo!()
                }}
            }}"#,
        )
        .unwrap();
        assert_eq!(
            s,
            "struct Foo;\n\nimpl Foo {\n    fn foo() {\n        todo!()\n    }\n}\n"
        );
    }

    #[test]
    fn indent() {
        let mut s = String::new();
        let mut f = Formatter::new(&mut s, "    ");
        f.indent();
        write!(
            f,
            r#"
            struct Foo;

            impl Foo {{
                fn foo() {{
                    todo!()
                }}
            }}
            "#,
        )
        .unwrap();
        assert_eq!(s, "    struct Foo;\n\n    impl Foo {\n        fn foo() {\n            todo!()\n        }\n    }\n");
    }

    #[test]
    fn inline() {
        let mut s = String::new();
        let mut f = Formatter::new(&mut s, "    ");
        write!(
            f,
            r#"struct Foo;
            fn foo() {{
            }}"#,
        )
        .unwrap();
        assert_eq!(s, "struct Foo;\n            fn foo() {\n            }");
    }
}
