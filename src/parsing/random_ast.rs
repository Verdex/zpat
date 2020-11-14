
use super::ast::*;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use parse_input::PSym;

impl Distribution<Type> for Standard {
    fn sample<R : Rng + ?Sized>(&self, rng: &mut R) -> Type {
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(1, 9);

        match choice {
            1 => Type::Void,
            2 => {
                // TODO params
                Type::Fun { params: vec![], ret: Box::new(rng.gen::<Type>()) }
            },
            3 => Type::Array(Box::new(rng.gen::<Type>())),
            4 => Type::Generic(PSym { value: "name".to_string(), start: 0, end: 0 }), 
            5 => {
                // TODO params
                // TODO namespace symbol
                Type::Index { name: NamespaceSymbol { namespace: vec![]
                                                    , name: PSym { value: "name".to_string(), start: 0, end: 0 }
                                                    }
                            , params: vec![]
                            }
            },
            6 => Type::Simple( NamespaceSymbol { namespace: vec![], name: PSym { value: "name".to_string(), start: 0, end: 0 } } ),
            7 => Type::Dict { key: Box::new(rng.gen::<Type>()) 
                            , value: Box::new(rng.gen::<Type>())
                            },
            8 => Type::Void, // TODO row
            _ => panic!("Encountered random number out of range"),
        }
    }
}
