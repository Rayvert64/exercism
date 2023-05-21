pub struct MultiWriteIterator<'a, T> {
    buffer: &'a mut T,
    index: usize,
}

/// Methods that make it easy to write and read from a buffer usig
/// multiple threads or async tasks.
pub trait MultiWriteBuffer: IntoIterator
where
    Self::Item: std::fmt::Debug,
{
    fn get_multi_write_iterators<'a>(&'a mut self) -> Vec<MultiWriteIterator<'a, Self::Item>>;
    fn get_multi_write_iterators_closure<'a, F>(
        &'a mut self,
        f: F,
    ) -> Vec<MultiWriteIterator<'a, Self::Item>>
    where
        F: Fn(&mut Self::Item) -> ();
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        //no implementation yet
    }
}
