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

#[derive(Debug)]
pub struct Type {
    type_modifiers: Vec<TypeModifier>,
    rest: Type2,
}

#[derive(Debug)]
pub enum Type2 {
    ParenthesizedType(Box<Type>),
    NullableType(NullableType),
    TypeReference,
    FunctionType,
}

#[derive(Debug)]
pub enum NullableType {
    TypeReference,
    ParenthesizedType(Box<Type>),
}

#[derive(Debug)]
pub enum TypeModifier {
    Annotation(Annotation),
    Suspend,
}

#[derive(Debug)]
pub enum TypeProjectionModifier {
    VarianceModifier(VarianceModifier),
    Annotation(Annotation),
}

#[derive(Debug)]
pub enum VarianceModifier {
    In,
    Out,
}

#[derive(Debug)]
pub enum ReificationModifier {
    Reified,
}

#[derive(Debug)]
pub enum Annotation {
    Todo,
}

#[derive(Debug)]
pub enum AnnotationUseSiteTarget {
    Field,
    Property,
    Get,
    Set,
    Receiver,
    Param,
    Setparam,
    Delegate,
}