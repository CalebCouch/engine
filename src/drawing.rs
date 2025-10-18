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

//create row of Brushes for bottom bumper
//create bumper above that's able to change colors and size
//create a hex text input
//Bumper is a column of BumperRow
//BumperRow is a Row of Bumper Buttons
//have a u32 variable and match on it. if it's 1 we .set() Brush to Ellipse, 2 to Rectangle, etc.. to achieve this tho we need button events to be isolated from each other

//questions for caleb:
//how can we isolate events for each button
//how can we change how many buttons we have for each bumper
//could we use find_at and do the events in Firstscreen??

#[derive(Debug, Component)]
pub struct Hex(Stack, Shape, Text);
impl OnEvent for Hex {}
impl Hex {
	pub fn new(ctx: &mut Context) -> Self {
		Hex(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Rectangle(5.0, (180.0, 55.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
			Text::new(
				ctx,
				"INSERT HEX CODE",
				TextStyle::Primary,
				18.0,
				Align::Left,
			)
		)
	}
}

#[derive(Debug, Component)]
pub struct ButtonColor(Stack, Shape);
impl OnEvent for ButtonColor {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
			match *my_state {
				MouseState::Pressed => {
					ctx.state().set(Brush::RoundedRectangle);
				},
				MouseState::Moved => {

				},
				MouseState::Released => {

				},
				_ => {

				}
			};
		}
		true
	}
}

impl ButtonColor {
	pub fn new(ctx: &mut Context) -> Self {
		ButtonColor(Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)), Shape{shape: ShapeType::RoundedRectangle(0.0, (55.0, 55.0), 20.0, 0.0), color: Color::from_hex("#0000FF", 255)})
	}
}

#[derive(Debug, Component)]
pub struct ButtonColor2(Stack, Shape);
impl OnEvent for ButtonColor2 {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
			match *my_state {
				MouseState::Pressed => {
					ctx.state().set(Brush::Rectangle);
				},
				MouseState::Moved => {

				},
				MouseState::Released => {

				},
				_ => {

				}
			};
		}
		true
	}
}

impl ButtonColor2 {
	pub fn new(ctx: &mut Context) -> Self {
		ButtonColor2(Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)), Shape{shape: ShapeType::RoundedRectangle(0.0, (55.0, 55.0), 20.0, 0.0), color: Color::from_hex("#0000FF", 255)})
	}
}

#[derive(Debug, Component)]
pub struct ButtonColor3(Stack, Shape);
impl OnEvent for ButtonColor3 {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
			match *my_state {
				MouseState::Pressed => {
					ctx.state().set(Brush::Ellipse);
				},
				MouseState::Moved => {

				},
				MouseState::Released => {

				},
				_ => {

				}
			};
		}
		true
	}
}

impl ButtonColor3 {
	pub fn new(ctx: &mut Context) -> Self {
		ButtonColor3(Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)), Shape{shape: ShapeType::RoundedRectangle(0.0, (55.0, 55.0), 20.0, 0.0), color: Color::from_hex("#0000FF", 255)})
	}
}

#[derive(Debug, Component)]
pub struct ButtonSize(Stack, Shape);
impl OnEvent for ButtonSize {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
			match *my_state {
				MouseState::Pressed => {
					ctx.state().set(ShapeSize::Twenty);
				},
				MouseState::Moved => {

				},
				MouseState::Released => {

				},
				_ => {

				}
			};
		}
		true
	}
}

impl ButtonSize {
	pub fn new(ctx: &mut Context) -> Self {
		ButtonSize(Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)), Shape{shape: ShapeType::RoundedRectangle(0.0, (55.0, 55.0), 20.0, 0.0), color: Color::from_hex("#00FF00", 255)})
	}
}

#[derive(Debug, Component)]
pub struct ButtonSize2(Stack, Shape);
impl OnEvent for ButtonSize2 {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
			match *my_state {
				MouseState::Pressed => {
					ctx.state().set(ShapeSize::Fourty);
				},
				MouseState::Moved => {

				},
				MouseState::Released => {

				},
				_ => {

				}
			};
		}
		true
	}
}

impl ButtonSize2 {
	pub fn new(ctx: &mut Context) -> Self {
		ButtonSize2(Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)), Shape{shape: ShapeType::RoundedRectangle(0.0, (55.0, 55.0), 20.0, 0.0), color: Color::from_hex("#00FF00", 255)})
	}
}

#[derive(Debug, Component)]
pub struct ButtonSize3(Stack, Shape);
impl OnEvent for ButtonSize3 {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
			match *my_state {
				MouseState::Pressed => {
					ctx.state().set(ShapeSize::Sixty);
				},
				MouseState::Moved => {

				},
				MouseState::Released => {

				},
				_ => {

				}
			};
		}
		true
	}
}

impl ButtonSize3 {
	pub fn new(ctx: &mut Context) -> Self {
		ButtonSize3(Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)), Shape{shape: ShapeType::RoundedRectangle(0.0, (55.0, 55.0), 20.0, 0.0), color: Color::from_hex("#00FF00", 255)})
	}
}


#[derive(Debug, Component)]
pub struct BumperRow(Row, ButtonSize, ButtonSize2, ButtonSize3);
impl OnEvent for BumperRow {}
impl BumperRow {
	pub fn new(ctx: &mut Context) -> Self {
		BumperRow(Row::center(40.0), ButtonSize::new(ctx), ButtonSize2::new(ctx), ButtonSize3::new(ctx))
	}
}

#[derive(Debug, Component)]
pub struct BumperRowTwo(Row, ButtonColor, ButtonColor2, ButtonColor3);
impl OnEvent for BumperRowTwo {}
impl BumperRowTwo {
	pub fn new(ctx: &mut Context) -> Self {
		BumperRowTwo(Row::center(40.0), ButtonColor::new(ctx), ButtonColor2::new(ctx), ButtonColor3::new(ctx))
	}
}

#[derive(Debug, Component)]
pub struct Bumper(Stack, Shape, Vec<BumperRow>);
impl OnEvent for Bumper {}
impl Bumper {
	pub fn new(ctx: &mut Context) -> Self {
		Bumper(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Rectangle(0.0, (380.0, 55.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
			vec![BumperRow::new(ctx)]
		)
	}
}

#[derive(Debug, Component)]
pub struct BumperTwo(Stack, Shape, Vec<BumperRowTwo>);
impl OnEvent for BumperTwo {}
impl BumperTwo {
	pub fn new(ctx: &mut Context) -> Self {
		BumperTwo(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Rectangle(0.0, (380.0, 55.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
			vec![BumperRowTwo::new(ctx)]
		)
	}
}

#[derive(Debug, Component)]
pub struct Canvas(CanvasLayout, Vec<Shape>, #[skip] bool, #[skip] String, #[skip] bool);
impl OnEvent for Canvas {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
			println!("{:?}", my_position);
			println!("{:?}", my_state);
			if *my_state == MouseState::Pressed {
				self.2 = !self.2;
				println!("drawing is enabled {}", self.2);
			}
			if self.2 == true {
				//we need to be able to get the Shapeype without altering size values.
				let size = match *ctx.state().get_or_default::<ShapeSize>() {
					ShapeSize::Twenty => 20.0,
					ShapeSize::Fourty => 40.0,
					ShapeSize::Sixty => 60.0,
				};

				let shape = match *ctx.state().get_or_default::<Brush>() {
					Brush::Ellipse => ShapeType::Ellipse(0.0, (size, size), 0.0),
					Brush::Rectangle => ShapeType::Rectangle(0.0, (size, size), 0.0),
					Brush::RoundedRectangle => ShapeType::RoundedRectangle(0.0, (size, size), 20.0, 0.0),
				};
				self.0.0.push(*my_position);
				if self.4 == false {
					self.1.push(Shape{shape, color: Color::from_hex("#FFD700", 255),});
				} else {
					self.1.push(Shape{shape, color: Color::from_hex(self.3.as_str(), 255),});
				}
			}
		}
		true
	}
}
impl Canvas {
    pub fn new(ctx: &mut Context) -> Self {
        Canvas(
			CanvasLayout(vec![]),
			vec![],
			false,
			String::new(),
			false,
		)
    }
}

#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page, #[skip] String);
impl OnEvent for FirstScreen {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(KeyboardEvent{key: my_key, state: KeyboardState::Pressed}) = event.downcast_ref::<KeyboardEvent>() {
			if let Some(key) = my_key.to_text() {
				if self.1.content().find_at::<Canvas>(1).unwrap().3.len() < 7 {
					match my_key {
						k => {
							self.1.content().find_at::<Canvas>(1).unwrap().3.push_str(key);
							let text = Text::new(ctx, self.1.content().find_at::<Canvas>(1).unwrap().3.as_str(), TextStyle::Primary, 16.0, Align::Left);
							self.1.content().find_at::<Hex>(2).unwrap().2 = text;
						}
					}
				}
				if Key::Named(NamedKey::Enter) == *my_key {
					let hex = "#ABCDEF0123456789";
					let hex_collect: Vec<char> = hex.chars().collect();
					let input_collect: Vec<char> = self.1.content().find_at::<Canvas>(1).unwrap().3.to_uppercase().chars().collect();
					for (index, chars) in input_collect.iter().enumerate() {
						if !hex_collect.contains(chars) {
							break;
						}
						if index == self.1.content().find_at::<Canvas>(1).unwrap().3.len() - 1 {
						//self.1.content().find_at::<Canvas>(1).unwrap().1.push(Shape{shape, color: Color::from_hex(self.2.as_str(), 255)});
						self.1.content().find_at::<Canvas>(1).unwrap().4 = true;
						//set bool to true, if true set a variable to equal self.3 which we now insert as the argument for color
						//so we have to be able to CONSTANTLY push that new color and coordinate, meaning we can't have it blocked behind enter. my idea was that we have some sort of bool that when active always pushes the new color, not just once, but idk.

						}
					}
				}
				if Key::Named(NamedKey::Backspace) == *my_key {
                     self.1.content().find_at::<Canvas>(1).unwrap().3.pop();
                     self.1.content().find_at::<Canvas>(1).unwrap().3.pop();
                     let backspace = Text::new(ctx, self.1.content().find_at::<Canvas>(1).unwrap().3.as_str(), TextStyle::Primary, 16.0, Align::Left);
                     self.1.content().find_at::<Hex>(2).unwrap().2 = backspace;
                }
			}
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
		let children: Vec<Box<dyn Drawable>> = vec![Box::new(BumperTwo::new(ctx)), Box::new(Canvas::new(ctx)), Box::new(Hex::new(ctx)), Box::new(Bumper::new(ctx))];
		let content = Content::new(ctx, Offset::Center, children);
		let header = Header::home(ctx, "Canvas", None);
		FirstScreen(Stack::default(), Page::new(Some(header), content, None), String::new())
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
#[derive(Default, Debug)]
pub enum Brush {
	#[default]
	Ellipse,
	Rectangle,
	RoundedRectangle,
}

#[derive(Default, Debug)]
pub enum ShapeSize {
	#[default]
	Twenty,
	Fourty,
	Sixty,
}
