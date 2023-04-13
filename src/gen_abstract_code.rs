use super::abstract_code::{AbstractCode, AbstractExpr};
use super::data::ast::*;
use super::data::tree::*;
use do_notation::m;
use nonempty::{nonempty, NonEmpty};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CodeGenError {
    Other(String),
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodeGenError::Other(msg) => write!(f, "CodeGenError: {}", msg),
        }
    }
}

impl error::Error for CodeGenError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

/*#[derive(Debug, Clone)]
struct DataTree {
    pub parent: Option<Weak<RefCell<DataTree>>>,
    pub children: Vec<Rc<RefCell<DataTree>>>,
    pub size: usize,
    pub level_number: u8,
    pub descriptions: Vec<DataDescriptionClause>,
    pub entry_name: String,
}

impl DataTree {
    pub fn new() -> DataTree {
        DataTree {
            parent: None,
            children: Vec::new(),
            size: 0,
            level_number: 0,
            descriptions: Vec::new(),
            entry_name: "".to_string(),
        }
    }

    pub fn from_data_description(description: &DataDescription) -> DataTree {
        DataTree {
            parent: None,
            children: Vec::new(),
            size: 0,
            level_number: description.level_number,
            descriptions: Vec::new(),
            entry_name: description.entry_name.to_string(),
        }
    }

    pub fn generate_abstract_code(&self) -> Vec<AbstractCode> {
        Vec::new()
    }
}*/

/// convert the list of data_descriptions to DataTree
fn get_data_tree<'a>(
    root_value: &'a DataDescription<'a>,
    descriptions: &'a VecDeque<DataDescription<'a>>,
) -> Result<Tree<&'a DataDescription<'a>>, CodeGenError> {
    if descriptions.len() == 0 {
        return Ok(Tree::new());
    }

    let insert_error = CodeGenError::Other("Failed to insert a item into tree".to_string());

    let mut tree = Tree::new();
    let mut description = &descriptions[0];
    let mut node_id = tree.add(0, root_value).ok_or(insert_error.clone())?;

    for new_description in descriptions.iter() {
        match description.level_number.cmp(&new_description.level_number) {
            Ordering::Equal => {
                let parent_id = tree.parent_id(node_id).unwrap_or(node_id);
                node_id = tree
                    .add(parent_id, new_description)
                    .ok_or(insert_error.clone())?;
            }
            Ordering::Less => {
                node_id = tree
                    .add(node_id, new_description)
                    .ok_or(insert_error.clone())?;
            }
            Ordering::Greater => {
                while let Some((parent_id, parent_description)) = tree.parent(node_id) {
                    if parent_description.level_number < new_description.level_number {
                        break;
                    }
                    node_id = parent_id;
                }
                node_id = tree
                    .add(node_id, new_description)
                    .ok_or(insert_error.clone())?;
            }
        }
        description = new_description;
    }
    Ok(tree)
}

#[derive(Clone, Debug)]
pub enum ByteArrayData {
    Bytes(Vec<u8>),
    String(String),
}

impl<'a> DataDescription<'a> {
    pub fn get_data_size(&self) -> u32 {
        if let Some(pic) = self.get_picture() {
            match pic {
                Picture::Numeric {
                    digits,
                    pic,
                    signed,
                    scale,
                } => digits,
                Picture::Alphanumeric { len, pic } => len,
            }
        } else {
            0
        }
    }

    fn bytes_to_str_if_printable(bytes: &Vec<u8>) -> ByteArrayData {
        let mut printable = true;
        for b in bytes {
            if !b.is_ascii_graphic() {
                printable = false;
                break;
            }
        }
        if printable {
            ByteArrayData::String(String::from_utf8(bytes.to_vec()).unwrap())
        } else {
            ByteArrayData::Bytes(bytes.clone())
        }
    }

    pub fn get_initial_bytes(&self) -> ByteArrayData {
        if let (Some(value), Some(pic)) = (self.get_value_clause(), self.get_picture()) {
            let bytes = value.as_bytes();
            match pic {
                Picture::Numeric {
                    digits,
                    pic,
                    signed,
                    scale,
                } => {
                    if bytes.len() > digits as usize {
                        Self::bytes_to_str_if_printable(&bytes[0..digits as usize].to_vec())
                    } else {
                        let mut ret = vec!['0' as u8; digits as usize - bytes.len()];
                        ret.extend(bytes.to_vec());
                        Self::bytes_to_str_if_printable(&ret)
                    }
                }
                Picture::Alphanumeric { len, pic } => {
                    if bytes.len() > len as usize {
                        Self::bytes_to_str_if_printable(&bytes[0..len as usize].to_vec())
                    } else {
                        let mut ret = vec!['0' as u8; len as usize - bytes.len()];
                        ret.extend(bytes.to_vec());
                        Self::bytes_to_str_if_printable(&ret.to_vec())
                    }
                }
            }
        } else {
            ByteArrayData::String("".to_string())
        }
    }

    pub fn get_type(&self) -> &'a str {
        if let Some(pic) = self.get_picture() {
            match pic {
                Picture::Numeric {
                    digits,
                    pic,
                    signed,
                    scale,
                } => "TYPE_NUMERIC_DISPLAY",
                Picture::Alphanumeric { pic, len } => "TYPE_ALPHANUMERIC",
            }
        } else {
            "TYPE_ALPHANUMERIC"
        }
    }

    pub fn get_digits(&self) -> u32 {
        if let Some(pic) = self.get_picture() {
            match pic {
                Picture::Numeric {
                    digits,
                    pic,
                    signed,
                    scale,
                } => digits,
                _ => 0,
            }
        } else {
            0
        }
    }

    pub fn get_scale(&self) -> i32 {
        if let Some(pic) = self.get_picture() {
            match pic {
                Picture::Numeric {
                    digits,
                    pic,
                    signed,
                    scale,
                } => scale,
                _ => 0,
            }
        } else {
            0
        }
    }

    pub fn get_flags_string(&self) -> &'a str {
        "FLAG_NONE"
    }

    pub fn get_pic(&self) -> &'a str {
        if let Some(pic) = self.get_picture() {
            match pic {
                Picture::Numeric {
                    digits,
                    pic,
                    signed,
                    scale,
                } => pic,
                Picture::Alphanumeric { pic, len } => pic,
            }
        } else {
            ""
        }
    }
}

fn abstract_code_of_data_description_tree_and_size<'a>(
    tree: &Tree<&DataDescription<'a>>,
) -> (Vec<AbstractCode<'a>>, u32) {
    match tree.root() {
        None => (Vec::new(), 0),
        Some(root_id) => {
            let mut total_data_size = 0;
            let mut code = Vec::new();
            for child in tree.children(root_id).iter() {
                let data_size = child.get_data_size();
                code.push(AbstractCode::LetField(
                    child.entry_name,
                    AbstractExpr::LibCoreFunc(
                        "register_field",
                        vec![
                            AbstractExpr::UInt(total_data_size),
                            AbstractExpr::UInt(data_size),
                            AbstractExpr::LibIdentifier(child.get_type()),
                            AbstractExpr::UInt(child.get_digits()),
                            AbstractExpr::Int(child.get_scale()),
                            AbstractExpr::LibIdentifier(child.get_flags_string()),
                            AbstractExpr::Str(child.get_pic()),
                        ],
                    ),
                ));
                total_data_size += data_size
            }
            (code, total_data_size)
        }
    }
}

pub fn generate_abstract_code<'a>(
    program: &'a CobolProgram,
    data_description_root_node: &'a DataDescription,
) -> Result<Vec<AbstractCode<'a>>, CodeGenError> {
    let mut code = Vec::new();

    let data_tree = match program.data_division {
        Some(ref data_division) => match data_division.working_storage_section {
            Some(ref working_storage_section) => Some(get_data_tree(
                &data_description_root_node,
                &working_storage_section.data_descriptions,
            )),
            None => None,
        },
        None => None,
    };

    let (construct_data_code, total_data_size) = match data_tree {
        None => (Vec::new(), 0),
        Some(Err(e)) => return Err(e),
        Some(Ok(ref tree)) => abstract_code_of_data_description_tree_and_size(&tree),
    };

    let initialize_core_code = AbstractCode::GetNewCore(total_data_size);

    let mut initialize_data_code = Vec::new();
    match data_tree {
        None => (),
        Some(Err(e)) => return Err(e),
        Some(Ok(tree)) => {
            for child in tree.children(tree.root().unwrap()).iter() {
                let initial_bytes = child.get_initial_bytes();
                initialize_data_code.push(AbstractCode::Expr(AbstractExpr::LibCoreFunc(
                    match initial_bytes {
                        ByteArrayData::String(_) => "set_string",
                        ByteArrayData::Bytes(_) => "set_bytes",
                    },
                    vec![
                        AbstractExpr::FieldIdentifier(child.entry_name),
                        match initial_bytes {
                            ByteArrayData::String(s) => AbstractExpr::String(s),
                            ByteArrayData::Bytes(b) => AbstractExpr::Bytes(b),
                        },
                    ],
                )));
            }
        }
    };

    let procedure_division_code = match &program.procedure_division {
        Some(procedure_division) => procedure_division
            .labels_statements
            .iter()
            .map(|x| match x {
                LabelStatement::Statement(Statement::Move(st)) => convert_move_statement(st),
                LabelStatement::Statement(Statement::Display(st)) => convert_display_statement(st),
                LabelStatement::Statement(Statement::Accept(st)) => convert_accept_statement(st),
                LabelStatement::Label(_) => Vec::new(),
                LabelStatement::Section(_) => Vec::new(),
            })
            .into_iter()
            .flatten()
            .collect(),
        None => Vec::new(),
    };

    code.push(initialize_core_code);
    code.extend(construct_data_code);
    code.extend(initialize_data_code);
    code.extend(procedure_division_code);

    Ok(code.clone())
}

fn convert_accept_statement<'a>(st: &AcceptStatement<'a>) -> Vec<AbstractCode<'a>> {
    vec![AbstractCode::Expr(AbstractExpr::LibCoreFunc(
        "accept",
        vec![AbstractExpr::FieldIdentifier(st.arg)],
    ))]
}

fn convert_move_statement<'a>(st: &MoveStatement<'a>) -> Vec<AbstractCode<'a>> {
    if st.srcs.len() == 1 {
        st.dsts
            .iter()
            .map(|dst| {
                AbstractCode::Expr(AbstractExpr::LibCoreFunc(
                    "move_field",
                    vec![
                        //TODO avoid invoking same procedure many times
                        AbstractExpr::FieldIdentifier(st.srcs[0]),
                        AbstractExpr::FieldIdentifier(dst),
                    ],
                ))
            })
            .collect()
    } else {
        st.srcs
            .iter()
            .zip(st.dsts.iter())
            .map(|(src, dst)| {
                AbstractCode::Expr(AbstractExpr::Func(
                    "move_field",
                    vec![
                        AbstractExpr::FieldIdentifier(src),
                        AbstractExpr::FieldIdentifier(dst),
                    ],
                ))
            })
            .collect()
    }
}

/*fn field_identifier<'a, 'b>(var: &'a str) -> AbstractExpr<'b> {
    //AbstractExpr::Identifier(&field_identifier_str(var).clone()).clone()
    AbstractExpr::Identifier(format!("{}_field", var).clone().as_str())
}

fn field_identifier_str(var: &str) -> String {
    format!("{}_field", var)
}*/

fn convert_display_statement<'a>(st: &DisplayStatement<'a>) -> Vec<AbstractCode<'a>> {
    st.args
        .iter()
        .map(|arg| {
            AbstractCode::Expr(AbstractExpr::LibFunc(
                "display",
                vec![AbstractExpr::LibCoreFunc(
                    "field_as_string",
                    vec![AbstractExpr::FieldIdentifier(arg)],
                )],
            ))
        })
        .collect()
}
