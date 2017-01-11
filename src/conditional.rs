pub trait Conditional {
    fn val(&self) -> bool;
}

impl Conditional for bool {
    fn val(&self) -> bool {
        *self
    }
}
