use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

lazy_static! {
    pub static ref IRTOFEBNUM: HashMap<Months, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Months::Jan, "01");
        m.insert(Months::Feb, "02");
        m.insert(Months::Mar, "03");
        m.insert(Months::Apr, "04");
        m.insert(Months::May, "05");
        m.insert(Months::Jun, "06");
        m.insert(Months::Jul, "07");
        m.insert(Months::Aug, "08");
        m.insert(Months::Sep, "09");
        m.insert(Months::Oct, "10");
        m.insert(Months::Nov, "11");
        m.insert(Months::Dec, "12");
        m
    };
}

lazy_static! {
    pub static ref IRTOFEB: HashMap<Months, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Months::Jan, "jan");
        m.insert(Months::Feb, "feb");
        m.insert(Months::Mar, "mar");
        m.insert(Months::Apr, "apr");
        m.insert(Months::May, "may");
        m.insert(Months::Jun, "jun");
        m.insert(Months::Jul, "jul");
        m.insert(Months::Aug, "aug");
        m.insert(Months::Sep, "sep");
        m.insert(Months::Oct, "oct");
        m.insert(Months::Nov, "nov");
        m.insert(Months::Dec, "dec");
        m
    };
}

lazy_static! {
    pub static ref IRTOFEBRUARY: HashMap<Months, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Months::Jan, "january");
        m.insert(Months::Feb, "february");
        m.insert(Months::Mar, "march");
        m.insert(Months::Apr, "april");
        m.insert(Months::May, "may");
        m.insert(Months::Jun, "june");
        m.insert(Months::Jul, "july");
        m.insert(Months::Aug, "august");
        m.insert(Months::Sep, "september");
        m.insert(Months::Oct, "october");
        m.insert(Months::Nov, "november");
        m.insert(Months::Dec, "december");
        m
    };
}

#[derive(PartialEq, std::cmp::Eq, std::hash::Hash, Copy, Clone, Debug)]
pub enum Months {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl fmt::Display for Months {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Months::Jan => write!(f, "Jan"),
            Months::Feb => write!(f, "Feb"),
            Months::Mar => write!(f, "Mar"),
            Months::Apr => write!(f, "Apr"),
            Months::May => write!(f, "May"),
            Months::Jun => write!(f, "Jun"),
            Months::Jul => write!(f, "Jul"),
            Months::Aug => write!(f, "Aug"),
            Months::Sep => write!(f, "Sep"),
            Months::Oct => write!(f, "Oct"),
            Months::Nov => write!(f, "Nov"),
            Months::Dec => write!(f, "Dec"),
        }
    }
}

lazy_static! {
    pub static ref TOIR: HashMap<&'static str, Months> = {
        let mut m = HashMap::new();
        m.insert("01", Months::Jan);
        m.insert("02", Months::Feb);
        m.insert("03", Months::Mar);
        m.insert("04", Months::Apr);
        m.insert("05", Months::May);
        m.insert("06", Months::Jun);
        m.insert("07", Months::Jul);
        m.insert("08", Months::Aug);
        m.insert("09", Months::Sep);
        m.insert("10", Months::Oct);
        m.insert("11", Months::Nov);
        m.insert("12", Months::Dec);
        m.insert("januar", Months::Jan);
        m.insert("february", Months::Feb);
        m.insert("march", Months::Mar);
        m.insert("april", Months::Apr);
        m.insert("may", Months::May);
        m.insert("june", Months::Jun);
        m.insert("july", Months::Jul);
        m.insert("august", Months::Aug);
        m.insert("september", Months::Sep);
        m.insert("october", Months::Oct);
        m.insert("november", Months::Nov);
        m.insert("december", Months::Dec);
        m
    };
}
