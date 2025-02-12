#[derive(Clone)]
pub struct Side {
    color: [[String; 3]; 3],
}

impl Side {
    pub fn new(color: [[String; 3]; 3]) -> Self {
        Side { color }
    }

    pub fn get_color(&self) -> [[String; 3]; 3] {
        self.color.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let correct_orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let actual_side = Side::new(correct_orange.clone());

        let expected_side = correct_orange;

        assert_eq!(actual_side.get_color(), expected_side);

        let incorrect_orange = [
            ["OGY".to_string(), "Oy".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let actual_side = Side::new(incorrect_orange);

        assert_ne!(actual_side.get_color(), expected_side);
    }
}
