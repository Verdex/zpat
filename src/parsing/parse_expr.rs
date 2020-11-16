
use parse_input::{Input, ParseError};

use super::ast::Expr;
use super::parse_misc;
use super::parse_type;

pub fn parse( input : &mut Input ) -> Result<Expr, ParseError> {
    let expr = input.choice( &[ parse_number
                              , parse_string
                              , parse_bool
                              , parse_namespace_symbol
                              ] )?;
    parse_trailing(input, expr)               
}

fn parse_trailing( input : &mut Input, init : Expr ) -> Result<Expr, ParseError> {
     
    match input.expect("[") {
        Ok(_) => {
            let index = Box::new(parse_inside_index(input)?); 
            return parse_trailing(input, Expr::Index { expr: Box::new(init), index });
        }, 
        Err(_) => (),
    }

    match input.expect(".") {
        Ok(_) => {
            let slot = input.parse_symbol()?;
            return parse_trailing(input, Expr::SlotAccess { expr: Box::new(init), slot });
        },
        Err(_) => (),
    }

    Ok(init)
}

fn parse_inside_index( input : &mut Input ) -> Result<Expr, ParseError> {
    fn parse_empty_slice( input : &mut Input ) -> Result<Expr, ParseError> {
        input.expect("..")?;
        input.expect("]")?;
        Ok(Expr::Slice { start: None, end: None })
    }

    fn parse_empty_start_slice( input : &mut Input ) -> Result<Expr, ParseError> {
        input.expect("..")?;
        let end = Some(Box::new(parse(input)?));
        input.expect("]")?;
        Ok(Expr::Slice { start: None, end })
    }

    fn parse_empty_end_slice( input : &mut Input ) -> Result<Expr, ParseError> {
        let start = Some(Box::new(parse(input)?));
        input.expect("..")?;
        input.expect("]")?;
        Ok(Expr::Slice { start, end: None })
    }

    fn parse_slice( input : &mut Input ) -> Result<Expr, ParseError> {
        let start = Some(Box::new(parse(input)?));
        input.expect("..")?;
        let end = Some(Box::new(parse(input)?));
        input.expect("]")?;
        Ok(Expr::Slice { start, end })
    }
    
    fn parse_no_slice( input : &mut Input ) -> Result<Expr, ParseError> {
        let e = parse(input)?;
        input.expect("]")?;
        Ok(e)
    }

    input.choice( &[ parse_empty_slice
                   , parse_empty_start_slice
                   , parse_empty_end_slice
                   , parse_slice
                   , parse_no_slice
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

#[cfg(test)]
mod test {
    use super::*;

    use rand::Rng;
    use super::super::random_ast;
    use super::super::unparse_ast;

    // TODO false_blarg symbol parses
    // TODO empty slice, no start, no end, x..y, only expr

    #[test]
    fn should_parse_random_exprs() -> Result<(), ParseError> {
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            random_ast::set_fuel( 20 );

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

