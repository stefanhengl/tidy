use crate::constants as c;
use crate::format_parser::Node;
use crate::error::{MyCustomError};

// Internal representation.
pub struct Data {
    pub year: Option<Year>,
    pub month: Option<c::Months>,
    pub day: Option<String>,
    pub holes: Vec<String>,
}

pub struct FileNameParser<'b> {
    pub name: std::iter::Peekable<std::str::Chars<'b>>,
    pub d: Data,
}

type Result<T> = std::result::Result<(), T>;

type Year = [char; 4];

impl FileNameParser<'_> {
    pub fn new(s: &str, num_holes: usize) -> FileNameParser {
        return FileNameParser {
            name: s.chars().peekable(),
            d: Data {
                year: None,
                month: None,
                day: None,
                holes: vec![String::from(""); num_holes],
            },
        };
    }
    pub fn parse(&mut self, template_nodes: &Vec<Node>) -> Result<MyCustomError> {
        let mut i: usize = 0;
        let mut nodes = template_nodes.iter().peekable();
        loop {
            i = i + 1;
            match nodes.next() {
                Some(n) => match n {
                    Node::Year => self.parse_year(4)?,
                    Node::YearShort => self.parse_year(2)?,
                    Node::FebNum => self.parse_month(2)?,
                    Node::Feb => self.parse_month(3)?,
                    Node::FebAllCaps => self.parse_month(3)?,
                    Node::FebCaps => self.parse_month(3)?,
                    Node::February => self.parse_month_flex(stop_char(nodes.peek()))?,
                    Node::FebruaryAllCaps => self.parse_month_flex(stop_char(nodes.peek()))?,
                    Node::FebruaryCaps => self.parse_month_flex(stop_char(nodes.peek()))?,
                    Node::Day => self.parse_day(2)?,
                    Node::Hole(i) => self.parse_hole(*i, stop_char(nodes.peek()))?,
                    Node::Literal(s) => self.parse_literal(s)?,
                },
                None => {
                    if self.name.peek() == None {
                        return Ok(());
                    }
                    return Err(MyCustomError::ParseError("filename does not match pattern".to_string()));
                }
            };
        }
    }

    fn parse_year(&mut self, i: u32) -> Result<&'static str> {
        if i != 2 && i != 4 {
            return Err("supported formats for year are 08 and 2008");
        }
        let mut year: Year = ['0'; 4];
        let mut cursor: usize = 0;
        if i == 2 {
            cursor += 2;
            year[0] = '2';
            year[1] = '0';
        }
        let mut err: &str = "";
        for _ in 0..i {
            match self.name.next() {
                Some(x) => {
                    if x.is_numeric() {
                        year[cursor]=x;
                        cursor+=1;
                        continue;
                    }
                    return Err("character can not be converted to a numeric");
                }
                None => err = "template does not match string. Most likely, the template contains too many placeholders",
            }
        }
        if err != "" {
            return Err(err);
        }
        self.d.year = Some(year);
        return Ok(());
    }

    fn parse_month(&mut self, i: u32) -> Result<&'static str> {
        let mut month = String::from("");
        let mut err: &str = "";
        for _ in 0..i {
            match self.name.next() {
                Some(x) => month.push(x),
                None => err = "This should not happen",
            };
        }
        if err != "" {
            return Err(err);
        }
        if c::TOIR.contains_key(&month[..]) {
            self.d.month = Some(c::TOIR[&month[..]]);
            return Ok(());
        }
        Err("unknown month")
    }

    fn parse_month_flex(&mut self, stop: char) -> Result<&'static str> {
        let mut month = String::from("");
        loop {
            match self.name.peek() {
                Some(x) if *x == stop => break,
                Some(x) => {
                    month.push(*x);
                    self.name.next();
                }
                None => break,
            }
        }
        let lc = month.to_lowercase();
        if c::TOIR.contains_key(&lc[..]) {
            self.d.month = Some(c::TOIR[&lc[..]]);
            return Ok(());
        }
        Err("unknown month")
    }

    fn parse_day(&mut self, i: u32) -> Result<&'static str> {
        let mut day = String::from("");
        let mut err: &str = "";
        for _ in 0..i {
            match self.name.next() {
                Some(x) => day.push(x),
                None => err = "This should not happen",
            };
        }
        if err != "" {
            return Err(err);
        }
        self.d.day = Some(day);
        Ok(())
    }

    // Literals of the original string are ignored.
    fn parse_literal(&mut self, s: &str) -> Result<&'static str> {
        if s.len() == 0 {
            return Err("empty literal");
        }
        for _ in 0..s.len() {
            self.name.next();
        }
        Ok(())
    }

    fn parse_hole(&mut self, i: u32, stop: char) -> Result<&'static str> {
        let mut hole = String::from("");
        loop {
            match self.name.peek() {
                Some(x) if *x == stop => break Ok(()),
                Some(x) => {
                    hole.push(*x);
                    self.name.next();
                }
                None => {
                    if stop == '$' {
                        break Ok(());
                    }
                    break Err("could not parse hole");
                }
            }
        }?;
        self.d.holes[i as usize] = hole;
        Ok(())
    }
}

fn stop_char(opt: Option<&&Node>) -> char {
    match opt {
        Some(x) => match x {
            Node::Literal(a) => a.chars().next().unwrap(),
            _ => panic!("node cannot provide stop char"),
        },
        None => '$',
    }
}

#[cfg(test)]
mod tests {
    use crate::constants as c;
    use crate::FileNameParser;
    use crate::FormatParser;
    use crate::MyCustomError;
    macro_rules! testify {
        ($name:ident, $($source_template:expr, $s:expr, $want_error:expr),+) => {
            #[test]
            fn $name()->Result<(),MyCustomError>{
                $({
                    let mut source = FormatParser::new($source_template);
                    source.parse()?;

                    let mut parser = FileNameParser::new($s, source.holes);
                    let res = parser.parse(&source.nodes);

                    if $want_error {
                        assert!(res.is_err())
                    } else {
                        assert!(res.is_ok())
                    }
                    Ok(())
                })+
            }
        }
    }

    testify!(error1, "2003", "july", true);
    testify!(error2, "2003", "2020_july", true);
    testify!(error3, "2003", "01.05.1990", true);
    testify!(error4, "20030201", "abc", true);
    testify!(error5, "20030201", "202102", true);
    testify!(error6, "$0_2003", "2030", true);
    testify!(error7, "$0_2003", "hello2030", true);

    testify!(works1, "$0", "hello", false);
    testify!(works2, "$0_$1", "hello_world", false);

    #[test]
    fn test_month_mapping() -> Result<(), MyCustomError> {
        let source_template: String = "february".to_string();
        let s: String = "june".to_string();

        let mut source = FormatParser::new(&source_template);
        source.parse()?;

        let mut parser = FileNameParser::new(&s, 0);
        let res = parser.parse(&source.nodes);

        assert!(res.is_ok());
        assert_eq!(parser.d.month.unwrap(), c::Months::Jun);
        Ok(())
    }

    #[test]
    fn test_month_mapping_lowercase() -> Result<(), MyCustomError> {
        let source_template: String = "february".to_string();
        let s: String = "JuNe".to_string();

        let mut source = FormatParser::new(&source_template);
        source.parse()?;

        let mut parser = FileNameParser::new(&s, 0);
        let res = parser.parse(&source.nodes);

        assert!(res.is_ok());
        assert_eq!(parser.d.month.unwrap(), c::Months::Jun);
        Ok(())
    }
}
