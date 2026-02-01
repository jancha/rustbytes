// no issue to solve, but rust tip
// non exhayustive - prevents direct initialization

// Non-exhaustive structs can be constructed as normal within the defining crate.

#[non_exhaustive]
pub struct Person {
    pub name: String,
}

impl Person {
    pub fn new(name: String) -> Person {
        Person { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_ok() {
        let p = Person::new("John".to_string());
        assert!(p.name == "John")
    }
}
