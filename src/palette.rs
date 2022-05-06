use rand::seq::SliceRandom;

#[derive(Debug, PartialEq)]
pub struct Palette {
    pub colors: [String; 9],
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            colors: [
                String::from("#E07E69"),
                String::from("#536E8F"),
                String::from("#FED700"),
                String::from("#CB4204"),
                String::from("#EEE"),
                String::from("#56325C"),
                String::from("#9BA77C"),
                String::from("#88D4D1"),
                String::from("#DEAF48"),
            ],
        }
    }

    pub fn rand(&self) -> String {
        self.colors
            .choose(&mut rand::thread_rng())
            .expect("random color")
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_color() {
        let default = Palette::new();
        let new_palette = Palette::new();
        assert_eq!(new_palette, default);
        assert!(default.colors.contains(&new_palette.rand()));
    }
}
