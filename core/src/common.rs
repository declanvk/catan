use std::iter;
use rand;
use rand::Rng;
use std::collections::HashSet;
use std::hash::Hash;

pub trait GameResource: Sized + Clone + Copy + Eq + Hash {
    fn count(self: Self) -> usize;
    fn all_variants() -> HashSet<Self>;

    fn full_shuffled_collection() -> Vec<Self> {
        let mut full_deck = Self::all_variants()
            .into_iter()
            .flat_map(|variant| iter::repeat(variant).take(variant.count()))
            .collect::<Vec<Self>>();
        rand::thread_rng().shuffle(&mut full_deck);

        full_deck
    }
}
