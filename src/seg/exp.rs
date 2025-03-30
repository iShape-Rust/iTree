#[derive(Debug, Clone, Copy)]
pub struct SegRange<R> {
    pub min: R,
    pub max: R
}

pub trait SegExpCollection<R, E, V> {
    fn insert(&mut self, range: SegRange<R>, val: V, time: E);
    type Iter<'a>: Iterator<Item = V>
    where
    Self: 'a;
    fn iter(&mut self, range: SegRange<R>, time: E) -> Self::Iter<'_>;
    fn clear(&mut self);
}

#[cfg(test)]
mod tests {


    #[test]
    fn test_00() {

    }
}