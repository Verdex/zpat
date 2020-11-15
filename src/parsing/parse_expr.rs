
use parse_input::{Input, ParseError};

use super::ast::Expr;
use super::parse_misc;
use super::parse_type;

pub fn parse( input : &mut Input ) -> Result<Expr, ParseError> {
    input.choice( &[ parse_number
                   , parse_string
                   , parse_bool
                   , parse_index
                   , parse_namespace_symbol
                   ] )
}

fn parse_number( input : &mut Input ) -> Result<Expr, ParseError> {
    Ok(Expr::Number(input.parse_number()?))
}

fn parse_string( input : &mut Input ) -> Result<Expr, ParseError> {
    Ok(Expr::ZString(input.parse_string()?))
}

fn parse_bool( input : &mut Input ) -> Result<Expr, ParseError> {
    let rp = input.create_restore();
    let v = input.parse_symbol()?;

    if v.value == "true" {
        Ok(Expr::Bool(true))
    }
    else if v.value == "false" {
        Ok(Expr::Bool(false))
    }
    else {
        input.restore(rp);
        Err(ParseError::ErrorAt(v.start, "Expected boolean".to_string()))
    }
}

fn parse_namespace_symbol( input : &mut Input ) -> Result<Expr, ParseError> {
    Ok(Expr::Binding(parse_misc::parse_namespace_symbol(input)?))
}

fn parse_index( input : &mut Input ) -> Result<Expr, ParseError> {
    let rp = input.create_restore();

    let expr = Box::new(parse(input)?);

    match input.expect("[") {
        Ok(_) => (), 
        Err(e) => {
            input.restore(rp);
            return Err(e)
        },
    }
    
    let index = Box::new(parse(input)?);

    input.expect("]")?;

    Ok(Expr::Index { expr, index })
}

#[cfg(test)]
mod test {
    use super::*;

    use rand::Rng;
    use super::super::random_ast;
    use super::super::unparse_ast;

    // TODO false_blarg symbol parses

    #[test]
    fn should_parse_random_exprs() -> Result<(), ParseError> {
        for _ in 0..50 {
            let mut rng = rand::thread_rng();
            let e_input = rng.gen::<Expr>();
            let string_value = unparse_ast::display_expr(e_input);

            let x = string_value.char_indices().collect::<Vec<(usize, char)>>();
            let mut input = Input::new(&x);

            let e_output = parse(&mut input)?;
            let output_string_value = unparse_ast::display_expr(e_output);

            assert_eq!( output_string_value, string_value );

            input.expect_end()?;
        }
        Ok(())
    }
}

