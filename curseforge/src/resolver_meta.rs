use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct LocalDependency {
    pub id: i64,
    pub url: String,
}

impl Hash for LocalDependency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for LocalDependency {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for LocalDependency {}
