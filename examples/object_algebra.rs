trait ExpAlg {
    type T;
    fn lit(x: u64) -> Self::T;
    fn add(l: Self::T, r: Self::T) -> Self::T;
}

struct ExpEval;

impl ExpAlg for ExpEval {
    type T = u64;
    fn lit(x: u64) -> Self::T {
        x
    }
    fn add(l: Self::T, r: Self::T) -> Self::T {
        l + r
    }
}

// add dump is easy
struct ExpDump;
impl ExpAlg for ExpDump {
    type T = String;
    fn lit(x: u64) -> Self::T {
        format!("{}", x)
    }
    fn add(l: Self::T, r: Self::T) -> Self::T {
        format!("{} + {}", l, r)
    }
}

// add mul
trait MulAlg {
    type T;
    fn mul(l: Self::T, r: Self::T) -> Self::T;
}

struct MulEval;

impl MulAlg for MulEval {
    type T = u64;
    fn mul(l: Self::T, r: Self::T) -> Self::T {
        l * r
    }
}

// mul for dump
struct MulDump;

impl MulAlg for MulDump {
    type T = String;
    fn mul(l: Self::T, r: Self::T) -> Self::T {
        format!("{} * {}", l, r)
    }
}

fn main() {
    let ret = ExpEval::add(ExpEval::lit(1), ExpEval::lit(2));
    println!("eval ret = {}", ret);

    let ret1 = ExpDump::add(ExpDump::lit(1), ExpDump::lit(2));
    println!("dump ret = {}", ret1);

    let ret2 = ExpEval::add(
        ExpEval::lit(1),
        MulEval::mul(ExpEval::lit(2), ExpEval::lit(3)),
    );
    println!("eval mul ret = {}", ret2);

    let ret3 = ExpDump::add(
        ExpDump::lit(1),
        MulDump::mul(ExpDump::lit(2), ExpDump::lit(3)),
    );
    println!("dump mul ret = {}", ret3);
}
