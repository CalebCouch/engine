use rand::rng;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use pelican_ui::*;
use runtime::{self, /*Service, ServiceList, ThreadContext, async_trait,*/ Services};
use pelican_ui::drawable::{Component};
use pelican_ui::events::{Event, OnEvent, TickEvent, KeyboardEvent, KeyboardState, NamedKey, Key, /*MouseEvent, MouseState*/};
use pelican_ui::drawable::{Shape, Color, Drawable, ShapeType, Align};
use pelican_ui::layout::{SizeRequest, Area, Layout, /*DefaultStack*/};

use pelican_ui_std::*;
pub struct TestApp;
impl Plugins for TestApp {
    fn plugins(_ctx: &mut Context) -> Vec<Box<dyn Plugin>> {vec![]}
}
impl Services for TestApp {}

impl Application for TestApp {
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
		ctx.theme = Theme::new(
			ColorResources::new(
				BackgroundColor{
					primary: Color::from_hex("808080", 255),
    				secondary: Color::from_hex("FFFFFF", 255),
				},
				OutlineColor::default(),
				StatusColor::default(),
				TextColor{heading: Color::from_hex("#FF006E", 255), primary: Color::from_hex("#000000", 255), secondary: Color::from_hex("#FFFFFF", 255)},
				BrandColor::default(),
				ButtonColors::default(),
				IllustrationColors::default(),
			),
			FontResources::default(&mut ctx.assets),
			IconResources::default(&mut ctx.assets),
			BrandResources::default(&mut ctx.assets),
			LayoutResources::default(),
		);
		let first = FirstScreen::new(ctx);
        let first = Interface::new(ctx, Box::new(first), None, None);
		let second = SecondPage::new(ctx);
 		let second = Interface::new(ctx, Box::new(second), None, None);
        Box::new(CustomNavigation(Stack::default(), EitherOr::new(first, second)))
        /*Box::new(Shape{
            shape: ShapeType::Ellipse(0.0, (400.0, 400.0), 0.0),
            color: Color(0, 0, 255, 255)
        })*/
    }
}

#[derive(Debug, Component)]
pub struct CustomNavigation(Stack, EitherOr<Interface, Interface>);
impl OnEvent for CustomNavigation{
	fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(nav_event) = event.downcast_ref::<NavEvent>() {
			self.1.display_left(nav_event.0);
			false
		} else {true}
	}
}

#[derive(Debug, Clone)]
pub struct NavEvent(bool);
impl Event for NavEvent{
	fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
		children.iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
	}
}

#[derive(Debug, Component)]
pub struct ScoreBoard(Stack, Shape, Text);
impl OnEvent for ScoreBoard {}
impl ScoreBoard {
	pub fn new(ctx: &mut Context) -> Self {
		ScoreBoard(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Rectangle(0.0, (80.0, 55.0), 0.0),
				color: Color::from_hex("#FFFFFF", 255),
			},
			Text::new(
				ctx,
				"SCORE:",
				TextStyle::Primary,
				20.0,
				Align::Left,
			)
		)
	}
	pub fn lives(ctx: &mut Context) -> Self {
		ScoreBoard(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Rectangle(0.0, (80.0, 55.0), 0.0),
				color: Color::from_hex("#FFFFFF", 255),
			},
			Text::new(
				ctx,
				"LIVES:",
				TextStyle::Primary,
				20.0,
				Align::Left,
			)
		)
	}
}

#[derive(Debug, Component)]
pub struct BumperRow(Row, ScoreBoard, ScoreBoard);
impl OnEvent for BumperRow {}
impl BumperRow {
	pub fn new(ctx: &mut Context) -> Self {
		BumperRow(
			Row::center(10.0),
			ScoreBoard::new(ctx),
			ScoreBoard::lives(ctx),
		)
	}
}


#[derive(Debug, Component)]
pub struct Bumper(Stack, Shape, BumperRow);
impl OnEvent for Bumper {}
impl Bumper {
	pub fn new(ctx: &mut Context) -> Self {
		Bumper(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Rectangle(0.0, (380.0, 55.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
			BumperRow::new(ctx),
		)
	}
}

//cloning a resources image is cheap. ctx.add_image is expensive
#[derive(Debug, Component)]
pub struct Ship(Stack, Shape);
impl OnEvent for Ship {}
impl Ship {
	pub fn new(_ctx: &mut Context) -> Self {
		Ship(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Ellipse(5.0, (55.0, 55.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
		)
	}
}

//one constructor function that accepts the width/height
#[derive(Debug, Component)]
pub struct Asteroid(Stack, Shape);
impl OnEvent for Asteroid {}
impl Asteroid {
	pub fn new(_ctx: &mut Context, height: f32, width: f32) -> Self {
		Asteroid(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Ellipse(5.0, (width, height), 0.0),
				color: Color::from_hex("#000000", 255),
			},
		)
	}
}

#[derive(Debug, Component)]
pub struct Canvas(CanvasLayout, Vec<Ship>, Vec<Asteroid>, Asteroid);
impl OnEvent for Canvas {}

//move ship to front
impl Canvas {
    pub fn new(ctx: &mut Context) -> Self {
        Canvas(
			CanvasLayout(vec![(160.0, 200.0), (300.0, 300.0), (20.0, 20.0), /*(140.0, 100.0),*/ (200.0, 20.0), (260.0, 200.0), /*(320.0, 120.0),*/ (-200.0, -200.0)]),
			vec![Ship::new(ctx)],
			vec![Asteroid::new(ctx, 80.0, 80.0), Asteroid::new(ctx, 60.0, 60.0), Asteroid::new(ctx, 40.0, 40.0), Asteroid::new(ctx, 60.0, 60.0)],
			Asteroid::new(ctx, 40.0, 40.0),
		)
    }
}

#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page, #[skip] (f32, f32), #[skip] (f32, f32), #[skip] bool);
impl OnEvent for FirstScreen {
fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
	if let Some(_tick_event) = event.downcast_ref::<TickEvent>() {
		self.2 = (2.0, 2.0);
		let canvas = self.1.content().find_at::<Canvas>(0).unwrap();
		let offset = &mut canvas.0.0[1..];
		for elements in &mut *offset {
			let asteroids = (elements.0 + self.2.0, elements.1 + self.2.0);
			*elements = asteroids;
		}
				if offset[1] > (1000.0, 1000.0) {
			offset[1] = (200.0, 20.0);
		}
		if offset[2] > (1000.0, 1000.0) {
			offset[2] = (260.0, 200.0);
		}
		if offset[3] > (1000.0, 1000.0) {
			offset[3] = (260.0, 200.0);
		}
		if offset[4] > (1000.0, 1000.0) {
			offset[4] = (20.0, 20.0);
		}
	} else if let Some(KeyboardEvent{key: my_key, state: KeyboardState::Pressed}) = event.downcast_ref::<KeyboardEvent>() {
			//TODO:
			//COMPLETED: so maybe we have the asteroids loop back through if they reach a certain number. we'll try this for now and add a better system later since we'll be moving with our ship.
			//COMPLETED: make code cleaner and less hardcoded
			//COMPLETED: add ship movement
			//COMPLETED: set the ship to be the center of the screen and when i hit the arrow keys move all the asteroids.
			//HALFWAY COMPLETED: create a bumper with the score and lives
			//HALFWAY COMPLETED: make ship shoot
			//HALFWAY COMPLETED: add asteroid collision and splitting into smaller asteroids
			//HALFWAY COMPLETED: create a way to automatically generate asteroids. definetely going to be using .push()
			//create death and respawn + update score board
			//replace shapes with sprites
			//figure out the front facing part of our ship is
			//add rotation
			//get rid of the looping asteroids and push new ones automatically. use weighted index function i created to make it so the chances of each variant of asteroid are different
			//BUGS: any named key double presses, all offsets move in the asteroid move code, including the ship's
			self.3 = (20.0, 20.0);
			let canvas = self.1.content().find_at::<Canvas>(0).unwrap();
			let offset = &mut canvas.0.0;
			match my_key {
				Key::Named(NamedKey::Space) => {
					//self.shoot(ctx);
					self.collision(ctx);
					self.generate_asteroids(ctx);
					self.shoot(ctx);
				},
				Key::Named(NamedKey::ArrowUp) => {
					//slices[0].1 = (offset[0].1 - self.3.1);
					for elements in &mut offset[1..] {
						let asteroids = elements.1 + self.3.1;
						elements.1 = asteroids;
					}
				},
				Key::Named(NamedKey::ArrowDown) => {
					//slices[0].1 = (offset[0].1 + self.3.1);
					for elements in &mut offset[1..] {
						let asteroids = elements.1 - self.3.1;
						elements.1 = asteroids;
					}
				},
				Key::Named(NamedKey::ArrowRight) => {
					//slices[0].0 = (offset[0].0 + self.3.0);
					for elements in &mut offset[1..] {
						let asteroids = elements.0 - self.3.1;
						elements.0 = asteroids;
					}
				},
				Key::Named(NamedKey::ArrowLeft) => {
					//slices[0].0 = (offset[0].0 - self.3.0);
					for elements in &mut offset[1..] {
						let asteroids = elements.0 + self.3.1;
						elements.0 = asteroids;
					}
				}
				_ => {
					println!("wrong key press?");
				}
			}
			//maybe use filter to create certain conditions
		}
		true
	}
}

impl AppPage for FirstScreen {
	fn has_nav(&self) -> bool { true }
	fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Err(self) }
}
//ctx.assets.add_image()
impl FirstScreen {
    pub fn new(ctx: &mut Context) -> Self {
		let children: Vec<Box<dyn Drawable>> = vec![Box::new(Canvas::new(ctx)), Box::new(Bumper::new(ctx))];
		let content = Content::new(ctx, Offset::Center, children);
		let header = Header::home(ctx, "Canvas", None);
		FirstScreen(Stack::default(), Page::new(Some(header), content, None), (0.0, 0.0), (0.0, 0.0), false)
    }

	pub fn shoot(&mut self, _ctx: &mut Context) {
		self.2 = (2.0, 2.0);
		let canvas = self.1.content().find_at::<Canvas>(0).unwrap();
		let offset = &mut canvas.0.0;
		//let shape = &mut canvas.3.1.shape;
		offset[5] = offset[0];
		let shoot = (offset[5].0 + self.2.0, offset[5].1 + self.2.1);
		offset[5] = shoot;

	}

	pub fn get_size(&mut self, _ctx: &mut Context) -> Vec<(f32, f32)> {
		let canvas = self.1.content().find_at::<Canvas>(0).unwrap();
		let shape = &mut canvas.2;
		let mut store: Vec<(f32, f32)> = vec![];
		for elements in &mut shape.iter() {
			match elements.1.shape {
				ShapeType::Ellipse(_stroke_width, (width, height), _rotation) => {
					store.push((width, height));
				},
				ShapeType::Rectangle(_stroke_width, (width, height), _rotation) => {
					store.push((width, height));
				},
				ShapeType::RoundedRectangle(_stroke_width, (width, height), _corner_radius, _rotation) => {
					store.push((width, height));
				},
			}
		}
		store
	}

	pub fn generate_asteroids(&mut self, ctx: &mut Context) {
		let canvas = self.1.content().find_at::<Canvas>(0).unwrap();
		let offset = &mut canvas.0.0;
		let shape = &mut canvas.2;
		let mut rng = rng();
		let asteroids = vec![
			(Asteroid::new(ctx, 80.0, 80.0), 1),
			(Asteroid::new(ctx, 60.0, 60.0), 3),
			(Asteroid::new(ctx, 40.0, 40.0), 5),
		];
		let a_weight = WeightedIndex::new(asteroids.iter().map(|x| x.1)).unwrap();

		let positions = vec![
			((200.0, 200.0), 1),
			((100.0, 100.0), 3),
			((50.0, 50.0), 5),
		];
		let p_weight = WeightedIndex::new(positions.iter().map(|x| x.1)).unwrap();
		//i could try cloning

		offset.push(positions[p_weight.sample(&mut rng)].0);
		shape.push(asteroids[a_weight.sample(&mut rng)].0);

		offset.push(positions[p_weight.sample(&mut rng)].0);
		shape.push(asteroids[a_weight.sample(&mut rng)].0);

		offset.push(positions[p_weight.sample(&mut rng)].0);
		shape.push(asteroids[a_weight.sample(&mut rng)].0);
	}
	pub fn collision(&mut self, ctx: &mut Context) {
		let sizes = self.get_size(ctx);
		let canvas = self.1.content().find_at::<Canvas>(0).unwrap();
		let offset = &mut canvas.0.0;
		//let shape = &mut canvas.1;
		for (index, (asteroid_height, asteroid_width)) in sizes.iter().enumerate() {
			let ship_radius: f32 = 27.5;
			let ship_size: (f32, f32) = (55.0, 55.0);
			let ship_center = (offset[0].0 + ship_size.0 / 2.0, offset[0].1 + ship_size.1 / 2.0);

			let asteroid_radius = asteroid_height / 2.0;
			let asteroid_center = (offset[index].0 + asteroid_height / 2.0, offset[index].1 + asteroid_width / 2.0);

			let distance_x = (ship_center.0 - asteroid_center.0).abs();
			let distance_y = (ship_center.1 - asteroid_center.1).abs();
			let radii = ship_radius + asteroid_radius;
			if distance_x < radii && distance_y < radii {
					println!("the distance was checked");
					//offset.remove(0);
					//shape.remove(0);
					//collision logic: remove both the asteroid and ship. asteroid is gonna be tricky
					//update lives
					//create explosion component and replace Ship's position with explosion (will be just a shape for now)
					//add Ship back (create limiter: if lives == 0, don't spawn and end game)
			}
			/*println!("this is the asteroid height {}", asteroid_height);
			println!("this is the asteroid width {}", asteroid_width);
			println!("this is the ship center {:?}", ship_center);
			println!("this is the asteroid radius {}", asteroid_radius);
			println!("this is the asteroid center {:?}", asteroid_center);
			println!("this is the distance x {}", distance_x);
			println!("this is the distance y {}", distance_y);
			println!("NEXT ASTEROID STATS");*/
		}
	}
}

#[derive(Debug, Component)]
 pub struct SecondPage(Stack, Page);
 impl OnEvent for SecondPage {}
 impl AppPage for SecondPage {
     fn has_nav(&self) -> bool { true }
     fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Err(self) }
 }

 impl SecondPage {
     pub fn new(ctx: &mut Context) -> Self {
         let child: Vec<Box<dyn Drawable>> = vec![Box::new(Canvas::new(ctx))];
         let header = Header::home(ctx, "CONGRATULATIONS", None);
         let content = Content::new(ctx, Offset::Center, child);
         SecondPage(Stack::default(), Page::new(Some(header), content, None))
     }
 }

#[derive(Debug)]
pub struct CanvasLayout(Vec<(f32, f32)>);//A vector of offsets (left, top)
impl Layout for CanvasLayout {
    fn request_size(&self, _ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::new(0.0, 0.0, f32::MAX, f32::MAX)
    }

    fn build(&self, _ctx: &mut Context, size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        if self.0.len() != children.len() {panic!("CanvasLayout does not have the same number of offsets as children");}
        self.0.iter().copied().zip(children).map(|(offset, child)|
            Area{offset, size: child.get((size.0-offset.0, size.1-offset.1))}
        ).collect()
    }
}


