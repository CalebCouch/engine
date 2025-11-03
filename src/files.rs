use pelican_ui::*;
use runtime::{self, Service, ServiceList, ThreadContext, async_trait, Services};
use pelican_ui::drawable::{Component, Image};
use pelican_ui::events::{Event, OnEvent, TickEvent, KeyboardEvent, KeyboardState, NamedKey, Key};
use pelican_ui::drawable::{Shape, Color, Drawable, ShapeType, Align};
use pelican_ui::layout::{SizeRequest, Area, Layout};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

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
		let first = FolderPage::new(ctx);
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
pub struct ListItem(Stack, ListItemGroup);
impl OnEvent for ListItem{}
impl ListItem {
	pub fn new(ctx: &mut Context) -> Self {
		let icon = Icon::new(ctx, "wallet", color, 150.0);
		let item = ListItem::new(ctx, Some(AvatarContent::Icon("wallet", AvatarIconStyle::Success), ListItemInfoLeft::new("folder", "random file", None, None), 
		ListItem(
			Stack(Offset::Center, Offset::Center, Size::Fit    , Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			ListItemGroup::new(item: Vec<ListItem>)
		)
	}
}

#[derive(Debug, Component)]
pub struct FolderPage(Stack, Page);
impl OnEvent for FolderPage {
	fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {

		} else if let Some(KeyboardEvent{key, state: KeyboardState::Pressed}) = event.downcast_ref::<KeyboardEvent>() {

		}
		true
	}
}

impl AppPage for FolderPage {
	fn has_nav(&self) -> bool { true }
	fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Err(self) }
}

impl FolderPage {
    pub fn new(ctx: &mut Context) -> Self {
		let children: Vec<Box<dyn Drawable>> = vec![];
        let content = Content::new(ctx, Offset::Center, children);

        let header = Header::home(ctx, "Folder Page", None);
		let current = 0;
        FolderPage(Stack::default(), Page::new(Some(header), content, None))
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
		let child: Vec<Box<dyn Drawable>> = vec![];
        let header = Header::home(ctx, "CONGRATULATIONS", None);
        let content = Content::new(ctx, Offset::Center, child);
		SecondPage(Stack::default(), Page::new(Some(header), content, None))
	}
}
