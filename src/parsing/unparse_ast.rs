
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

fn display_slice_option( so : SliceOption ) -> String {
    match so {
        SliceOption::Blank => "".to_string(),
        SliceOption::Value(e) => display_expr(*e),
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


/*

    Lambda { params: Vec<(PSym, Option<Type>)>, ret_type: Option<Type>, body: Box<Expr> },
    ArrayCons { params: Vec<Expr> },
    DictCons { params: Vec<(Expr, Expr)> }, 
    ObjCons { params: Vec<(PSym, Expr)> },
    Let { name: (PSym, Option<Type>), params: Vec<(PSym, Option<Type>)>, body: Box<Expr> }, 
    Block { exprs: Vec<Expr> },

*/

pub fn display_expr( e : Expr ) -> String {
    match e {
        Expr::Number(sym) => sym.value,
        Expr::ZString(sym) => format!( "\"{}\"", sym.value ),
        Expr::Bool(true) => "true".to_string(),
        Expr::Bool(false) => "false".to_string(),
        Expr::Binding(ns) => display_namespace_symbol(ns),

        Expr::Index { expr, index } => format!( "{}[{}]"
                                              , display_expr(*expr)
                                              , display_expr(*index)
                                              ),
        Expr::Slice { start, end } => format!( "{}..{}"
                                             , display_slice_option(start)
                                             , display_slice_option(end)
                                             ),
        Expr::SlotAccess { expr, slot } => format!( "{}.{}"
                                                  , display_expr(*expr)
                                                  , slot.value
                                                  ),
        Expr::FunCall { expr, params } => format!( "{}({})"
                                                 , display_expr(*expr)
                                                 , params.into_iter()
                                                         .map(display_expr)
                                                         .collect::<Vec<String>>()
                                                         .join(", ")
                                                 ),
        Expr::ExtensionFunCall { left, right } => format!( "{}-{}"
                                                         , display_expr(*left)
                                                         , display_expr(*right)
                                                         ),
        Expr::ArrayCons(items) => format!( "[{}]"
                                         , items.into_iter()
                                                .map(display_expr)
                                                .collect::<Vec<String>>()
                                                .join(", ")
                                         ),
        Expr::DictCons(mappings) => format!( "{{ {} }}"
                                         , mappings.into_iter()
                                                   .map(|(key, value)| format!( "{} => {}"
                                                                              , display_expr(key)
                                                                              , display_expr(value) 
                                                                              ))
                                                   .collect::<Vec<String>>()
                                                   .join(", ")
                                         ),

        Expr::ObjCons(slots) => format!( "{{ {} }}"
                                       , slots.into_iter()
                                              .map(|(slot, value)| format!( "{}: {}"
                                                                          , slot.value
                                                                          , display_expr(value) 
                                                                          ))
                                              .collect::<Vec<String>>()
                                              .join(", ")
                                         ),
        _ => panic!("blarg"),
    }
}
