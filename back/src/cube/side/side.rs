#[derive(Clone)]
pub struct Side {
    color: [[String; 3]; 3],
}

impl Side {
    pub fn new(color: [[String; 3]; 3]) -> Self {
        Side { color }
    }

    pub fn set_color(&mut self, color: [[String; 3]; 3]) {
        self.color = color;
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

    #[test]
    fn test_set_side() {
        let orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let mut actual_side = Side::new(orange);

        let red = [
            ["RBY".to_string(), "RY".to_string(), "RYG".to_string()],
            ["RB".to_string(), "R".to_string(), "RG".to_string()],
            ["RWB".to_string(), "RW".to_string(), "RGW".to_string()],
        ];
        actual_side.set_color(red.clone());

        let expected_side = red;

        assert_eq!(actual_side.get_color(), expected_side);

        let orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let expected_side = orange;

        assert_ne!(actual_side.get_color(), expected_side);
    }
}
