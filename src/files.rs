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
        let root = ctx.state().get_mut_or_default::<Folder>();
        *root = Folder{
            name: "root".to_string(),
            files: BTreeMap::new(),
            folders: BTreeMap::from([
                ("pictures".to_string(), Folder{
                    name: "pictures".to_string(),
                    files: BTreeMap::new(),
                    folders: BTreeMap::from([
                        ("kittens".to_string(), Folder{
                            name: "kittens".to_string(),
                            files: BTreeMap::from([
                                ("blue_kitten".to_string(), File{
                                    name: "blue_kitten".to_string(),
                                    body: "Encoded Image Bytes".to_string()
                                })
                            ]),
                            folders: BTreeMap::new()
                        })
                    ])
                })
            ])
        };


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

///Describes a File structure with name and a body(this is text we can change like a note later)
#[derive(Debug, Clone)]
pub struct File{
    name: String,
    body: String
}

///Describes a Folder structure with a name and a list of child files and folders
#[derive(Debug, Default, Clone)]
pub struct Folder {
    name: String,
    files: BTreeMap<String, File>,
    folders: BTreeMap<String, Self>,
}

impl Folder {
	fn get_folder(self, ctx: &mut Context, path: String) -> &Folder {
		path.split('/');
		println!("{:?}", path);
		if let Some(file) = self.folders.get(&path) {
			file
		} else {
		}
		//return the last element of the collection we created?
	}
    //TODO: impl a function like get_folder that accepts a path, splits the path by "/" and then
    //tries to get the sub folder matching the path
    //
    //My root folder example would be root.get("pictures/kittens") should return me the folder with
    //the blue_kitten file
}

//  ctx.get_or_default::<Folder>().folders.get("pictures").folders.get("kittens")
//  /pictures/kittens

#[derive(Debug, Component)]
pub struct Files(Stack, ListItemGroup);
impl OnEvent for Files{
	fn on_event(&mut self, ctx: &mut Context, event: Box<(dyn pelican_ui::events::Event + 'static)>) -> Vec<Box<dyn Event>> {
		if let Some(tick_event) = event.downcast_ref::<TickEvent>() {
            //Every tick re-create the Files structure to refresh the UI
            *self = Files::new(ctx, String::new());
        }
        vec![event]
    }
}
impl Files {
	pub fn new(ctx: &mut Context, path: String) -> Self {
        //Get the folder out of state
        let root = ctx.state().get_or_default::<Folder>().clone().folders.remove("pictures").unwrap().folders.remove("kittens").unwrap();
        //Loop through the files and create a list item for each one
		let mut files = root.files.into_iter().map(|(name, file)| {
            let icon = Icon::new(ctx, "wallet", Some(Color::from_hex("#FF0000", 255)), 150.0);
            ListItem::new(
                ctx,
                Some(AvatarContent::Icon("wallet".to_string(), AvatarIconStyle::Success)),
                ListItemInfoLeft::new("files", &file.name, None, None),
                None, None, None, |ctx: &mut Context| println!("it worked")
            )
        }).collect::<Vec<_>>();

        //Loop through the folders and create a list item for each one
        let folders = root.folders.into_iter().map(|(name, folder)| {
            let icon = Icon::new(ctx, "wallet", Some(Color::from_hex("#FF0000", 255)), 150.0);
            ListItem::new(
                ctx,
                Some(AvatarContent::Icon("wallet".to_string(), AvatarIconStyle::Success)),
                ListItemInfoLeft::new("folder", &folder.name, None, None),
                None, None, None, |ctx: &mut Context| println!("it worked")
            )
        }).collect::<Vec<_>>();

        files.extend(folders);

		Files(
			Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
                               //Concat the file and folder list items and display
			ListItemGroup::new(files),
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
				println!("{:?}", self.1.content().find_at::<Files>(0).unwrap().1);
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

//TODO:
//MAYBE DONE: remove all elements from listitemgroup
//DONE: read the files from ctx.state()
//then, create a list from that and push it to the listitemgroup
impl FolderPage {
    pub fn new(ctx: &mut Context) -> Self {
		//let files = ctx.state().get_mut_or_default::<Vec<Files>>();
		let mut children: Vec<Box<dyn Drawable>> = vec![Box::new(Files::new(ctx, String::new()))];
		//children.push(Box::new(files[0]));
		let file_button = PrimaryButton::new(ctx, "new file", move |ctx: &mut Context|{
			let item = ListItem::new(
				ctx,
				Some(AvatarContent::Icon("wallet".to_string(),
				AvatarIconStyle::Success)),
				ListItemInfoLeft::new("folder", "random file", None, None),
				None, None, None,
				|ctx: &mut Context| println!("it worked")
			);
			let file = ctx.state().get_mut_or_default::<Vec<Files>>();
			file.push(Files(
				Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0.0, 0.0, 0.0, 0.0)),
				ListItemGroup::new(vec![item]),
			));
			//println!("{:?}", file);
		}, false);
		let folder_button = PrimaryButton::new(ctx, "new folder", |ctx: &mut Context|{
			println!("it worked");
		}, false);

		let move_button = PrimaryButton::new(ctx, "new folder", |ctx: &mut Context|{
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

#[derive(Debug, Component)]
pub struct MovePage(Stack, Page);
impl OnEvent for MovePage {}
/*impl AppPage for SecondPage {
	fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage + 'static>, PelicanError> { Err(self) }
}*/

impl MovePage {
	pub fn new(ctx: &mut Context) -> Self {
		//let color = ctx.theme.colors.text.heading;
        //let icon = Icon::new(ctx, "down", color, 128.0);
		let child: Vec<Box<dyn Drawable>> = vec![];
        let header = Header::home(ctx, "CONGRATULATIONS", None);
        let content = Content::new(ctx, Offset::Center, child);
		MovePage(Stack::default(), Page::new(header, content, None))
	}
}

enum FileFolder {
	
}
