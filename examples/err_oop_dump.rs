trait Exp {
    fn eval(&self) -> u64;
}

struct Lit {
    x: u64,
}
impl Exp for Lit {
    fn eval(&self) -> u64 {
        self.x
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
}

// add dump
trait Dump {
    fn dump(&self) -> String;
}
impl Dump for Lit {
    fn dump(&self) -> String {
        format!("{}", self.x)
    }
}
impl Dump for Add {
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
