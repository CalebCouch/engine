use pelican::*;
use pelican_ui::*;
use pelican_ui::components::interface::general::{Interface, Page, Content, Header, Bumper};
use pelican_ui::components::interface::navigation::{AppPage, PelicanError, RootInfo};
use pelican_ui::components::list_item::{ListItemGroup, ListItem, ListItemInfoLeft};
use pelican_ui::components::avatar::{AvatarContent, AvatarIconStyle};
use pelican_ui::components::Icon;
use pelican_ui::components::button::PrimaryButton;
use pelican_ui::layouts::{Stack, EitherOr, Offset, Size, Padding, Row};
use pelican_ui::drawable::{Drawable, Color, Shape, ShapeType};
use pelican_ui::events::{OnEvent, Event, KeyboardEvent, KeyboardState, TickEvent, Key, NamedKey};
use pelican_ui::theme::Theme;
use pelican_ui::plugin::PelicanUI;

use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

pub struct TestApp;
impl Plugin for TestApp {}
impl Application for TestApp {
	async fn new(ctx: &mut Context) -> impl Drawable {
		let home = RootInfo::icon("home", "my files", |ctx: &mut Context| {
			Box::new(FolderPage::new(ctx)) as Box<dyn AppPage>
		});
		Interface::new(ctx, (vec![home], None))
	}
	fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
		let theme = Theme::light(&mut ctx.assets, Color::from_hex("#00bf69ff", 255));
		vec![Box::new(PelicanUI::new(ctx, theme))]
	}
}
impl Services for TestApp {}

/*impl Application for TestApp {
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
		/*ctx.theme = Theme::new(
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
		);*/
		let first = FolderPage::new(ctx);
        let first = Interface::new(ctx, (vec![], None));
        let second = SecondPage::new(ctx);
        let second = Interface::new(ctx, (vec![], None));
        Box::new(CustomNavigation(Stack::default(), EitherOr::new(first, second)))
/*        Box::new(Shape{
            shape: ShapeType::Ellipse(0.0, (400.0, 400.0), 0.0),
            color: Color(0, 0, 255, 255)
        })*/
    }
}*/

#[derive(Debug, Component)]
pub struct CustomNavigation(Stack, EitherOr<Interface, Interface>);
impl OnEvent for CustomNavigation{
	fn on_event(&mut self, ctx: &mut Context, event: Box<(dyn pelican_ui::events::Event + 'static)>) -> Vec<Box<(dyn pelican_ui::events::Event + 'static)>> {
		if let Some(nav_event) = event.downcast_ref::<NavEvent>() {
			self.1.display_left(nav_event.0);
			vec![event]
		} else {
			vec![event]
		}
	}
}

#[derive(Debug, Clone)]
pub struct NavEvent(bool);
impl Event for NavEvent{
	fn pass(self: Box<Self>, ctx: &mut Context, children: &Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
		children.iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
	}
}

#[derive(Debug, Component)]
pub struct Files(Stack, ListItemGroup);
impl OnEvent for Files{}
impl Files {
	pub fn new(ctx: &mut Context) -> Self {
		let icon = Icon::new(ctx, "wallet", Some(Color::from_hex("#FF0000", 255)), 150.0);
		let item = ListItem::new(ctx, Some(AvatarContent::Icon("wallet".to_string(), AvatarIconStyle::Success)), ListItemInfoLeft::new("folder", "random file", None, None), None, None, None, |ctx: &mut Context| println!("it worked"));
		Files(
			Stack(Offset::Center, Offset::Center, Size::Fit    , Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
			ListItemGroup::new(vec![item]),
		)
	}
}

#[derive(Debug, Component)]
pub struct FolderPage(Stack, Page);
impl OnEvent for FolderPage {
	fn on_event(&mut self, ctx: &mut Context, event: Box<(dyn pelican_ui::events::Event + 'static)>) -> Vec<Box<dyn Event>> {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
		} else if let Some(KeyboardEvent{key, state: KeyboardState::Pressed}) = event.downcast_ref::<KeyboardEvent>() {
			if *key == Key::Named(NamedKey::Space) {
				let canvas = self.1.content().find_at::<Files>(0).unwrap();
				println!("this many files {:?}", canvas.1);
			}
		}
		vec![event]
	}
}

impl AppPage for FolderPage {
	fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<(dyn AppPage + 'static)>, PelicanError> {
 		todo!()
	}
}

impl FolderPage {
    pub fn new(ctx: &mut Context) -> Self {
		//make separate variable we push into then push that into children varaible?
		//maybe we can push another Vec<box> drawables into content?
		//maybe we can use find_at?
		//maybe make separate function? but how would we give argument to Bumper
		//maybe we can make another content that only stores File::new and we can use find_at on that?
		let mut children: Vec<Box<dyn Drawable>> = vec![Box::new(Files::new(ctx))];
		let file_button = PrimaryButton::new(ctx, "new file", move |ctx: &mut Context|{
			fn on_event(ctx: &mut Context, event: Box<(dyn pelican_ui::events::Event + 'static)>) -> Vec<Box<dyn Event>> {
				if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
					let item = ListItem::new(ctx, Some(AvatarContent::Icon("wallet".to_string(), AvatarIconStyle::Success)), ListItemInfoLeft::new("folder", "random file", None, None), None, None, None, |ctx: &mut Context| println!("it worked"));
					ctx.state();
					//can't use find_at because content is out of scope and we would lose ownership anyways
					ctx.state().get_mut_or_default::<Vec<Files>>().push(Files(
						Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
						ListItemGroup::new(vec![item]),
					));
					return vec![event];
				} else if let Some(KeyboardEvent{key, state: KeyboardState::Pressed}) = event.downcast_ref::<KeyboardEvent>() {}
				vec![event]
			}
		}, false);
		let folder_button = PrimaryButton::new(ctx, "new folder", |ctx: &mut Context|{
			println!("it worked");
		}, false);
		let buttons: Vec<Box<dyn Drawable>> = vec![Box::new(file_button), Box::new(folder_button)];
		let bumper = Bumper::new(ctx, buttons);
        let header = Header::home(ctx, "Folder Page", None);
        let content = Content::new(ctx, Offset::Center, children);
        FolderPage(Stack::default(), Page::new(header, content, Some(bumper)))
    }
}

#[derive(Debug, Component)]
pub struct FilePage(Stack, Page);
impl OnEvent for FilePage {}
/*impl AppPage for SecondPage {
	fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage + 'static>, PelicanError> { Err(self) }
}*/

impl FilePage {
	pub fn new(ctx: &mut Context) -> Self {
		//let color = ctx.theme.colors.text.heading;
        //let icon = Icon::new(ctx, "down", color, 128.0);
		let child: Vec<Box<dyn Drawable>> = vec![];
        let header = Header::home(ctx, "CONGRATULATIONS", None);
        let content = Content::new(ctx, Offset::Center, child);
		FilePage(Stack::default(), Page::new(header, content, None))
	}
}
