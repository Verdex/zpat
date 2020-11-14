
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

