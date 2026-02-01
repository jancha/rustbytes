// no issue to solve, but rust tip
// #[inline] on small hot-path methods in traits
pub trait Measurable {
    fn name_len(&self) -> u64;
}

struct Person {
    name: String,
}

impl Measurable for Person {
    #[inline(always)]
    fn name_len(&self) -> u64 {
        self.name.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_len() {
        let p = Person {
            name: "John".to_string(),
        };

        assert_eq!(p.name_len(), 4)
    }
}
