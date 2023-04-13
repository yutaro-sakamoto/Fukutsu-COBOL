use regex::Regex;
use std::collections::VecDeque;

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct CobolProgram<'a> {
    pub identification_division: IdentificationDivision<'a>,
    pub environment_division: Option<EnvironmentDivision<'a>>,
    pub data_division: Option<DataDivision<'a>>,
    pub procedure_division: Option<ProcedureDivision<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct IdentificationDivision<'a> {
    pub program_id: &'a str,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct EnvironmentDivision<'a> {
    pub dummy: &'a str,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct DataDivision<'a> {
    pub working_storage_section: Option<WorkingStorageSection<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct WorkingStorageSection<'a> {
    pub data_descriptions: VecDeque<DataDescription<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct DataDescription<'a> {
    pub level_number: u8,
    pub entry_name: &'a str,
    pub description_clauses: Vec<DataDescriptionClause<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum DataDescriptionClause<'a> {
    Picture(Picture<'a>),
    Value(String),
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct ProcedureDivision<'a> {
    pub labels_statements: Vec<LabelStatement<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum LabelStatement<'a> {
    Section(&'a str),
    Label(&'a str),
    Statement(Statement<'a>),
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Statement<'a> {
    Move(MoveStatement<'a>),
    Display(DisplayStatement<'a>),
    Accept(AcceptStatement<'a>),
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct MoveStatement<'a> {
    pub srcs: Vec<&'a str>,
    pub dsts: Vec<&'a str>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct DisplayStatement<'a> {
    pub args: Vec<&'a str>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct AcceptStatement<'a> {
    pub arg: &'a str,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Picture<'a> {
    Numeric {
        pic: &'a str,
        signed: bool,
        digits: u32,
        scale: i32,
    },
    Alphanumeric {
        pic: &'a str,
        len: u32,
    },
}

pub fn size_pic_with_brackets(pic: &str) -> u32 {
    let mut i = 0;
    let bytes = pic.as_bytes();
    let mut size = 0;
    while i < bytes.len() {
        if bytes[i] == b'(' {
            let mut j = i + 1;
            let mut local_size = 0;
            while bytes[j] != b')' {
                local_size += 10 * local_size + (bytes[j] - b'0') as u32;
                j += 1;
            }
            i = j + 1;
            size += local_size;
        } else {
            size += 1;
            i += 1;
        }
    }
    size
}

pub fn parse_pic_9_leading_p<'a>(pic: &'a str) -> Option<Picture<'a>> {
    let re = Regex::new(r"([sS]?)([pP]*)((9(\([0-9]+\)))+)").unwrap();

    match re.captures(pic) {
        Some(caps) => {
            let signed = caps[1].len() > 0;
            let ps: u32 = caps[2].len() as u32;
            let digits = size_pic_with_brackets(&caps[3]);
            let scale = (if ps > 0 { digits + ps } else { 0 }) as i32;
            Some(Picture::Numeric {
                pic,
                signed,
                digits,
                scale,
            })
        }
        None => None,
    }
}

pub fn parse_pic_9_trailing_p<'a>(pic: &'a str) -> Option<Picture<'a>> {
    let re = Regex::new(r"([sS]?)((9(\([0-9]+\)))+)([pP]*)").unwrap();

    match re.captures(pic) {
        Some(caps) => {
            let signed = caps[1].len() > 0;
            let digits = size_pic_with_brackets(&caps[2]);
            let scale: i32 = -1 * caps[3].len() as i32;
            Some(Picture::Numeric {
                pic,
                signed,
                digits,
                scale,
            })
        }
        None => None,
    }
}

pub fn parse_pic_9_v<'a>(pic: &'a str) -> Option<Picture<'a>> {
    let re = Regex::new(r"[sS]?(9(\([0-9]+\)))+V(9(\([0-9]+\)))*").unwrap();

    match re.captures(pic) {
        Some(caps) => {
            let signed = caps[1].len() > 0;
            let digits1 = size_pic_with_brackets(&caps[2]);
            let digits2 = size_pic_with_brackets(&caps[3]);
            let digits = digits1 + digits2;
            let scale: i32 = digits2 as i32;
            Some(Picture::Numeric {
                pic,
                signed,
                digits,
                scale,
            })
        }
        None => None,
    }
}

impl<'a> DataDescription<'a> {
    pub fn get_picture(&self) -> Option<Picture<'a>> {
        for clause in &self.description_clauses {
            match clause {
                DataDescriptionClause::Picture(pic) => return Some(pic.clone()),
                _ => (),
            }
        }
        None
    }
    pub fn get_value_clause(&self) -> Option<String> {
        for clause in &self.description_clauses {
            match clause {
                DataDescriptionClause::Value(s) => return Some(s.clone()),
                _ => (),
            }
        }
        None
    }
}
