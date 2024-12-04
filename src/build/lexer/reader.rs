#[derive(Debug, Copy, Clone)]
pub struct Char {
    pub char: char,
    pub column: usize,
    pub line: usize,
}
impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}, column: {}, line: {}",
            self.char, self.column, self.line
        )
    }
}

#[derive(Debug)]
pub struct Reader {
    pub lines: Vec<String>,
    pub chars: Vec<Char>,
    pub pos: usize,
}

impl Reader {
    pub fn new(input: String) -> Self {
        let mut output = Vec::with_capacity(input.len());
        let mut input: Vec<char> = input.chars().collect();
        input.reverse();

        let mut lines: Vec<String> = Vec::new();
        let mut line_string: String = String::new();

        let mut line: usize = 1;
        let mut column: usize = 0;

        loop {
            let char = match input.pop() {
                Some(char) => char,
                None => break,
            };

            match char {
                '\r' => continue,
                '\n' => {
                    lines.push(std::mem::take(&mut line_string));
                    line += 1;
                    column = 0;
                }
                '\t' => {
                    column += 4;
                    line_string.push_str("    ");
                }
                ch => {
                    line_string.push(ch);
                    column += 1;
                }
            }
            output.push(Char { char, column, line });
        }
        lines.push(line_string);
        output.reverse();
        Self {
            lines,
            chars: output,
            pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<Char> {
        self.pos += 1;
        self.chars.pop()
    }

    pub fn peek(&mut self) -> Option<&Char> {
        self.chars.last()
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let char: &Char = match self.peek() {
                Some(tk) => tk,
                None => {
                    break;
                }
            };
            if !char.char.is_whitespace() {
                break;
            } else {
                self.next().unwrap();
            }
        }
    }

    pub fn is_letter(ch: Char) -> bool {
        'a' <= ch.char && ch.char <= 'z' || 'A' <= ch.char && ch.char <= 'Z' || ch.char == '_'
    }

    pub fn is_number(ch: Char) -> bool {   
     '0' <= ch.char && ch.char <= '9'
    }
}
