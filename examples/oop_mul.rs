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

// for oop style
// add constructor is easy
struct Mul {
    l: Box<Exp>,
    r: Box<Exp>,
}
impl Exp for Mul {
    fn eval(&self) -> u64 {
        self.l.eval() * self.r.eval()
    }
}

fn main() {
    let e = Add {
        l: Box::new(Lit { x: 1 }),
        r: Box::new(Lit { x: 2 }),
    };
    println!("eval(e) = {}", e.eval());

    let e1 = Add {
        l: Box::new(Lit { x: 1 }),
        r: Box::new(Mul {
            l: Box::new(Lit { x: 2 }),
            r: Box::new(Lit { x: 3 }),
        }),
    };
    println!("eval(e1) = {}", e1.eval());
}
