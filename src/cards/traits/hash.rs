pub type ValueHashT = u32;

pub trait ValueHash {
    fn value_hash(&self) -> ValueHashT;
}

pub type UniqueHashT = u128;
pub trait UniqueHash {
    fn calculate_unique_hash_from_instance(&self) -> UniqueHashT {
        Self::calculate_unique_hash()
    }
    fn calculate_unique_hash() -> UniqueHashT {
        rand::random()
    }
    fn update_unique_hash(&mut self) {
        let hash = Self::calculate_unique_hash();
        self.set_unique_hash(hash);
    }
    fn unique_hash(&self) -> UniqueHashT;
    fn set_unique_hash(&mut self, hash: UniqueHashT);
}
