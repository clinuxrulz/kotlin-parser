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
pub struct Parameter {
    pub name: String,
    pub type_: Box<Type>,
}

#[derive(Debug)]
pub struct Type {
    pub type_modifiers: Vec<TypeModifier>,
    pub rest: Type2,
}

#[derive(Debug)]
pub enum Type2 {
    ParenthesizedType(Box<Type>),
    NullableType(NullableType),
    TypeReference(TypeReference),
    FunctionType,
    Todo,
}

#[derive(Debug)]
pub enum NullableType {
    TypeReference(TypeReference),
    ParenthesizedType(Box<Type>),
}

#[derive(Debug)]
pub enum TypeReference {
    UserType(UserType),
    Dynamic,
}

#[derive(Debug)]
pub struct UserType {
    pub parts: Vec<SimpleUserType>,
}

#[derive(Debug)]
pub struct SimpleUserType {
    pub name: String,
    pub type_arguments: Vec<TypeProjection>,
}

#[derive(Debug)]
pub enum TypeProjection {
    TypeProjection(TypeProjection2),
    Star,
}

#[derive(Debug)]
pub struct TypeProjection2 {
    pub type_projection_modifiers: Vec<TypeProjectionModifier>,
    pub type_: Box<Type>,
}

#[derive(Debug)]
pub enum Expression {
    PrimaryExpression(PrimaryExpression),
    JumpExpression(JumpExpression),
    CallableReference(Option<ReceiverType>,SimpleIdentifierOrClass),
}

#[derive(Debug)]
pub enum PrimaryExpression {
    ParenthesizedExpression(Box<Expression>),
    SimpleIdentifier(String),
    LiteralConstant(),
    StringLiteral(),
    CallableRederence(),
    FunctionLiteral(),
    ObjectLiteral(),
    CollectionLiteral(),
    ThisExpression(),
    SuperExpression(),
    IfExpression(),
    WhenExpression(),
    TryExpression(),
    JumpExpression(JumpExpression),
}

#[derive(Debug)]
pub enum JumpExpression {
    Throw(Box<Expression>),
    Return(Box<Expression>),
    ReturnAt(Box<Expression>),
    Continue,
    ContinueAt,
    Break,
    BreatAt,
}

#[derive(Debug)]
pub enum SimpleIdentifierOrClass {
    SimpleIdentifier(String),
    Class,
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
pub struct FunctionType {
    pub receiver_type_op: Option<ReceiverType>,
    pub type_parameters: Vec<FunctionTypeParameter>,
}

#[derive(Debug)]
pub enum FunctionTypeParameter {
    Parameter(Parameter),
    Type(Box<Type>),
}

#[derive(Debug)]
pub struct ReceiverType {
    pub type_modifiers: Vec<TypeModifier>,
    pub rest: ReceiverType2,
}

#[derive(Debug)]
pub enum ReceiverType2 {
    ParenthesizedType(Box<Type>),
    NullableType(NullableType),
    TypeReference(TypeReference),
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
