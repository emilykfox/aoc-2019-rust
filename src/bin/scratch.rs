pub trait Unwrapping {
    type UnwrapItem;

    fn unext(&mut self) -> Self::UnwrapItem;
    fn umap(&mut self) -> impl Iterator<Item = Self::UnwrapItem>;
}

mod private {
    use super::*;

    trait UnwrappingHelper<R> {
        type UnwrapItem;

        fn unext(&mut self) -> Self::UnwrapItem;
        fn umap(&mut self) -> impl Iterator<Item = Self::UnwrapItem>;
    }

    impl<T, R, I: Iterator<Item = R> + UnwrappingHelper<R, UnwrapItem = T>> Unwrapping for I {
        type UnwrapItem = T;

        fn unext(&mut self) -> Self::UnwrapItem {
            UnwrappingHelper::unext(self)
        }

        fn umap(&mut self) -> impl Iterator<Item = Self::UnwrapItem> {
            UnwrappingHelper::umap(self)
        }
    }

    impl<T, E: std::fmt::Debug, I: Iterator<Item = Result<T, E>>> UnwrappingHelper<Result<T, E>> for I {
        type UnwrapItem = T;

        fn unext(&mut self) -> T {
            self.next().unwrap().unwrap()
        }

        fn umap(&mut self) -> impl Iterator<Item = T> {
            self.map(Result::unwrap)
        }
    }

    impl<'a, T: 'a, E: 'a + std::fmt::Debug, I: Iterator<Item = &'a Result<T, E>>>
        UnwrappingHelper<&'a Result<T, E>> for I
    {
        type UnwrapItem = &'a T;

        fn unext(&mut self) -> &'a T {
            self.next().unwrap().as_ref().unwrap()
        }

        fn umap(&mut self) -> impl Iterator<Item = &'a T> {
            self.map(|item| item.as_ref().unwrap())
        }
    }

    impl<'a, T: 'a, E: 'a + std::fmt::Debug, I: Iterator<Item = &'a mut Result<T, E>>>
        UnwrappingHelper<&'a mut Result<T, E>> for I
    {
        type UnwrapItem = &'a mut T;

        fn unext(&mut self) -> &'a mut T {
            self.next().unwrap().as_mut().unwrap()
        }

        fn umap(&mut self) -> impl Iterator<Item = &'a mut T> {
            self.map(|item| item.as_mut().unwrap())
        }
    }
}

fn main() {
    let mut a: Vec<Result<String, String>> = vec![Ok("HYPE".into()); 5];
    for b in a.iter().umap() {
        println!("{b}")
    }
    for b in a.iter_mut().umap() {
        *b = "HOIP".into();
    }
    for b in a.into_iter().umap() {
        println!("{b}");
    }
}
