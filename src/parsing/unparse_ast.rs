
use parse_input::PSym;

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

fn display_fun_param( param : (PSym, Option<Type>) ) -> String {
    match param {
        (name, None) => name.value,
        (name, Some(t)) => format!( "{} : {}", name.value, display_type(t) ),
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
        Expr::Slice { start: Some(start), end: Some(end) } => format!( "{}..{}"
                                                                     , display_expr(*start)
                                                                     , display_expr(*end)
                                                                     ),
        Expr::Slice { start: Some(start), end: None } => format!( "{}..{}"
                                                                , display_expr(*start)
                                                                , "".to_string() 
                                                                ),
        Expr::Slice { start: None, end: Some(end) } => format!( "{}..{}"
                                                              , "".to_string() 
                                                              , display_expr(*end) 
                                                              ),
        Expr::Slice { start: None, end: None } => format!( "{}..{}"
                                                         , "".to_string() 
                                                         , "".to_string() 
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
        Expr::Block(es) => format!( "{{ {} }}"
                                  , es.into_iter()
                                      .map(display_expr)
                                      .collect::<Vec<String>>()
                                      .join(";\n")
                                  ),
        Expr::Lambda { params, ret_type, body } => 
            match ret_type {
                None => format!( "|{}| {}"
                               , params.into_iter()
                                       .map(display_fun_param)
                                       .collect::<Vec<String>>()
                                       .join(" ")
                               , display_expr(*body)
                               ),
                Some(ret_type) => format!( "|{}| -> {} {}"
                                         , params.into_iter()
                                                 .map(display_fun_param)
                                                 .collect::<Vec<String>>()
                                                 .join(" ")
                                         , display_type(ret_type)
                                         , display_expr(*body)
                                         )
            },
        Expr::Let { name, mut params, value, body } => {
            let mut one = vec![name];
            one.append(&mut params);
            format!( "let {} = {} in {};"
                   , one.into_iter()
                        .map(display_fun_param)
                        .collect::<Vec<String>>()
                        .join(" ")
                   , display_expr(*value)
                   , display_expr(*body)
                   )
        },
    }
}

