/*use std::collections::HashMap;
use crate::drawable::Image;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::any::Any;
use pelican_ui::*;
use runtime::{self, Service, ServiceList, ThreadContext, async_trait, Services};
use serde::de::Unexpected::Str;
use pelican::drawable::Component;

use pelican::events::{Event, OnEvent, TickEvent, KeyboardEvent, KeyboardState, NamedKey, Key};
use pelican::drawable::{Shape, Color, Drawable, ShapeType, Align};
use pelican::layout::{SizeRequest, Area, Layout};

use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::time::Duration;

use pelican_ui_std::*;
pub struct TestApp;
impl Plugins for TestApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {vec![]}
}
impl Services for TestApp {}
start!(TestApp);
fn main() {
    maverick_main();
}


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
/*        Box::new(Shape{
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
pub struct Letter(Stack, Shape, Text);
impl OnEvent for Letter {}
impl Letter {
    pub fn new(ctx: &mut Context) -> Self {
        Letter(
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
pub struct Emoji(Stack, Image);
impl OnEvent for Emoji{}
impl Emoji {
	fn new(ctx: &mut Context) -> Self {
		Emoji(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Icon::new(ctx, "emoji", Color::from_hex("#FF006E", 255), 120.0),
		)
	}
}

#[derive(Debug, Component)]
pub struct Message(Stack, Text);
impl OnEvent for Message{}
impl Message {
	pub fn new(ctx: &mut Context) -> Self {
		Message(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			Text::new(
				ctx,
				"U WIN",
				TextStyle::Primary,
				50.0,
				Align::Left,
			),
		)
	}
}

#[derive(Debug, Component)]
pub struct Win(Row, Emoji, Message, Emoji);
impl OnEvent for Win{}
impl Win {
	pub fn new(ctx: &mut Context) -> Self {
		Win(Row::center(10.0), Emoji::new(ctx), Message::new(ctx), Emoji::new(ctx))
	}
}

#[derive(Debug, Component)]
pub struct Word(Row, Vec<Letter>);
impl OnEvent for Word{}
impl Word {
    pub fn new(ctx: &mut Context) -> Self {
        Word(Row::center(10.0), vec![Letter::new(ctx), Letter::new(ctx), Letter::new(ctx), Letter::new(ctx), Letter::new(ctx)])
    }
}

//TODO: make it so enter isn't a valid key input.  make it add one to the index of Word and maybe Letter.    .push each key into a input variable or field.
#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page, #[skip] usize, #[skip] String, #[skip] String);
impl OnEvent for FirstScreen {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(KeyboardEvent{key: my_key, state: KeyboardState::Pressed}) = event.downcast_ref::<KeyboardEvent>() {
			if let Some(key) = my_key.to_text() {
				let text = Text::new(ctx, key, TextStyle::Primary, 16.0, Align::Left);
				if self.3.len() < 5 {
					match my_key {
						k =>  if let Some(key) = k.to_text() {
								self.1.content().find_at::<Word>(self.2).unwrap().1[self.3.len()].2 = text;
								self.3.push_str(key);
							println!("{}", self.3);
							println!("{}", self.4);
						}
					}
				}
				if Key::Named(NamedKey::Enter) == *my_key {
					let guessed_chars: Vec<char> = self.3.chars().collect();
					let random_word: Vec<char> = self.4.chars().collect();
					let mut frequency: HashMap<&char, u8> = HashMap::new();
					for c in &random_word {
						*frequency.entry(c).or_insert(0) += 1;
					}
//we want to default it to red assuming that everything is wrong. how do we do this tho?
					let mut change_colors = [Colors::Red; 5];
					for (slices, c) in guessed_chars.iter().enumerate() {
						if *c == random_word[slices] {
							change_colors[slices] = Colors::Green;
							if let Some(count) = frequency.get_mut(c) {
								*count -= 1;
							}
						}
					}

					for (slices, c) in guessed_chars.iter().enumerate() {
						if change_colors[slices] == Colors::Green {
							continue;
						}
						if let Some(count2) = frequency.get_mut(c) {
							if *count2 > 0 {
								change_colors[slices] = Colors::Yellow;
								*count2 -= 1;
							}
						}
					}

					for (slices, c) in guessed_chars.iter().enumerate() {
						let c_str = &c.to_string();
						let text2 = Text::new(ctx, c_str, TextStyle::Label(Color::from_hex("#00FF00", 255)), 16.0, Align::Left);
						let text3 = Text::new(ctx, c_str, TextStyle::Label(Color::from_hex("#FF0000", 255)), 16.0, Align::Left);
						let text4 = Text::new(ctx, c_str, TextStyle::Label(Color::from_hex("FFD700", 255)), 16.0, Align::Left);
						if change_colors[slices] == Colors::Green {
							self.1.content().find_at::<Word>(self.2).unwrap().1[slices].2 = text2;
						} if change_colors[slices] == Colors::Yellow {
							self.1.content().find_at::<Word>(self.2).unwrap().1[slices].2 = text4;
						} if change_colors[slices] == Colors::Red {
							self.1.content().find_at::<Word>(self.2).unwrap().1[slices].2 = text3;
						}
					}

					if change_colors == [Colors::Green; 5] {
						ctx.trigger_event(NavEvent(false));
					}
					self.2 += 1;
					self.3.clear();
				}
				if Key::Named(NamedKey::Backspace) == *my_key {
					let backspace = Text::new(ctx, " ", TextStyle::Primary, 16.0, Align::Left);
					self.1.content().find_at::<Word>(self.2).unwrap().1[self.3.len()-1].2 = backspace;
					self.3.pop();
					self.3.pop();
					println!("{}", self.3);
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
	fn get_word(path: &str) -> Vec<String> {
		let file = File::open(path).expect("failed to open");
		let reader = BufReader::new(file);
		reader.lines().filter_map(Result::ok).collect()
	}

    pub fn new(ctx: &mut Context) -> Self {
        let color = ctx.theme.colors.text.heading;
        let icon = Icon::new(ctx, "settings", color, 128.0);

		let color2 = ctx.theme.colors.shades.darken;
		let icon2 = Icon::new(ctx, "settings", color, 150.0);

        let font_size = ctx.theme.fonts.size;
        let text = Text::new(ctx, "Hello World!", TextStyle::Heading, font_size.h2, Align::Center);
        let subtext = ExpandableText::new(ctx, "First project loaded successfully.", TextStyle::Primary, font_size.md, Align::Center, None);
		let children: Vec<Box<dyn Drawable>> = vec![Box::new(Word::new(ctx)), Box::new(Word::new(ctx)), Box::new(Word::new(ctx)), Box::new(Word::new(ctx)), Box::new(Word::new(ctx)), Box::new(Word::new(ctx))];
        let content = Content::new(ctx, Offset::Center, children);

        let header = Header::home(ctx, "Wordle", None);
		let current = 0;
        FirstScreen(Stack::default(), Page::new(Some(header), content, None), current, String::new(), Self::get_word("words.txt").choose(&mut rand::thread_rng()).unwrap().to_string())
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Colors {
	Red,
	Green,
	Yellow,
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
		let child: Vec<Box<dyn Drawable>> = vec![Box::new(Win::new(ctx))];
        let header = Header::home(ctx, "CONGRATULATIONS", None);
        let content = Content::new(ctx, Offset::Center, child);
		SecondPage(Stack::default(), Page::new(Some(header), content, None))
	}
}*/
//use pelican_ui::{maverick_start, MaverickOS, PleicanEngine};
mod files;
pelican_ui::start!(files::TestApp);
fn main() {
    maverick_main();
}
