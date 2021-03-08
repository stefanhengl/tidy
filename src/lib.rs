use crate::constants as c;
use crate::filename_parser::FileNameParser;
use crate::format_parser::FormatParser;
use ansi_term::Colour::{Green, Red, Yellow};
use convert_case::{Case, Casing};
use error::MyCustomError;
use log::debug;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

mod constants;
mod error;
mod filename_parser;
mod format_parser;

pub struct RunOpt {
    pub simulate: bool,
    pub force: bool,
    pub review: bool,
}

pub fn run(
    source_template: &str,
    target_template: &str,
    dir: &str,
    opt: RunOpt,
) -> Result<(), MyCustomError> {
    let mut source = FormatParser::new(source_template);
    source.parse()?;
    debug!("parsing source success!");

    let mut target = FormatParser::new(target_template);
    target.parse()?;
    debug!("parsing target success!");

    let mut i: i32 = 0;
    let entries = WalkDir::new(dir);

    debug!("looping over files");
    for entry in entries
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        debug!("file: {}", entry.path().display());
        let new_filename_no_ext = match process(&source, &target, &stem_for_entry(&entry)) {
            Ok(x) => x,
            Err(x) => {
                if opt.force {
                    print!(
                        "{}",
                        Yellow.paint(format!(
                            "WARN: skipping processing of file {} because of error: {}\n",
                            entry.path().to_str().unwrap(),
                            x
                        ))
                    );
                    continue;
                } else {
                    print!(
                        "{}",
                        Red.paint(format!(
                            "ERR: aborting processing. Error in file {}\n",
                            entry.path().to_str().unwrap()
                        ))
                    );
                    return Err(x);
                }
            }
        };
        let p = entry.path();
        let a = p.to_str().ok_or(MyCustomError::PathError)?;
        let ext = match entry.path().extension() {
            Some(x) => x.to_str(),
            None => None,
        }
        .ok_or(MyCustomError::Default("extension error".to_string()))?;
        let b = p
            .parent()
            .ok_or(MyCustomError::PathError)?
            .join(Path::new(&(new_filename_no_ext + "." + ext)));
        i = i + 1;
        if opt.simulate {
            print!(
                "{}\n{}\n\n",
                Red.paint(format!("-{}", &a)),
                Green.paint(format!("+{}", &b.to_str().unwrap()))
            );
            continue;
        }
        if opt.review == true {
            print!("\x1B[2J\x1B[1;1H");
            print!(
                "{}\n{}\n\n",
                Red.paint(format!("-{}", &a)),
                Green.paint(format!("+{}", &b.to_str().unwrap()))
            );
            match ask_user_input() {
                UserInput::Abort => {
                    break;
                }
                UserInput::Accept => (),
                UserInput::Skip => {
                    continue;
                }
            }
        }
        fs::rename(a, b)?;
    }
    return Ok(());
}

enum UserInput {
    Abort,
    Accept,
    Skip,
}

fn ask_user_input() -> UserInput {
    print!("(a) accept, (x) abort, (S/s) skip\n");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    match &input[..] {
        "x\n" => UserInput::Abort,
        "a\n" => UserInput::Accept,
        "s\n" => UserInput::Skip,
        "\n" => UserInput::Skip,
        _ => UserInput::Abort,
    }
}

fn stem_for_entry(entry: &walkdir::DirEntry) -> String {
    return String::from(entry.path().file_stem().unwrap().to_str().unwrap());
}

fn process(
    template: &FormatParser,
    target: &FormatParser,
    s: &str,
) -> Result<String, MyCustomError> {
    let mut f = FileNameParser::new(s, template.holes);
    f.parse(&template.nodes)?;

    let mut nodes = target.nodes.iter().peekable();

    let mut res: String = String::from("");
    loop {
        match nodes.next() {
            Some(n) => match n {
                format_parser::Node::Year => {
                    res = format!(
                        "{}{}",
                        res,
                        f.d.year.as_ref().unwrap().iter().collect::<String>()
                    )
                }
                format_parser::Node::YearShort => {
                    res = format!(
                        "{}{}",
                        res,
                        f.d.year.as_ref().unwrap()[2..].iter().collect::<String>()
                    )
                }
                format_parser::Node::FebNum => {
                    res = format!("{}{}", res, c::IRTOFEBNUM[f.d.month.as_ref().unwrap()])
                }
                format_parser::Node::Feb => {
                    res = format!("{}{}", res, c::IRTOFEB[f.d.month.as_ref().unwrap()])
                }
                format_parser::Node::FebCaps => {
                    res = format!(
                        "{}{}",
                        res,
                        c::IRTOFEB[f.d.month.as_ref().unwrap()]
                            .to_string()
                            .to_case(Case::Title)
                    )
                }
                format_parser::Node::FebAllCaps => {
                    res = format!(
                        "{}{}",
                        res,
                        c::IRTOFEB[f.d.month.as_ref().unwrap()]
                            .to_string()
                            .to_case(Case::Upper)
                    )
                }
                format_parser::Node::February => {
                    res = format!("{}{}", res, c::IRTOFEBRUARY[f.d.month.as_ref().unwrap()])
                }
                format_parser::Node::FebruaryCaps => {
                    res = format!(
                        "{}{}",
                        res,
                        c::IRTOFEBRUARY[f.d.month.as_ref().unwrap()]
                            .to_string()
                            .to_case(Case::Title)
                    )
                }
                format_parser::Node::FebruaryAllCaps => {
                    res = format!(
                        "{}{}",
                        res,
                        c::IRTOFEBRUARY[f.d.month.as_ref().unwrap()]
                            .to_string()
                            .to_case(Case::Upper)
                    )
                }
                format_parser::Node::Day => res = format!("{}{}", res, f.d.day.as_ref().unwrap()),
                format_parser::Node::Hole(h) => res = format!("{}{}", res, f.d.holes[*h as usize]),
                format_parser::Node::Literal(l) => res = format!("{}{}", res, l),
            },
            None => break,
        }
    }
    return Ok(res);
}

#[cfg(test)]
mod tests {
    use crate::process;
    use crate::FormatParser;
    use crate::MyCustomError;
    macro_rules! testify {
        ($name:ident, $($source:expr, $target:expr, $filename:expr, $want:expr),+) => {
            #[test]
            fn $name()->Result<(),MyCustomError>{
                $({
                    let source_template = String::from($source);
                    let target_template = String::from($target);
                    let s = String::from($filename);
                    let want = String::from($want);
                    let mut source = FormatParser::new(&source_template);
                    source.parse()?;
                    let mut target = FormatParser::new(&target_template);
                    target.parse()?;
                    let got = process(&source, &target, &s)?;
                    assert_eq!(got, want);
                    Ok(())
                })+
            }
        }
    }
    testify!(febnum, "20030201", "01_02_2003", "20210601", "01_06_2021");
    testify!(
        to_short_year,
        "20030201",
        "01_02_03",
        "20210601",
        "01_06_21"
    );
    testify!(
        from_short_year,
        "030201",
        "01_02_2003",
        "210601",
        "01_06_2021"
    );
    testify!(short, "20030201", "01_feb_2003", "20210601", "01_jun_2021");
    testify!(
        short_title,
        "20030201",
        "01_Feb_2003",
        "20210601",
        "01_Jun_2021"
    );
    testify!(
        short_all_caps,
        "20030201",
        "01_FEB_2003",
        "20210601",
        "01_JUN_2021"
    );
    testify!(
        long,
        "20030201",
        "01_february_2003",
        "20210601",
        "01_june_2021"
    );
    testify!(
        long_title,
        "20030201",
        "01_February_2003",
        "20210601",
        "01_June_2021"
    );
    testify!(
        long_all_caps,
        "20030201",
        "01_FEBRUARY_2003",
        "20210601",
        "01_JUNE_2021"
    );
    testify!(drop_day, "20030201", "FEB_2003", "20210601", "JUN_2021");
    testify!(
        full_date_with_holes,
        "$0-$1_01.02.2003",
        "2003-02-01-$0-$1",
        "foo-bar_01.01.1984",
        "1984-01-01-foo-bar"
    );
}
