use super::abstract_code::AbstractCode;
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
    descriptions: &'a VecDeque<DataDescription<'a>>,
) -> Result<Tree<&'a DataDescription<'a>>, CodeGenError> {
    if descriptions.len() == 0 {
        return Ok(Tree::new());
    }

    let insert_error = CodeGenError::Other("Failed to insert a item into tree".to_string());

    let mut tree = Tree::new();
    let mut description = &descriptions[0];
    let mut node_id = tree.add(0, &descriptions[0]).ok_or(insert_error.clone())?;

    for new_description in descriptions.range(1..) {
        match description.level_number.cmp(&new_description.level_number) {
            Ordering::Equal => {
                let parent_id = tree.parent_id(node_id).ok_or(insert_error.clone())?;
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

pub fn generate_abstract_code<'a>(
    program: &'a CobolProgram,
) -> Result<Vec<AbstractCode>, CodeGenError> {
    let mut code = Vec::new();

    let data_tree = program
        .data_division
        .as_ref()
        .and_then(|d| d.working_storage_section.as_ref())
        .map(|w| get_data_tree(&w.data_descriptions));

    /*let data_initialization_code: Vec<AbstractCode> = match data_tree {
        None => Vec::new(),
        Some(Err(e)) => return Err(e),
        Some(Ok(code_list)) => code_list,
        Some(k) => (*(k.unwrap())).borrow().generate_abstract_code(),
    };*/

    let procedure_division_code = match &program.procedure_division {
        Some(procedure_division) => procedure_division
            .labels_statements
            .iter()
            .map(|x| match x {
                LabelStatement::Section(name) => {
                    vec![AbstractCode::Func(
                        "console.log".to_string(),
                        vec![format!("'Section: {}'", name)],
                    )]
                }
                LabelStatement::Label(name) => {
                    vec![AbstractCode::Func(
                        "console.log".to_string(),
                        vec![format!("'Label: {}'", name)],
                    )]
                }
                LabelStatement::Statement(Statement::Move(st)) => convert_move_statement(st),
                LabelStatement::Statement(Statement::Display(st)) => convert_display_statement(st),
            })
            .into_iter()
            .flatten()
            .collect(),
        None => Vec::new(),
    };

    //code.extend(data_initialization_code);
    code.extend(procedure_division_code);

    Ok(code)
}

fn convert_move_statement<'a>(st: &MoveStatement<'a>) -> Vec<AbstractCode> {
    if st.srcs.len() == 1 {
        let src_name = format!("{}_field", st.srcs[0]);
        st.dsts
            .iter()
            .map(|dst| {
                AbstractCode::Func(
                    "core.move_field".to_string(),
                    vec![src_name.clone(), format!("{}_field", dst)],
                )
            })
            .collect()
    } else {
        st.srcs
            .iter()
            .zip(st.dsts.iter())
            .map(|(src, dst)| {
                AbstractCode::Func(
                    "core.move_field".to_string(),
                    vec![format!("{}_field", src), format!("{}_field", dst)],
                )
            })
            .collect()
    }
}

fn convert_display_statement<'a>(st: &DisplayStatement<'a>) -> Vec<AbstractCode> {
    let args: Vec<String> = st.args.iter().map(|arg| format!("'{}'", arg)).collect();
    vec![AbstractCode::Func(
        "console.log".to_string(),
        vec![args.join(" + ")],
    )]
}
