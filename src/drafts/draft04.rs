use super::Draft;

#[derive(Debug, Default)]
pub struct Draft04;

impl Draft for Draft04 {
    fn new_boxed() -> Box<dyn Draft>
    where
        Self: Sized,
    {
        Box::new(Draft04)
    }
}
