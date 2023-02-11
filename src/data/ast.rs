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
    pub working_storage_section: WorkingStorageSection<'a>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct WorkingStorageSection<'a> {
    pub data_descriptions: Vec<DataDescription<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct DataDescription<'a> {
    pub level_number: u8,
    pub entry_name: &'a str,
    pub descriptions: Vec<DataDescriptionClause<'a>>,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum DataDescriptionClause<'a> {
    Picture(&'a str),
    Value(&'a str),
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct ProcedureDivision<'a> {
    pub labelsStatements: Vec<LabelStatement<'a>>,
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
    pub src: &'a str,
    pub dsts: Vec<&'a str>,
    pub corresponding: bool,
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct DisplayStatement<'a> {
    pub arg: Vec<&'a str>,
}
