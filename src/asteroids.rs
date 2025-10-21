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
pub struct Canvas(CanvasLayout, Vec<Asteroid>, Ship);
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
			CanvasLayout(vec![(300.0, 300.0), (20.0, 20.0), /*(140.0, 100.0),*/ (200.0, 20.0), (260.0, 200.0), /*(320.0, 120.0),*/ (20.0, 270.0)]),
			vec![Asteroid::big(ctx), Asteroid::medium(ctx), /*Asteroid::small(ctx),*/ Asteroid::small(ctx), /*Asteroid::medium(ctx),*/ Asteroid::medium(ctx)],
			Ship::new(ctx),
		)
    }
}
//display ellipse when mouse is clicked
#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page, #[skip] (f32, f32), #[skip] u8);
impl OnEvent for FirstScreen {
fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(MouseEvent{position: Some(my_position), state: my_state}) = event.downcast_ref::<MouseEvent>() {
			//TODO:
			//COMPLETED: so maybe we have the asteroids loop back through if they reach a certain number. we'll try this for now and add a better system later since we'll be moving with our ship.
			//make code cleaner and less hardcoded
			//add asteroid collision and splitting into smaller asteroids
			//add ship movement
			//make ship shoot
			self.3 = :
			self.2 = (5.0, 5.0);
			let mut a = self.1.content().find_at::<Canvas>(0).unwrap().0.0[0];
			let mut b = self.1.content().find_at::<Canvas>(0).unwrap().0.0[1];
			let mut c = self.1.content().find_at::<Canvas>(0).unwrap().0.0[2];
			let mut d = self.1.content().find_at::<Canvas>(0).unwrap().0.0[3];
			let asteroid0 = (a.0 - self.2.0, a.1 - self.2.1);
			let asteroid1 = (b.0 + self.2.0, b.1 + self.2.1);
			let asteroid2 = (c.0 + self.2.0, c.1 + self.2.1);
			let asteroid3 = (d.0 + self.2.0, d.1 + self.2.1);
			self.1.content().find_at::<Canvas>(0).unwrap().0.0[0] = asteroid0;
			self.1.content().find_at::<Canvas>(0).unwrap().0.0[1] = asteroid1;
			self.1.content().find_at::<Canvas>(0).unwrap().0.0[2] = asteroid2;
			self.1.content().find_at::<Canvas>(0).unwrap().0.0[3] = asteroid3;
			if self.1.content().find_at::<Canvas>(0).unwrap().0.0[1] == (1000.0, 1000.0) {
				self.1.content().find_at::<Canvas>(0).unwrap().0.0[1] = (-100.0, -100.0);
			}
			//we need a way to index all the elements of a vec. we can't use a for loop because it breaks the program. i might have a really cheesy way of doing this but we'll see
			//we could just make a variable that equals a vec that we could store each individual slice in, but that is still kind of inefficient
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
		let children: Vec<Box<dyn Drawable>> = vec![Box::new(Canvas::new(ctx)),];
		let content = Content::new(ctx, Offset::Center, children);
		let header = Header::home(ctx, "Canvas", None);
		FirstScreen(Stack::default(), Page::new(Some(header), content, None), (0.0, 0.0), 0)
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


