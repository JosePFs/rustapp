#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Patch<T> {
    Omit,
    Clear,
    Set(T),
}

impl<T> Patch<T> {
    pub fn is_omit(&self) -> bool {
        matches!(self, Patch::Omit)
    }
}
