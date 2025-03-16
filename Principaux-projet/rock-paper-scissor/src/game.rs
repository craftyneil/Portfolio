use crate::player::{Player, PlayerChoice};

pub struct Game {
    players: [Player; 2],
}

impl Game {
    fn mut_player1(&mut self) -> &mut Player {
        &mut self.players[0]
    }

    pub fn player1(&self) -> &Player {
        &self.players[0]
    }

    fn mut_player2(&mut self) -> &mut Player {
        &mut self.players[1]
    }

    pub fn player2(&self) -> &Player {
        &self.players[1]
    }

    pub fn winner(&self) -> Option<&Player> {
        match self.player1().wins_against(self.player2()) {
            Some(true) => Some(self.player1()),
            Some(false) => Some(self.player2()),
            None => None,
        }
    }

    pub fn setup() -> Self {
        let player1_name = Player::get_new_name();
        let player2_name = Player::get_new_name();
        Self {
            players: [Player::new(player1_name), Player::new(player2_name)],
        }
    }

    pub fn run(&mut self) {
        // Get player names first
        let player1_name = self.player1().name().to_string();
        let player2_name = self.player2().name().to_string();

        self.mut_player1()
            .set_choice(PlayerChoice::get_new_choice(&player1_name));

        self.mut_player2()
            .set_choice(PlayerChoice::get_new_choice(&player2_name));

        let winner = self.winner();

        let winner = match winner {
            Some(player) => player.name(),
            None => "None",
        };

        println!("The winner is {winner:?}");
    }
}
