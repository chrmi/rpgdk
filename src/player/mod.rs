use super::dice;

/// # State container for the player.
pub struct Player {
    pub name: String,
    pub level: i16,
    pub x: i16,
    pub y: i16,
}

/// # Actions the Player can take in a given turn.
impl Player {
    pub fn move_xy(&mut self, x: i16, y: i16) {
        self.x += x;
        self.y += y;
        println!("{} has moved to {}, {}", self.name, self.x, self.y);
    }

    pub fn search(&self, difficulty:i16) -> bool {
        let roll = dice::roll(dice::Dice::D20(1, self.level));
        println!("Searching...");
        if roll > difficulty {
            println!("After searching around, {} found something!", self.name);
            return true;
        }
        println!("Hmmm, there doesn't seem to be anything here.");
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn player_creates() {
        let p = Player {
            name: String::from("TestName"), 
            level: 1, 
            x: 0,
            y: 0,
        };

        assert_eq!(p.name, "TestName", "name: {}", p.name);
        assert_eq!(p.level, 1, "level: {}", p.level);
        assert!(p.x == 0, "x: {}", p.x);
        assert!(p.y == 0, "y: {}", p.y);
    }

    #[test]
    fn player_moves_xy() {
        let mut p = Player {
            name: String::from("TestName"), 
            level: 1, 
            x: 0,
            y: 0,
        };

        p.move_xy(2, 3);

        assert!(p.x == 2, "x: {}", p.x);
        assert!(p.y == 3, "y: {}", p.y);
    }

    #[test]
    fn player_searches_success() {
         let p = Player {
            name: String::from("TestName"), 
            level: 1, 
            x: 0,
            y: 0,
        };

        let s = p.search(1);

        assert!(s, "search result: {}", s);
    }

    #[test]
    fn player_searches_fail() {
         let p = Player {
            name: String::from("TestName"), 
            level: 1, 
            x: 0,
            y: 0,
        };

        let s = p.search(22);

        assert!(!s, "search result: {}", s);
    }
}
