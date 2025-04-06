pub mod is_equal;
pub mod is_not_equal;

pub mod string;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let a = "he";
        let b = "hi";

        string::matches_length(&a, &b);
    }
}
