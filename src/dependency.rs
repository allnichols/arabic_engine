#[derive(Debug, Clone, PartialEq)]
pub enum Relation {
    Root, 
    Subject, 
    Object, 
    Predicate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dependency {
    pub head: usize,
    pub dependent: usize,
    pub relation: Relation,
}