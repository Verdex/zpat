

use parse_input::{PSym, Input, ParseError};

use super::ast::Type;

pub fn parse( input : Input ) -> Result<Type, ParseError> {
    Ok(Type::Void)
}

/*

    void
    fun(type array) -> type
    [type]
    'name
    sym<type>
    sym::sym<type>
    Simple
    sym::Simple
    { sym : type, ... | rest }
    Dict<type, type>

*/
