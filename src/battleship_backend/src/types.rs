use candid::{CandidType};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Game {
	pub home: String,
	pub away: Option<String>,
	#[serde(default)]
	pub home_ready: bool,
	#[serde(default)]
	pub away_ready: bool,
	pub home_board: HashMap<(u32, u32), (bool, bool)>,
	pub away_board: HashMap<(u32, u32), (bool, bool)>,
	pub creation_time: u32,
	pub is_over: bool,
	pub winner: String
}

#[derive(Default)]
#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct State {
	pub increment: u32,
	pub games: HashMap<u32, Game>
}
