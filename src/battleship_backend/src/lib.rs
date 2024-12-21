mod types;

use types::*;
use std::collections::HashMap;

thread_local! {
	pub static STATE: std::cell::RefCell<State> = std::cell::RefCell::default();
}

////////////////////////////////////////////////////////////////////////////////////

#[ic_cdk::init]
fn init() {
	STATE.with(|state| {
		*state.borrow_mut() = State {
			increment: 0,
			games: HashMap::new()
		};
	});
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
	STATE.with(|state| {
		let state = state.borrow();
		ic_cdk::storage::stable_save((
			state.clone(),
		)).expect("Failed to save state (pre-upgrade)")
	})
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
	let (state,): (State,) = ic_cdk::storage::stable_restore().expect("Failed to fetch saved state (post-upgrade)");
	STATE.with(|s| {
		*s.borrow_mut() = state;
	})
}


////////////////////////////////////////////////////////////////////////////////////

#[ic_cdk::update]
fn create_game(home: String) -> u32 {
	STATE.with(|s| {
		let mut state = s.borrow_mut();
		state.increment += 1;
		let game_id = state.increment;
		let game = Game {
			home,
			away: None,
			home_board: HashMap::new(),
			away_board: HashMap::new(),
			creation_time: ic_cdk::api::time() as u32,
			is_over: false,
			winner: String::new(),
		};
		state.games.insert(game_id, game);
		game_id
	})
}

#[ic_cdk::query]
fn query_game_id(id: u32) -> bool {
	STATE.with(|s| {
		let state = s.borrow();
		state.games.contains_key(&id)
	})
}

#[ic_cdk::update]
fn join_game(id: u32, username: String) -> bool {
	STATE.with(|s| {
		let mut state = s.borrow_mut();
		if let Some(game) = state.games.get_mut(&id) {
			if game.away.is_none() {
				game.away = Some(username);
				true
			} else {
				false
			}
		} else {
			false
		}
	})
}

#[ic_cdk::update]
fn place_ships(game_id: u32, username: String, ships: Vec<(u32, u32)>) -> bool {
	STATE.with(|s| {
		let mut state = s.borrow_mut();
		if let Some(game) = state.games.get_mut(&game_id) {
			if game.home == username {
				for (x, y) in ships {
					game.home_board.insert((x, y), (true, false));
				}
				true
			} else if game.away.as_ref() == Some(&username) {
				for (x, y) in ships {
					game.away_board.insert((x, y), (true, false));
				}
				true
			} else {
				false
			}
		} else {
			false
		}
	})
}

#[ic_cdk::update]
fn shoot(game_id: u32, username: String, coordinate: (u32, u32)) -> bool {
	if coordinate.0 >= 8 || coordinate.1 >= 8 {
		return false
	}
	
	STATE.with(|s| {
		let mut state = s.borrow_mut();
		if let Some(game) = state.games.get_mut(&game_id) {
			if game.home == username {
				if let Some(cell) = game.home_board.get_mut(&coordinate) {
					cell.1 = true;
					cell.0
				} else { false }
			} else if game.away.as_ref() == Some(&username) {
				if let Some(cell) = game.away_board.get_mut(&coordinate) {
					cell.1 = true;
					cell.0
				} else { false }
			} else {
				false
			}
		} else {
			false
		}
	})
}

#[ic_cdk::query]
fn is_game_over(game_id: u32) -> Option<Game> {
	STATE.with(|s| {
		let mut state = s.borrow_mut();
		if let Some(game) = state.games.get_mut(&game_id) {
			let home_ships_sunk = game.home_board.values().all(|&(is_ship, is_shot)| !is_ship || (is_ship && is_shot));
			let away_ships_sunk = game.away_board.values().all(|&(is_ship, is_shot)| !is_ship || (is_ship && is_shot));

			if home_ships_sunk || away_ships_sunk {
				game.is_over = true;
				game.winner = if home_ships_sunk {
					game.away.clone().unwrap_or_default()
				} else {
					game.home.clone()
				};
				
				Some(game.clone())
			} else {
				None
			}
		} else {
			None
		}
	})
}

ic_cdk::export_candid!();
