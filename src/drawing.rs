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
				shape: ShapeType::Rectangle(5.0, (55.0, 55.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
			Text::new(
				ctx,
				" ",
				TextStyle::Primary,
				50.0,
				Align::Left,
			)
		)
	}
}

#[derive(Debug, Component)]
pub struct Button(Stack, Shape);
impl OnEvent for Button {
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

impl Button {
	pub fn new(ctx: &mut Context) -> Self {
		Button(Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)), Shape{shape: ShapeType::RoundedRectangle(0.0, (55.0, 55.0), 20.0, 0.0), color: Color::from_hex("#0000FF", 255)})
	}
}
#[derive(Debug, Component)]
pub struct BumperRow(Row, Vec<Button>);
impl OnEvent for BumperRow {}
impl BumperRow {
	pub fn new(ctx: &mut Context) -> Self {
		BumperRow(Row::center(40.0), vec![Button::new(ctx), Button::new(ctx)])

	}
}

#[derive(Debug, Component)]
pub struct Bumper(Stack, Shape, Vec<BumperRow>);
impl OnEvent for Bumper {
/*fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
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
	}*/
}

impl Bumper {
	pub fn new(ctx: &mut Context) -> Self {
		Bumper(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Shape{
				shape: ShapeType::Rectangle(0.0, (450.0, 55.0), 0.0),
				color: Color::from_hex("#000000", 255),
			},
			vec![BumperRow::new(ctx)])
			/*Shape{shape: ShapeType::RoundedRectangle(0.0, (55.0, 55.0), 20.0, 0.0), color: Color::from_hex("#0000FF", 255)},*/
	}
}

#[derive(Debug, Component)]
pub struct Canvas(CanvasLayout, Vec<Shape>, #[skip] bool);
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
				self.0.0.push(*my_position);
				let shape = match *ctx.state().get_or_default::<Brush>() {
					Brush::Ellipse => ShapeType::Ellipse(0.0, (20.0, 20.0), 0.0),
					Brush::Rectangle => ShapeType::Rectangle(0.0, (20.0, 20.0), 0.0),
					Brush::RoundedRectangle => ShapeType::RoundedRectangle(0.0, (20.0, 20.0), 20.0, 0.0),
				};
				self.1.push(Shape{shape, color: Color::from_hex("#FFD700", 255),});
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
			false
		)
    }
}
//display ellipse when mouse is clicked
#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page, #[skip] String);
impl OnEvent for FirstScreen {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(KeyboardEvent{key: my_key, state: KeyboardState::Pressed}) = event.downcast_ref::<KeyboardEvent>() {
			if let Some(key) = my_key.to_text() {
				let text = Text::new(ctx, self.2.as_str(), TextStyle::Primary, 16.0, Align::Left);
				match my_key {
					k => {
						self.1.content().find_at::<Hex>(2).unwrap().2 = text;
						self.2.push_str(key);
					}
				}
				if Key::Named(NamedKey::Enter) == *my_key {
					self.2.clear();
				}
				if Key::Named(NamedKey::Backspace) == *my_key {
                     let backspace = Text::new(ctx, self.2.as_str(), TextStyle::Primary, 16.0, Align::Left);
                     self.1.content().find_at::<Hex>(2).unwrap().2 = backspace;
                     self.2.pop();
                     self.2.pop();
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
		let children: Vec<Box<dyn Drawable>> = vec![Box::new(Bumper::new(ctx)), Box::new(Canvas::new(ctx)), Box::new(Hex::new(ctx)), Box::new(Bumper::new(ctx))];
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
