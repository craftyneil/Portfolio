// use crate::player::Player;
// use rpassword::read_password;
// use serde::{Deserialize, Serialize};
// use std::{
//     collections::HashMap,
//     fs,
//     io::{stdout, Write},
//     path::Path,
// };

// #[derive(Serialize, Deserialize)]
// struct PlayerScore {
//     score: u32,
//     password: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ScoreBook {
//     players: HashMap<String, PlayerScore>,
// }

// impl ScoreBook {
//     pub fn init_save_file() {
//         let scorebook = ScoreBook {
//             players: HashMap::new(),
//         };
//         let json = serde_json::to_string_pretty(&scorebook).unwrap();
//         fs::write("scorebook.json", json).unwrap();
//     }

//     fn get_scorebook() -> ScoreBook {
//         let json_scorebook = fs::read_to_string("scorebook.json").unwrap();
//         serde_json::from_str(&json_scorebook).unwrap()
//     }

//     fn get_password(sentence: &str) -> String {
//         print!("{}", sentence);
//         stdout().flush().unwrap();
//         read_password().unwrap()
//     }

//     pub fn save_player_score(winner: &Player) {
//         //handle the possibility where the file does not exist
//         if !Path::new("scorebook.json").exists() {
//             ScoreBook::init_save_file();
//         }
//         let mut scorebook = ScoreBook::get_scorebook();

//         if let Some(player_score) = scorebook.players.get_mut(&winner.name) {
//             player_score.score += 1;
//         } else {
//             scorebook.players.insert(
//                 winner.name.clone(),
//                 PlayerScore {
//                     score: 1,
//                     password: ScoreBook::get_password(
//                         "Write the password that will be associated to your score",
//                     ),
//                 },
//             );
//         }

//         let updated_json_scorebook = serde_json::to_string_pretty(&scorebook).unwrap();
//         fs::write("scorebook.json", updated_json_scorebook).unwrap();
//     }
// }
