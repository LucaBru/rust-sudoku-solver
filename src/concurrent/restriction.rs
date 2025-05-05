#[derive(Clone, PartialEq, Eq)]
pub enum RestrictionType {
    Discovery,
    Clue,
}

#[derive(Clone)]
pub struct Restriction {
    pub type_id: RestrictionType,
    pub digit: usize,
    pub row: usize,
    pub clm: usize,
}

impl Restriction {
    fn row(&self) -> usize {
        self.row
    }
    fn clm(&self) -> usize {
        self.clm
    }
    fn digit(&self) -> usize {
        self.digit
    }
    pub fn of_type(&self) -> RestrictionType {
        self.type_id.clone()
    }
}
