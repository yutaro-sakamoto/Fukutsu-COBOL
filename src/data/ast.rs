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
    pub data_descriptions: Vec<DataDescription<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct DataDescription<'a> {
    pub level_number: u8,
    pub entry_name: &'a str,
    pub description_clauses: Vec<DataDescriptionClause<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum DataDescriptionClause<'a> {
    Picture(&'a str),
    Value(&'a str),
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
