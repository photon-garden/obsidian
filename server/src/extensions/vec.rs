use nanorand::{Rng, WyRand};

pub trait VecExtension<Element> {
    fn random_element(&self) -> &Element;
}

impl<Element> VecExtension<Element> for Vec<Element> {
    fn random_element(&self) -> &Element {
        let mut rng = WyRand::new_seed(chrono::Utc::now().timestamp() as u64);
        let index = rng.generate_range(0..self.len());
        &self[index]
    }
}
