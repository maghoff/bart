pub trait Conditional {
    fn val(&self) -> bool;
}

impl Conditional for bool {
    fn val(&self) -> bool {
        *self
    }
}

impl<T> Conditional for Vec<T> {
    fn val(&self) -> bool {
        !self.is_empty()
    }
}

impl<'a, T> Conditional for &'a [T] {
    fn val(&self) -> bool {
        !self.is_empty()
    }
}

impl<'a, T: Conditional> Conditional for &'a T {
    fn val(&self) -> bool {
        (*self).val()
    }
}
