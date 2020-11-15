
use super::ast::*;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use parse_input::PSym;

const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";
const NUMS: &[u8] = b"1234567890";

fn gen_num<R : Rng + ?Sized>( rng : &mut R ) -> PSym {
    let value = (0..rng.gen_range(2, 5))
        .map(|_| {
            let vlet = rng.gen_range(0, NUMS.len());
            NUMS[vlet] as char
        }).collect::<String>();
    let end = value.len();
    PSym { value, start: 0, end }
}

fn gen_symbol<R : Rng + ?Sized>( rng : &mut R ) -> PSym {
    let value = (0..rng.gen_range(2, 5))
        .map(|_| {
            let vlet = rng.gen_range(0, CHARS.len());
            CHARS[vlet] as char
        }).collect::<String>();
    let end = value.len();
    PSym { value, start: 0, end }
}

fn gen_namespace_symbol<R : Rng + ?Sized>( rng : &mut R ) -> NamespaceSymbol {
    let name = gen_symbol(rng);
    let namespace = (0..rng.gen_range(1, 3))
        .map(|_| gen_symbol(rng))
        .collect::<Vec<PSym>>();
    NamespaceSymbol { name, namespace }
}

fn gen_vec<T, R : Rng + ?Sized, F : Fn(&mut R) -> T>( rng : &mut R, f : F, min : usize, max : usize ) -> Vec<T> {
    (0..rng.gen_range(min, max))
        .map(|_| f(rng))
        .collect()
}

fn gen_option<T, R : Rng + ?Sized, F : Fn(&mut R) -> T>( rng : &mut R, f : F) -> Option<T> {
    match rng.gen::<bool>() {
        true => Some(f(rng)),
        false => None,
    }
}

impl Distribution<Type> for Standard {
    fn sample<R : Rng + ?Sized>(&self, rng: &mut R) -> Type {
        let choice = rng.gen_range(1, 9);

        match choice {
            1 => Type::Void,
            2 => Type::Fun { params: gen_vec(rng, |r| r.gen::<Type>(), 0, 3)
                           , ret: Box::new(rng.gen::<Type>())
                           },
            3 => Type::Array(Box::new(rng.gen::<Type>())),
            4 => Type::Generic(gen_symbol(rng)), 
            5 => Type::Index { name: gen_namespace_symbol(rng)
                             , params: gen_vec(rng, |r| r.gen::<Type>(), 1, 3)
                             },
            6 => Type::Simple(gen_namespace_symbol(rng)),
            7 => Type::Dict { key: Box::new(rng.gen::<Type>()) 
                            , value: Box::new(rng.gen::<Type>())
                            },
            8 => Type::Row { params: gen_vec(rng, |r| (gen_symbol(r), r.gen::<Type>()), 1, 3)
                           , rest_name: gen_option(rng, |r| gen_symbol(r))
                           },
            _ => panic!("Encountered random number out of range for type"),
        }
    }
}

impl Distribution<Expr> for Standard {
    fn sample<R : Rng + ?Sized>(&self, rng: &mut R) -> Expr {
        let choice = rng.gen_range(1, 6);
        
        match choice {
            1 => Expr::Number(gen_num(rng)),
            2 => Expr::ZString(gen_symbol(rng)),
            3 => Expr::Bool(rng.gen::<bool>()),
            4 => Expr::Binding(gen_namespace_symbol(rng)),
            5 => Expr::Index { expr: Box::new(rng.gen::<Expr>())
                             , index: Box::new(rng.gen::<Expr>())
                             },
            _ => panic!("Encountered random number out of range for expr"),
        }
    }
}

/*
    Lambda { params: Vec<(PSym, Option<Type>)>, ret_type: Option<Type>, body: Box<Expr> },
    Slice { start: SliceOption, end: SliceOption }, 
    SlotAccess { expr: Box<Expr>, slot: PSym },
    FunCall { expr: Box<Expr>, params: Vec<Expr> },
    ExtensionFunCall { left: Box<Expr>, right: Box<Expr> },
    ArrayCons(Vec<Expr>),
    DictCons(Vec<(Expr, Expr)>), 
    ObjCons(Vec<(PSym, Expr)>),
    Let { name: (PSym, Option<Type>), params: Vec<(PSym, Option<Type>)>, value: Box<Expr>, body: Box<Expr> }, 
    Block { exprs: Vec<Expr> },

*/
