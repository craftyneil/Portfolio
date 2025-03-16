use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone)]
pub enum PlayerChoice {
    Rock,
    Paper,
    Scissor,
}

impl PlayerChoice {
    pub fn get_new_choice(current_player_name: &str) -> PlayerChoice {
        loop {
            print!("{current_player_name}, choose between rock, paper and scissor: ");
            stdout().flush().unwrap();

            let mut player_choice = String::new();
            stdin().read_line(&mut player_choice).unwrap();

            let player_choice: Result<PlayerChoice, _> = player_choice.try_into();

            if let Ok(choice) = player_choice {
                break choice;
            } else {
                println!("{}", player_choice.unwrap_err());
            }
        }
    }
}

impl TryFrom<String> for PlayerChoice {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "rock" => Ok(PlayerChoice::Rock),
            "paper" => Ok(PlayerChoice::Paper),
            "scissor" => Ok(PlayerChoice::Scissor),
            &_ => Err("The choice was not written correctly".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    choice: PlayerChoice,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            // the initialised state
            choice: PlayerChoice::Rock,
        }
    }

    pub fn wins_against(&self, opponent: &Self) -> Option<bool> {
        match (&self.choice, &opponent.choice) {
            (PlayerChoice::Rock, PlayerChoice::Scissor)
            | (PlayerChoice::Paper, PlayerChoice::Rock)
            | (PlayerChoice::Scissor, PlayerChoice::Paper) => Some(true),
            (PlayerChoice::Paper, PlayerChoice::Scissor)
            | (PlayerChoice::Rock, PlayerChoice::Paper)
            | (PlayerChoice::Scissor, PlayerChoice::Rock) => Some(false),

            _ => None,
        }
    }

    pub fn get_new_name() -> String {
        print!("choose your name: ");
        stdout().flush().unwrap();

        let mut player_name = String::new();
        stdin().read_line(&mut player_name).unwrap();

        player_name.trim().to_string()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_choice(&mut self, new_choice: PlayerChoice) {
        self.choice = new_choice;
    }
}
