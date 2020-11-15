
use parse_input::PSym;

#[derive(Debug)]
pub struct NamespaceSymbol {
    pub namespace : Vec<PSym>,
    pub name : PSym,
}

#[derive(Debug)]
pub enum Type {
    Void,
    Fun { params : Vec<Type>, ret : Box<Type> },
    Array(Box<Type>),
    Generic(PSym),
    Index { name : NamespaceSymbol, params : Vec<Type> },
    Simple(NamespaceSymbol),
    Dict { key : Box<Type>, value : Box<Type> },
    Row { params : Vec<(PSym, Type)>, rest_name : Option<PSym> },
}

#[derive(Debug)]
pub enum SliceOption {
    Blank,
    Value(Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Number(PSym),
    ZString(PSym),
    Bool(bool),
    Binding(NamespaceSymbol),
    Lambda { params: Vec<(PSym, Option<Type>)>, ret_type: Option<Type>, body: Box<Expr> },
    Index { expr: Box<Expr>, index: Box<Expr> },
    Slice { start: SliceOption, end: SliceOption }, 
    SlotAccess { expr: Box<Expr>, slot: PSym },
    FunCall { expr: Box<Expr>, params: Vec<Expr> },
    ExtensionFunCall { left: Box<Expr>, right: Box<Expr> },
    ArrayCons(Vec<Expr>),
    DictCons(Vec<(Expr, Expr)>), 
    ObjCons(Vec<(PSym, Expr)>),
    Let { name: (PSym, Option<Type>), params: Vec<(PSym, Option<Type>)>, body: Box<Expr> }, 
    Block { exprs: Vec<Expr> },
    // match
    // match all
}

