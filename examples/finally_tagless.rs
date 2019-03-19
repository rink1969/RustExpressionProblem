// finally tagless got same code in Rust
trait Expr {
    fn lit(x: u64) -> Self;
    fn add(l: Self, r: Self) -> Self;
}

// eval
impl Expr for u64 {
    fn lit(x: u64) -> Self {
        x
    }
    fn add(l: Self, r: Self) -> Self {
        l + r
    }
}

// add dump is easy
impl Expr for String {
    fn lit(x: u64) -> Self {
        format!("{}", x)
    }
    fn add(l: Self, r: Self) -> Self {
        format!("{} + {}", l, r)
    }
}

// add mul
trait MulExpr {
    fn mul(l: Self, r: Self) -> Self;
}
// eval mul
impl MulExpr for u64 {
    fn mul(l: Self, r: Self) -> Self {
        l * r
    }
}

// mul for dump
impl MulExpr for String {
    fn mul(l: Self, r: Self) -> Self {
        format!("{} * {}", l, r)
    }
}

fn main() {
    let ret: u64 = Expr::add(Expr::lit(1), Expr::lit(2));
    println!("eval ret = {}", ret);

    let ret1: String = Expr::add(Expr::lit(1), Expr::lit(2));
    println!("dump ret = {}", ret1);

    let ret2: u64 = Expr::add(Expr::lit(1), MulExpr::mul(Expr::lit(2), Expr::lit(3)));
    println!("eval mul ret = {}", ret2);

    let ret3: String = Expr::add(Expr::lit(1), MulExpr::mul(Expr::lit(2), Expr::lit(3)));
    println!("dump mul ret = {}", ret3);
}
