

use parse_input::{PSym, Input, ParseError};

use super::ast::{Type};

pub fn parse( input : Input ) -> Result<Type, ParseError> {
    Ok(Type::Void)
}
