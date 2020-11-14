
use super::ast::*;


fn display_namespace_symbol( ns : NamespaceSymbol ) -> String {
    if ns.namespace.len() != 0 {
        format!( "{}::{}"
               , ns.namespace.into_iter()
                             .map(|v| v.value)
                             .collect::<Vec<String>>()
                             .join("::")
               , ns.name.value
               )
    }
    else {
        ns.name.value
    }
}

pub fn display_type( t : Type ) -> String {
    match t {
        Type::Void => "void".to_string(),
        Type::Fun { params, ret } => format!( "Fun({}) -> {}"
                                            , params.into_iter()
                                                    .map(display_type)
                                                    .collect::<Vec<String>>()
                                                    .join(", ")
                                            , display_type(*ret)
                                            ),
        Type::Array(t) => format!( "[{}]", display_type(*t) ),
        Type::Generic(sym) => format!( "'{}", sym.value ),
        Type::Index { name, params } => format!( "{}<{}>"
                                               , display_namespace_symbol(name)
                                               , params.into_iter()
                                                       .map(display_type)
                                                       .collect::<Vec<String>>()
                                                       .join(", ")
                                               ),
        Type::Simple(name) => display_namespace_symbol(name),
        Type::Dict { key, value } => format!( "Dict<{}, {}>"
                                            , display_type(*key)
                                            , display_type(*value) 
                                            ),
        Type::Row { params, rest_name: Some(rest_name) } => 
            format!( "{{ {} | {} }}"
                   , params.into_iter()
                           .map(|(n,t)| format!( "{} : {}", n.value, display_type(t) ))
                           .collect::<Vec<String>>()
                           .join(", ")
                   , rest_name.value
                   ),
        Type::Row { params, rest_name: None } => format!( "{{ {} }}"
                                                        , params.into_iter()
                                                                .map(|(n,t)| format!( "{} : {}", n.value, display_type(t) ))
                                                                .collect::<Vec<String>>()
                                                                .join(", ")
                                                        ),
    }
}
