enum Exp {
    Lit(u64),
    Add(Box<Exp>, Box<Exp>),
}

impl Exp {
    fn eval(&self) -> u64 {
        match self {
            Exp::Lit(x) => *x,
            Exp::Add(l, r) => l.eval() + r.eval(),
        }
    }

    fn dump(&self) -> String {
        match self {
            Exp::Lit(x) => format!("{}", x),
            Exp::Add(l, r) => format!("{} + {}", l.dump(), r.dump()),
        }
    }
}

fn main() {
    let e = Exp::Add(Box::new(Exp::Lit(1)), Box::new(Exp::Lit(2)));
    println!("eval(e) = {}", e.eval());
    println!("dump(e) = {}", e.dump());
}
