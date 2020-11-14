
use parse_input::{Input, ParseError};

use super::ast::NamespaceSymbol;


pub fn parse_namespace_symbol( input : &mut Input ) -> Result<NamespaceSymbol, ParseError> {
    let namespace = input.zero_or_more(|p| {
        let sym = p.parse_symbol()?;  
        p.expect("::")?;
        Ok(sym)
    })?;

    let name = input.parse_symbol()?;

    Ok(NamespaceSymbol { name, namespace })
}


