use rust_on_rails::prelude::*;

use std::collections::HashMap;
use crossterm::{execute, terminal::ClearType};
use std::io::{stdout, Write};
use std::time::Instant;
use dyn_clone::DynClone;
use dyn_clone::clone_trait_object;


pub enum Action{
	Shoot,
	Move(Coord),
	Remove,
}

#[derive(Clone, Debug)]
pub struct UpBullet{}
impl ShipTrait for UpBullet {
	fn draw(&self, ctx: &mut Context, x: u8, y: u8) {
		ctx.draw(CanvasItem::Shape(
			Area((SIZE * x as u32, SIZE * y as u32), None),
			Shape::Ellipse(0, (SIZE, SIZE)),
			"0000FF", 255
		));
	}
	fn get_next_move(&mut self, current_position: Coord) -> Action {
		match current_position.1.checked_sub(1) {
			Some(whatever) => {
				Action::Move((current_position.0, current_position.1-1))
			},
			None => {
				Action::Remove
			},
		}
	}
	fn is_enemy(&self) -> bool {
		false
	}
}

#[derive(Clone, Debug)]
pub struct DownBullet{}
impl ShipTrait for DownBullet {
	fn draw(&self, ctx: &mut Context, x: u8, y: u8) {
		ctx.draw(CanvasItem::Shape(
			Area((SIZE * x as u32, SIZE * y as u32), None),
			Shape::Ellipse(0, (SIZE, SIZE)),
			"ffD700", 255
		));
	}
	fn get_next_move(&mut self, current_position: Coord) -> Action {
		match current_position.1.checked_add(1) {
			Some(whatever) => {
				Action::Move((current_position.0, current_position.1+1))
			},
			None => {
				Action::Remove
			},
		}
	}
	fn is_enemy(&self) -> bool {
		false
	}
}

#[derive(Debug, Clone)]
pub struct Fly{shoot_next: bool}
impl ShipTrait for Fly {
	fn draw(&self, ctx: &mut Context, x: u8, y: u8) {
		ctx.draw(CanvasItem::Shape(
			Area((SIZE * x as u32, SIZE * y as u32), None),
			Shape::Ellipse(0, (SIZE, SIZE)),
			"008000", 255
		));
	}
	fn get_next_move(&mut self, current_position: Coord) -> Action {
		let switch = match self.shoot_next {
			true => {
				match current_position.1.checked_add(1) {
					Some(whatever) => {
						Action::Move((current_position.0+1, current_position.1))
					},
					None => {
						Action::Remove
					},
				}
			},
			false => {
				Action::Shoot
			},
		};
		self.shoot_next = !self.shoot_next;
		return switch;
	}
	fn is_enemy(&self) -> bool {
		true
	}
}
pub trait ShipTrait: std::fmt::Debug + DynClone {
	fn draw(&self, ctx: &mut Context, x: u8, y: u8);
	fn get_next_move(&mut self, current_position: Coord) -> Action;
	fn is_enemy(&self) -> bool;
}
clone_trait_object!(ShipTrait);
	//ships: Vec<Box<dyn ShipTrait>> = vec![Box::new(DownBullet{})];
pub type Coord = (u8, u8);
const BSIZE: u8 = 20;
const CSIZE: u8 = 19;
const SIZE: u32 = 30;

#[derive(Debug)]
struct MyApp {
	player_position: Coord,
	board: HashMap<Coord, Box<dyn ShipTrait>>, //Box<Ship> Trait
	last_tick: Instant,
	fly: HashMap<Coord, Box<dyn ShipTrait>>,
}

impl App for MyApp {
	async fn new(ctx: &mut Context) -> Self {
		MyApp{
        	board: HashMap::from([(
				(8, 10),
				Box::new(Fly{shoot_next: true}) as Box<dyn ShipTrait>
			)]),
			player_position: (10, 19),
			last_tick: Instant::now(),
			fly: HashMap::from([
				((2, 5), Box::new(Fly{shoot_next: true}) as Box<dyn ShipTrait>),
				((4, 5), Box::new(Fly{shoot_next: true}) as Box<dyn ShipTrait>),
				((6, 5), Box::new(Fly{shoot_next: true}) as Box<dyn ShipTrait>),
				((8, 5), Box::new(Fly{shoot_next: true}) as Box<dyn ShipTrait>),
				((10, 5), Box::new(Fly{shoot_next: true}) as Box<dyn ShipTrait>),
				((12, 5), Box::new(Fly{shoot_next: true}) as Box<dyn ShipTrait>),
				((14, 5), Box::new(Fly{shoot_next: true}) as Box<dyn ShipTrait>)
			]),
		}
	}
	async fn draw(&mut self, ctx: &mut Context) {
		if self.last_tick.elapsed().as_millis() > 1000 {
			self.last_tick = Instant::now();
			self.move_ships();
		}
		self.print(ctx)
	}
	async fn on_click(&mut self, ctx: &mut Context) {}
	async fn on_move(&mut self, ctx: &mut Context) {}
	//matches on a String type with multiple key bindings that can act as user movement.
	//the movement can only be enabled if it is within the boundries of the board.
	//the shoot key binding inserts an a bullet above the player position when pressed
	async fn on_press(&mut self, ctx: &mut Context, t: String) {
			match t.as_str() {
				"a" if self.player_position.0 > 0 => self.player_position = (self.player_position.0-1, self.player_position.1),
				"d" if self.player_position.0 < BSIZE - 1 => self.player_position = (self.player_position.0+1, self.player_position.1),
				"w" if self.player_position.1 > BSIZE - 3 => self.player_position = (self.player_position.0, self.player_position.1-1),
				"s" if self.player_position.1 < CSIZE => self.player_position = (self.player_position.0, self.player_position.1+1),
				"e" => {
					let above = self.board.insert((self.player_position.0, self.player_position.1-1), Box::new(UpBullet{}));
				}
				_ => {}
			};
			println!("{}", t);
		}
}
impl MyApp {
	///move_ships loops through all the ships in the board
	///asks each ship for its next position after passing its current position
	///checks the next position for a collision and moves or removes the ship
	//respawn if hit the edge
	//flying patterns
	//change the enum Action to an Action trait

	fn move_ships(&mut self) {
		if self.board.iter().any(|(_, ship)| ship.is_enemy()) {
				println!("they're still alive");
			} else {
				println!("they're all dead");
				self.board.extend(self.fly.clone());
			}
		let occupied: Vec<Coord> = self.board.keys()
			.copied()
			.collect();
		for coord in occupied {
			//matches on the optional Ship at the coordinate to check that it hasn't been destroyed.
			match self.board.remove(&coord) {
				Some(mut ship) => {
					match ship.get_next_move(coord) {
						Action::Shoot => {
							let below = self.board.insert((coord.0, coord.1+1), Box::new(DownBullet{}));
							self.add_char(coord, ship);
						},
						Action::Move(next_coord) => {
							if next_coord == self.player_position {
								panic!("HOUSTON WE HAVE A PROBLEM")
							};
							//Checks for a ship at the next coord and removes it if anything (_) is there, if nothing is there (None) move the Ship to the next coord.
							match self.board.get(&(next_coord)) {
								Some(_) => {
									self.remove_char(&next_coord);
								},
								None => {
									self.add_char(next_coord, ship);
								}
							}
						},
						Action::Remove => {},
					}
				}
				None => {}
			}
		}
	}
	//method of GameState that inserts a tuple of u8s key (Coord) and a value of char (ship). when called upon you must insert that same key and value types.
    fn add_char(&mut self, coord: Coord, ship: Box<dyn ShipTrait>) {
        self.board.insert(coord, ship);
	}
	//method of GameState that removes a value of the hashmap based on the coordinates given as a key.
	fn remove_char(&mut self, coord: &Coord) {
		self.board.remove(coord);
	}
	//first creates the backround for our board. then, it creates a for loop and another for loop inside of that one to create our 20x20 board. afterwards, x and y variables from the for loop our used to act a coordinate placeholder stored inside coord.
	//TODO: finish this later
	fn print(&self, ctx: &mut Context) {
		ctx.clear("000000");
        for y in 0..BSIZE {
            for x in 0..BSIZE {
				let coord = (x, y);
				match coord == self.player_position {
					true => {
						ctx.draw(CanvasItem::Shape(
							Area((SIZE * x as u32, SIZE * y as u32), None),
							Shape::Ellipse(0, (SIZE, SIZE)),
							"ffffff", 255
						));
					},
					false => {
						match self.board.get(&coord) {
							None => {
								ctx.draw(CanvasItem::Shape(
									Area((SIZE * x as u32, SIZE * y as u32), None),
									Shape::Ellipse(0, (SIZE, SIZE)),
									"ff00ff", 255
								));
							},
							Some(ship) => ship.draw(ctx, x, y)
						}
					}
				}
            }
        }
    }
}

//TODO: insert a bunch of Flies onto the board and make it so when you destory all of them they all respawn
create_entry_points!(MyApp);

fn main() {
	desktop_main()
}
