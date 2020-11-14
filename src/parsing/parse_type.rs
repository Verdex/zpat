

use parse_input::{Input, ParseError};

use super::ast::{Type, NamespaceSymbol};
use super::parse_misc;

pub fn parse( input : &mut Input ) -> Result<Type, ParseError> {
    input.choice( &[ parse_void
                   , parse_array
                   , parse_generic
                   , parse_dict
                   , parse_fun
                   , parse_row
                   , parse_index_or_simple
                   ] )
}

fn parse_void( input : &mut Input ) -> Result<Type, ParseError> {
    input.expect( "void" )?;
    Ok(Type::Void)
}

fn parse_array( input : &mut Input ) -> Result<Type, ParseError> {
    input.expect("[")?;

    let t = parse( input )?;

    input.expect("]")?;

    Ok(Type::Array(Box::new(t)))
}

fn parse_generic( input : &mut Input ) -> Result<Type, ParseError> {
    input.expect("'")?;

    let name = input.parse_symbol()?;

    Ok(Type::Generic(name))
}

fn parse_dict( input : &mut Input ) -> Result<Type, ParseError> {
    input.expect("Dict")?;
    input.expect("<")?;

    let key = Box::new(parse(input)?);

    input.expect(",")?;

    let value = Box::new(parse(input)?);

    input.expect(">")?;

    Ok(Type::Dict { key, value })
}

fn parse_fun( input : &mut Input ) -> Result<Type, ParseError> {
    input.expect("fun")?;
    input.expect("(")?;

    let params = input.list(parse)?;

    input.expect(")")?;
    input.expect("->")?;

    let ret = Box::new(parse(input)?);

    Ok(Type::Fun { params, ret })
}

fn parse_row( input : &mut Input ) -> Result<Type, ParseError> {
    input.expect("{")?;
    let params = input.list(|p| {
        let name = p.parse_symbol()?;
        p.expect(":")?;
        let t = parse(p)?;
        Ok((name, t))
    })?;

    let rest_name = input.maybe( |p| {
        p.expect("|")?;
        p.parse_symbol()
    });

    Ok(Type::Row { params, rest_name })
}

fn parse_index_or_simple( input : &mut Input ) -> Result<Type, ParseError> {
    let name = parse_misc::parse_namespace_symbol(input)?;

    let maybe_index = input.maybe( |p| {
        p.expect("<")?;
        let params = p.list( parse )?;
        p.expect(">")?;
        Ok(params)
    });

    match maybe_index {
        None => Ok(Type::Simple(name)),
        Some(params) => Ok(Type::Index { params, name }),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    use rand::Rng;
    use super::super::random_ast;
    use super::super::unparse_ast;


    #[test]
    fn should() {
        let mut rng = rand::thread_rng();
        let t = rng.gen::<Type>();
        panic!( "{}", unparse_ast::display_type(t) );
    }
}
