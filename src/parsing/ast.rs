
use parse_input::PSym;

#[derive(Debug)]
pub struct NamespaceSymbol {
    namespace : Vec<PSym>,
    name : PSym,
}

#[derive(Debug)]
pub enum Type {
    Void,
    Fun { params : Vec<Type>, ret : Box<Type> },
    Array(Box<Type>),
    Generic(String),
    Index { name : NamespaceSymbol, params : Vec<Type> },
    Simple(NamespaceSymbol),
    Dict { key : Box<Type>, value : Box<Type> },
    Row { params : Vec<(String, Type)>, rest_name : Option<String> },
}
