use std::iter::*;

pub trait NegativeIterator {
    type Item;
    type IntoIter : Iterator;

    fn neg_iter(&self) -> Self::IntoIter;
}

impl<T> NegativeIterator for Option<T> {
    type Item = ();
    type IntoIter = <Option<()> as IntoIterator>::IntoIter;

    fn neg_iter(&self) -> Self::IntoIter {
        match self {
            &Some(_) => None,
            &None => Some(())
        }.into_iter()
    }
}

impl<'a, T, E> NegativeIterator for &'a Result<T, E> {
    type Item = &'a E;
    type IntoIter = <Option<&'a E> as IntoIterator>::IntoIter;

    fn neg_iter(&self) -> Self::IntoIter {
        match *self {
            &Ok(_) => None,
            &Err(ref err) => Some(err)
        }.into_iter()
    }
}

// TODO impl for Vec<_> that checks if len() == 0
//  -> Maybe tell people to use negative conditional instead {{^vec?}}...
// TODO impl for [T]

#[cfg(test)]
mod test {
    use super::NegativeIterator;

    #[test]
    fn option_some() {
        for _ in Some(5).neg_iter() {
            panic!("Should not iterate");
        }
    }

    #[test]
    fn option_none() {
        let mut iterations = 0;
        let option: Option<i32> = None;
        for _ in option.neg_iter() {
            iterations += 1;
        }
        assert_eq!(1, iterations);
    }

    #[test]
    fn result_ok() {
        let result: Result<i32, i32> = Ok(5);
        for _ in (&result).neg_iter() {
            panic!("Should not iterate");
        }
    }

    #[test]
    fn result_err() {
        let mut iterations = 0;
        let result: Result<i32, i32> = Err(5);
        for ref x in (&result).neg_iter() {
            iterations += 1;
            assert_eq!(&&5, x);
        }
        assert_eq!(1, iterations);
    }
}
