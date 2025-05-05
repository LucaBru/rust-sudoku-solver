use std::collections::{HashMap, HashSet};

enum Voter {
    Row,
    Clm,
    Box,
}

struct Election {
    candidates: HashSet<usize>,
    voters: HashMap<usize, HashMap<Voter, usize>>,
}
/*



*/
