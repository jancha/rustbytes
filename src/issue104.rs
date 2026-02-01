// bool::then: Lazy Option Creation

// for fun, custom implementaion of then
trait ThenExt<T> {
    fn then_ext(&self, f: fn() -> T) -> Option<T>;
}

impl<T> ThenExt<T> for bool {
    fn then_ext(&self, f: fn() -> T) -> Option<T> {
        if *self {
            Some(f())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_then() {
        let should_have_name = true;

        let name: Option<String> = should_have_name.then(|| "John".to_string());
        let name2: Option<String> = should_have_name.then_ext(|| "John".to_string());

        assert_eq!(name, name2)
    }
}
