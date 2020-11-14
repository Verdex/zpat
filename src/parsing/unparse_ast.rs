
use super::ast::*;

/*

    Row { params : Vec<(PSym, Type)>, rest_name : Option<PSym> },

*/

fn display_namespace_symbol( ns : NamespaceSymbol ) -> String {

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
        Type::Array(types) => format!( "[{}]"
                                     , types.into_iter()
                                            .map(display_type)
                                            .collect::<Vec<String>>()
                                            .join(", ")
                                     ),
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

    }
}
