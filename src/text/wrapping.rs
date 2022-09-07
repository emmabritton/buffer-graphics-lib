#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum WrappingStrategy {
    /// Don't wrap, will draw off screen
    None,
    /// Splits string at column
    AtCol(usize),
    /// Wraps at the first space before column (acts like AtCol if no space is on that line)
    SpaceBeforeCol(usize),
    /// Same as AtCol but adds a hypen if it's splitting a word
    AtColWithHypen(usize),
    /// Chops off the end of string it's over specific length
    Cutoff(usize),
    /// Same as Cutoff but adds an ellipsis
    Ellipsis(usize),
}

impl Default for WrappingStrategy {
    fn default() -> Self {
        WrappingStrategy::None
    }
}

impl WrappingStrategy {
    pub fn wrap(&self, input: &str) -> Vec<String> {
        match self {
            WrappingStrategy::None => vec![input.to_string()],
            WrappingStrategy::AtCol(col) => {
                let mut output = vec![];
                let mut text = input.to_string();
                while text.chars().count() > *col {
                    output.push(text.chars().take(*col).collect());
                    text = text.chars().skip(*col).collect();
                }
                output.push(text);
                output
            }
            WrappingStrategy::SpaceBeforeCol(col) => {
                let mut output = vec![];
                let mut text = input.to_string();
                while text.chars().count() > *col {
                    let chars: Vec<char> = text.chars().collect();
                    let line: String = text.chars().take(*col).collect();
                    if line.ends_with(|c: char| c.is_whitespace())
                        || text
                            .chars()
                            .skip(*col)
                            .next()
                            .and_then(|c| Some(c.is_whitespace()))
                            .unwrap_or(false)
                    {
                        output.push(text.chars().take(*col).collect());
                        text = text.chars().skip(*col).collect();
                    } else {
                        let mut whitespace = chars
                            .iter()
                            .take(*col)
                            .rposition(|c| c.is_whitespace())
                            .unwrap_or(0);
                        if whitespace == 0 {
                            whitespace = *col
                        }
                        output.push(text.chars().take(whitespace).collect());
                        text = text.chars().skip(whitespace).collect();
                    }
                }
                output.push(text);
                output.iter().map(|s| s.trim().to_string()).collect()
            }
            WrappingStrategy::AtColWithHypen(col) => {
                let mut output = vec![];
                let mut text = input.to_string();
                while text.chars().count() > *col {
                    let line: String = text.chars().take(*col).collect();
                    if line.chars().last().unwrap_or(' ').is_alphabetic()
                        && text.chars().next().unwrap_or(' ').is_alphabetic()
                    {
                        output.push(format!("{}-", line));
                    } else {
                        output.push(line);
                    }
                    text = text.chars().skip(*col).collect();
                }
                output.push(text);
                output
            }
            WrappingStrategy::Cutoff(col) => vec![input.chars().take(*col).collect()],
            WrappingStrategy::Ellipsis(col) => {
                if input.chars().count() >= *col {
                    vec![format!("{}…", input.chars().take(*col).collect::<String>())]
                } else {
                    vec![input.to_string()]
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use WrappingStrategy::*;

    fn c(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn none() {
        assert_eq!(None.wrap("this is a test"), c(&["this is a test"]));
    }

    #[test]
    fn at_col() {
        assert_eq!(
            AtCol(4).wrap("some words, and some are longer"),
            c(&["some", " wor", "ds, ", "and ", "some", " are", " lon", "ger"])
        );
    }

    #[test]
    fn at_col_hypen() {
        assert_eq!(
            AtCol(4).wrap("some words, and some are longer"),
            c(&["some", " wor", "ds, ", "and ", "some", " are", " lon", "ger"])
        );
    }

    #[test]
    fn space_before_col() {
        assert_eq!(
            SpaceBeforeCol(4).wrap("some words, and some are longer"),
            c(&["some", "wor", "ds,", "and", "some", "are", "lon", "ger"])
        );
        assert_eq!(
            SpaceBeforeCol(8).wrap("some words, and some are longer"),
            c(&["some", "words,", "and some", "are", "longer"])
        );
    }

    #[test]
    fn cutoff() {
        assert_eq!(Cutoff(30).wrap("short test"), c(&["short test"]));
        assert_eq!(Cutoff(10).wrap("longer test string"), c(&["longer tes"]));
    }

    #[test]
    fn ellipsis() {
        assert_eq!(Ellipsis(30).wrap("short test"), c(&["short test"]));
        assert_eq!(Ellipsis(10).wrap("longer test string"), c(&["longer tes…"]));
    }
}
