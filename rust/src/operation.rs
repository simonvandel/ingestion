pub enum Operation{
    Minus(f64, f64)
}

impl Operation {
    pub fn eval(&self) -> f64{
        match self {
            Operation::Minus(l, r) => l - r
        }
    }
}

