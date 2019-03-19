trait Exp {
    fn eval(&self) -> u64;
    fn dump(&self) -> String;
}

struct Lit {
    x: u64,
}
impl Exp for Lit {
    fn eval(&self) -> u64 {
        self.x
    }
    fn dump(&self) -> String {
        format!("{}", self.x)
    }
}

struct Add {
    l: Box<Exp>,
    r: Box<Exp>,
}
impl Exp for Add {
    fn eval(&self) -> u64 {
        self.l.eval() + self.r.eval()
    }
    fn dump(&self) -> String {
        format!("{} + {}", self.l.dump(), self.r.dump())
    }
}

fn main() {
    let e = Add {
        l: Box::new(Lit { x: 1 }),
        r: Box::new(Lit { x: 2 }),
    };
    println!("eval(e) = {}", e.eval());
    println!("dump(e) = {}", e.dump());
}
