use crate::error::MyCustomError;
use std::collections::HashSet;
use log::{debug};

pub struct FormatParser<'a> {
    pub input: std::iter::Peekable<std::str::Chars<'a>>,
    pub nodes: Vec<Node>,
    pub keywords: HashSet<char>,

    pub holes: usize,
    sum: u32,
    count: u32,
}

#[derive(PartialEq, Debug)]
pub enum Node {
    Hole(u32),
    Year,
    YearShort,
    Day,
    Literal(String),

    // Month.
    Feb,
    FebCaps,
    FebAllCaps,
    FebNum,
    February,
    FebruaryCaps,
    FebruaryAllCaps,
}

type Result<T> = std::result::Result<(), T>;

impl FormatParser<'_> {
    pub fn new(s: &str) -> FormatParser {
        let mut kw = HashSet::new();
        kw.insert('%');
        kw.insert('0');
        kw.insert('2');
        kw.insert('F');
        kw.insert('f');
        return FormatParser {
            input: s.chars().peekable(),
            nodes: Vec::new(),
            keywords: kw,
            holes: s.matches('%').count(),
            sum: 0,
            count: 0,
        };
    }

    pub fn parse(&mut self) -> Result<MyCustomError> {
        let res: Result<MyCustomError> = loop {
            match self.input.peek() {
                Some(x) if *x == '%' => self.parse_hole()?,
                Some(x) if *x == '0' => self.parse_date()?,
                Some(x) if *x == 'F' || *x == 'f' => self.parse_month()?,
                Some(x) if *x == '2' => self.parse_year_long()?,
                Some(_) => self.parse_literal()?,
                None => break Ok(()),
            };
        };
        if self.count > 0 && self.sum != (self.count) * (self.count - 1) / 2 {
            return Err(MyCustomError::ParseError(
                "holes have to start at 0 and must not have gaps".to_string(),
            ));
        }
        return res;
    }

    fn parse_month(&mut self) -> Result<MyCustomError> {
        let mut is_all_caps = true;
        let mut is_cap = false;
        let mut first = true;
        let mut want = "feb";
        for c in want.chars() {
            match self.input.peek() {
                Some(a) if (*a).to_lowercase().collect::<String>() == c.to_string() => {
                    if first {
                        if *a == 'F' {
                            is_cap = true
                        }
                        first = false;
                    }
                    if (*a).is_lowercase() {
                        is_all_caps = false;
                    }
                    self.input.next();
                }
                Some(_) => return Err(MyCustomError::ParseError("".to_string())),
                None => return Err(MyCustomError::ParseError("".to_string())),
            };
        }
        want = "ruary";
        first = true;
        for c in want.chars() {
            match self.input.peek() {
                Some(a) if (*a).to_lowercase().collect::<String>() == c.to_string() => {
                    first = false;
                    if (*a).is_lowercase() {
                        is_all_caps = false;
                    }
                    self.input.next();
                }
                Some(_) => {
                    if first {
                        if is_all_caps {
                            self.nodes.push(Node::FebAllCaps);
                        } else if is_cap {
                            self.nodes.push(Node::FebCaps);
                        } else {
                            self.nodes.push(Node::Feb);
                        };
                        return Ok(());
                    } else {
                        return Err(MyCustomError::ParseError("".to_string()));
                    };
                }
                None => return Err(MyCustomError::ParseError("".to_string())),
            };
        }
        if is_all_caps {
            self.nodes.push(Node::FebruaryAllCaps);
        } else if is_cap {
            self.nodes.push(Node::FebruaryCaps);
        } else {
            self.nodes.push(Node::February);
        };
        return Ok(());
    }

    fn parse_date(&mut self) -> Result<MyCustomError> {
        debug!("parse_date");
        self.input.next();
        match self.input.peek() {
            Some(a) if *a == '3' => {
                self.nodes.push(Node::YearShort);
                self.input.next();
                return Ok(());
            }
            Some(a) if *a == '2' => {
                self.input.next();
                self.nodes.push(Node::FebNum);
                return Ok(());
            }
            Some(a) if *a == '1' => {
                self.input.next();
                self.nodes.push(Node::Day);
                return Ok(());
            }
            Some(_) => Err(MyCustomError::ParseError("".to_string())),
            None => Err(MyCustomError::ParseError("".to_string())),
        }
    }

    fn parse_year_long(&mut self) -> Result<MyCustomError> {
        debug!("parse_year_long");
        let want = "2003";
        for c in want.chars() {
            match self.input.peek() {
                Some(a) if *a == c => {
                    self.input.next();
                    Ok(())
                }
                Some(_) => Err(MyCustomError::ParseError("invalid long year".to_string())),
                None => Err(MyCustomError::ParseError(
                    "invalid long year. The format ended too early".to_string(),
                )),
            }?;
        }
        self.nodes.push(Node::Year);
        return Ok(());
    }

    fn parse_hole(&mut self) -> Result<MyCustomError> {
        debug!("parse hole");
        self.input.next();
        match self.input.peek() {
            Some(x) => match x.to_digit(10) {
                Some(x) => {
                    self.sum += x;
                    self.count += 1;
                    self.nodes.push(Node::Hole(x));
                    self.input.next();
                    return Ok(());
                }
                None => {
                    return Err(MyCustomError::ParseError(format!(
                        "could not parse hole, {} is not a digit",
                        x
                    )))
                }
            },
            None => {
                return Err(MyCustomError::ParseError(
                    "next() did non return a character".to_string(),
                ))
            }
        };
    }

    fn parse_literal(&mut self) -> Result<MyCustomError> {

    debug!("parsing literal");
        let mut literal = String::from("");
        loop {
            match self.input.peek() {
                Some(x) if self.keywords.contains(x) => break,
                Some(x) => {
                    literal.push(*x);
                    self.input.next();
                }
                None => break,
            }
        }
        debug!("literal: {}", literal);
        self.nodes.push(Node::Literal(literal));
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::format_parser::{FormatParser, Node};
    #[test]
    fn test_year_long() {
        let format_string = String::from("2003");
        let want = vec![Node::Year];
        let mut p = FormatParser::new(&format_string);
        assert!(p.parse().is_ok());
        for (i, n) in want.iter().enumerate() {
            assert_eq!(&p.nodes[i], n)
        }
    }
    #[test]
    fn test_year_short() {
        let format_string = String::from("03");
        let want = vec![Node::YearShort];
        let mut p = FormatParser::new(&format_string);
        assert!(p.parse().is_ok());
        for (i, n) in want.iter().enumerate() {
            assert_eq!(&p.nodes[i], n)
        }
    }
    #[test]
    fn test_mixed_1() {
        let format_string = String::from("030201");
        let want = vec![Node::YearShort, Node::FebNum, Node::Day];
        let mut p = FormatParser::new(&format_string);
        assert!(p.parse().is_ok());
        for (i, n) in want.iter().enumerate() {
            assert_eq!(&p.nodes[i], n)
        }
    }
    #[test]
    fn test_long_year_with_literals() {
        let format_string = String::from("2003-02-01");
        let want = vec![
            Node::Year,
            Node::Literal("-".to_string()),
            Node::FebNum,
            Node::Literal("-".to_string()),
            Node::Day,
        ];
        let mut p = FormatParser::new(&format_string);
        assert!(p.parse().is_ok());
        for (i, n) in want.iter().enumerate() {
            assert_eq!(&p.nodes[i], n)
        }
    }
    #[test]
    fn test_mixes_month() {
        let format_string = String::from("Februaryfebfebruary");
        let want = vec![Node::FebruaryCaps, Node::Feb, Node::February];
        let mut p = FormatParser::new(&format_string);
        assert!(p.parse().is_ok());
        for (i, n) in want.iter().enumerate() {
            assert_eq!(&p.nodes[i], n)
        }
    }
    #[test]
    fn test_year_feb_day() {
        let format_string = String::from("2003-FEB-01");
        let want = vec![
            Node::Year,
            Node::Literal("-".to_string()),
            Node::FebAllCaps,
            Node::Literal("-".to_string()),
            Node::Day,
        ];
        let mut p = FormatParser::new(&format_string);
        assert!(p.parse().is_ok());
        for (i, n) in want.iter().enumerate() {
            assert_eq!(&p.nodes[i], n)
        }
    }
    #[test]
    fn test_year_february_day() {
        let format_string = String::from("2003-february-01");
        let want = vec![
            Node::Year,
            Node::Literal("-".to_string()),
            Node::February,
            Node::Literal("-".to_string()),
            Node::Day,
        ];
        let mut p = FormatParser::new(&format_string);
        assert!(p.parse().is_ok());
        for (i, n) in want.iter().enumerate() {
            assert_eq!(&p.nodes[i], n)
        }
    }
    #[test]
    fn test_invalid_hole() {
        let format_string = String::from("%2004");
        let mut p = FormatParser::new(&format_string);
        assert!(p.parse().is_err());
    }
}
