

use parse_input::{Input, ParseError};

use super::ast::Type;

pub fn parse( input : &mut Input ) -> Result<Type, ParseError> {
    input.choice( &[ parse_void
                   , parse_array
                   , parse_generic
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

/*

    fun(type array) -> type
    sym<type>
    sym::sym<type>
    Simple
    sym::Simple
    { sym : type, ... | rest }
    Dict<type, type>

*/

#[cfg(test)]
mod test {
    use super::*;

    use rand::Rng;
    use rand::distributions::{Distribution, Standard};

    impl Distribution<Type> for Standard {
        fn sample<R : Rng + ?Sized>(&self, rng: &mut R) -> Type {
            Type::Void
        }
    }

    fn display( t : Type ) -> String {
        "string".to_string()
    }

    #[test]
    fn should() {

    }
}
