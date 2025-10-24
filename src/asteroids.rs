use std::collections::HashMap;
use std::any::Any;
use pelican_ui::*;
use runtime::{self, Service, ServiceList, ThreadContext, async_trait, Services};
use serde::de::Unexpected::Str;
use pelican_ui::drawable::{Component, Image};
use pelican_ui::events::{Event, OnEvent, TickEvent, KeyboardEvent, KeyboardState, NamedKey, Key, MouseEvent, MouseState};
use pelican_ui::drawable::{Shape, Color, Drawable, ShapeType, Align};
use pelican_ui::layout::{SizeRequest, Area, Layout, DefaultStack};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::time::Duration;

use pelican_ui_std::*;
pub struct TestApp;
impl Plugins for TestApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {vec![]}
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
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(nav_event) = event.downcast_ref::<NavEvent>() {
			self.1.display_left(nav_event.0);
			false
		} else {true}
	}
}

#[derive(Debug, Clone)]
pub struct NavEvent(bool);
impl Event for NavEvent{
	fn pass(self: Box<Self>, ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
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


#[derive(Debug, Component)]
pub struct Ship(Stack, Shape);
impl OnEvent for Ship {}
impl Ship {
	pub fn new(ctx: &mut Context) -> Self {
		Ship(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Rectangle(5.0, (55.0, 55.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
		)
	}
}

#[derive(Debug, Component)]
pub struct Asteroid(Stack, Shape);
impl OnEvent for Asteroid {}
impl Asteroid {
	pub fn new(ctx: &mut Context) -> Self {
		Asteroid(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Ellipse(5.0, (120.0, 120.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
		)
	}

	pub fn big(ctx: &mut Context) -> Self {
		Asteroid(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Ellipse(5.0, (80.0, 80.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
		)
	}

	pub fn medium(ctx: &mut Context) -> Self {
		Asteroid(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Ellipse(5.0, (60.0, 60.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
		)
	}

	pub fn small(ctx: &mut Context) -> Self {
		Asteroid(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Ellipse(5.0, (40.0, 40.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
		)
	}
}

#[derive(Debug, Component)]
pub struct Canvas(CanvasLayout, Vec<Asteroid>, Ship, Asteroid);
impl OnEvent for Canvas {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
		}
		true
	}
}

impl Canvas {
    pub fn new(ctx: &mut Context) -> Self {
        Canvas(
			CanvasLayout(vec![(300.0, 300.0), (20.0, 20.0), /*(140.0, 100.0),*/ (200.0, 20.0), (260.0, 200.0), /*(320.0, 120.0),*/ (20.0, 270.0), (-200.0, -200.0)]),
			vec![Asteroid::big(ctx), Asteroid::medium(ctx), /*Asteroid::small(ctx),*/ Asteroid::small(ctx), /*Asteroid::medium(ctx),*/ Asteroid::medium(ctx)],
			Ship::new(ctx),
			Asteroid::small(ctx),
		)
    }
}

//display ellipse when mouse is clicked
#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page, #[skip] (f32, f32), #[skip] (f32, f32), #[skip] bool);
impl OnEvent for FirstScreen {
fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
	if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		self.2 = (4.0, 4.0);
		let offset = self.1.content().find_at::<Canvas>(0).unwrap();
		let slices = &mut offset.0.0;
		for bruh in &mut slices[0..4] {
			println!("{:?}", bruh);
			let asteroids = (bruh.0 + self.2.0, bruh.1 + self.2.0);
			*bruh = asteroids;
		}
		if slices[0] > (1000.0, 1000.0) {
			slices[0] = (20.0, 20.0);
		}
		if slices[1] > (1000.0, 1000.0) {
			slices[1] = (200.0, 20.0);
		}
		if slices[2] > (1000.0, 1000.0) {
			slices[2] = (260.0, 200.0);
		}
		if slices[3] > (1000.0, 1000.0) {
			slices[3] = (260.0, 200.0);
		}
		for test in 0..10 {
			if self.4 == true {
				self.shoot(ctx);
			}
		}

	} else if let Some(KeyboardEvent{key: my_key, state: my_state}) = event.downcast_ref::<KeyboardEvent>() {
			//TODO:
			//COMPLETED: so maybe we have the asteroids loop back through if they reach a certain number. we'll try this for now and add a better system later since we'll be moving with our ship.
			//COMPLETED: make code cleaner and less hardcoded
			//COMPLETED: add ship movement
			//HALFWAY COMPLETED: create a bumper with the score and lives
			//HALFWAY COMPLETED: make ship shoot
			//create a way to automatically generate asteroids. definetely going to be using .push()
			//add asteroid collision and splitting into smaller asteroids
			//replace shapes with sprites
			//BUGS: any named key double presses
			//create death and respawn + update score board
			self.3 = (12.0, 12.0);
			let offset = self.1.content().find_at::<Canvas>(0).unwrap();
			let slices = &mut offset.0.0;
			match my_key {
				Key::Named(NamedKey::Space) => {
					self.4 = true;
				},
				Key::Named(NamedKey::ArrowUp) => {
					let up = (slices[4].1 - self.3.1);
					slices[4].1 = up;
					println!("{}", self.1.content().find_at::<Canvas>(0).unwrap().0.0[4].1);
				},
				Key::Named(NamedKey::ArrowDown) => {
					let down = (slices[4].1 + self.3.1);
					slices[4].1 = down;
				},
				Key::Named(NamedKey::ArrowRight) => {
					let right = (slices[4].0 + self.3.0);
					slices[4].0 = right;
				},
				Key::Named(NamedKey::ArrowLeft) => {
					let left = (slices[4].0 - self.3.0);
					slices[4].0 = left;
				}
				_ => {
					println!("wrong key press?");
				}
			}
			//let asteroid = (self.1.content().find_at::<Canvas>(0).unwrap().0.0[0..3].0 + self.2.0, self.1.content().find_at::<Canvas>(0).unwrap().0.0[0..3].1 + self.2.1);
			//maybe use filter to create certain conditions
		}
		true
	}
}

impl AppPage for FirstScreen {
	fn has_nav(&self) -> bool { true }
	fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Err(self) }
}

impl FirstScreen {
    pub fn new(ctx: &mut Context) -> Self {
		//let button = Button::new(ctx, None, None, None, None, ButtonSize::Medium, ButtonWidth::Expand, ButtonStyle::Primary, ButtonState::Default, Offset::Center, |ctx: &mut Context| {}, Some("Hello".to_string()));
		//let bumper = Bumper::single_button(ctx, button);
		let children: Vec<Box<dyn Drawable>> = vec![Box::new(Canvas::new(ctx)), Box::new(Bumper::new(ctx))];
		let content = Content::new(ctx, Offset::Center, children);
		let header = Header::home(ctx, "Canvas", None);
		FirstScreen(Stack::default(), Page::new(Some(header), content, None), (0.0, 0.0), (0.0, 0.0), false)
    }
	pub fn shoot(&mut self, ctx: &mut Context) {
		//how to make ship shoot? we could push a new offset and shape above the position of our ship? we would have to avoid hardcoding our ship's position so we'll need some sort of variable or we quite literally could just index into CanvasLayout lol
		//issues: we can't push properly cuz we can't self on CanvasLayout
		self.2 = (20.0, 20.0);
		let canvas = self.1.content().find_at::<Canvas>(0).unwrap();
		let offset = &mut canvas.0.0;
		let shape = &mut canvas.1[3].1.shape;
		println!("{:?}", shape);
		//delete all of this code and revisit it, it clearly isn't working.
		offset[5] = offset[4];
			let shoot = (offset[5].0 + self.2.0, offset[5].1 + self.2.1);
			offset[5] = shoot;
		//make sure that bullets spawn a little above the ship's front lol
		//make sure the bullets move
	}
	pub fn collision(&mut self, ctx: &mut Context) {
		//get the radius of asteroids/ships. add the width and height to the offset to get the center. check if ship.center() - asteroid.center() is within your radius
		let test: (f32, f32) = (260.0, 200.0);
		let test: (f32, f32) = (20.0, 270.0);
		let canvas = self.1.content().find_at::<Canvas>(0).unwrap();
		let offset = &mut canvas.0.0;
		let shape = &mut canvas.1;
		let asteroid_radius = (offset[3].0 + test.0, offset[3].1 + test.1);
		let ship_radius = (offset[4].0 + test.0, offset[4].1 + test.1);
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
         let color = ctx.theme.colors.text.heading;
         let icon = Icon::new(ctx, "down", color, 128.0);
         let child: Vec<Box<dyn Drawable>> = vec![Box::new(Canvas::new(ctx))];
         let header = Header::home(ctx, "CONGRATULATIONS", None);
         let content = Content::new(ctx, Offset::Center, child);
         SecondPage(Stack::default(), Page::new(Some(header), content, None))
     }
 }

#[derive(Debug)]
pub struct CanvasLayout(Vec<(f32, f32)>);//A vector of offsets (left, top)
impl Layout for CanvasLayout {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::new(0.0, 0.0, f32::MAX, f32::MAX)
    }

    fn build(&self, _ctx: &mut Context, size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        if self.0.len() != children.len() {panic!("CanvasLayout does not have the same number of offsets as children");}
        self.0.iter().copied().zip(children).map(|(offset, child)|
            Area{offset, size: child.get((size.0-offset.0, size.1-offset.1))}
        ).collect()
    }
}


