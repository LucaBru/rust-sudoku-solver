#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RuleType {
    Discovery,
    Clue,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub type_id: RuleType,
    pub digit: usize,
    pub row: usize,
    pub clm: usize,
}
