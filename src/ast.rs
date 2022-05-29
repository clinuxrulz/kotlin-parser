#[derive(Debug)]
pub struct Identifier {
    pub parts: Vec<String>,
}

#[derive(Debug)]
pub struct ImportHeader {
    pub identifier: Identifier,
    pub rest_op: Option<ImportHeader2>,
}

#[derive(Debug)]
pub enum ImportHeader2 {
    ImportAll,
    ImportAlias(String),
}
