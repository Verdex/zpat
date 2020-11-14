

use parse_input::{Input, ParseError};

use super::ast::{Type, NamespaceSymbol};

pub fn parse( input : &mut Input ) -> Result<Type, ParseError> {
    input.choice( &[ parse_void
                   , parse_array
                   , parse_generic
                   , parse_dict
                   , parse_fun
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

/*

    sym<type>
    sym::sym<type>
    Simple
    sym::Simple
    { sym : type, ... | rest }

*/

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
