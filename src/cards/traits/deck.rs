pub trait Deck {
    fn card_num(&self) -> usize;
    fn swap(&mut self, idx1: usize, idx2: usize);
    fn shuffle(&mut self) {
        self.shuffle_from(0);
    }
    fn shuffle_from(&mut self, start_idx: usize) {
        use rand::Rng;
        let card_num = self.card_num();
        for idx in start_idx..card_num {
            let target_idx = rand::thread_rng().gen_range(idx..card_num);
            self.swap(idx, target_idx)
        }
    }
}
