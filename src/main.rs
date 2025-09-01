use pelican_ui::*;
use runtime::{self, Service, ServiceList, ThreadContext, async_trait, Services};

use pelican_ui::drawable::Component;

use pelican_ui::events::{Event, OnEvent};
use pelican_ui::drawable::{Shape, Color, Drawable, ShapeType, Align};
use pelican_ui::layout::{SizeRequest, Area, Layout};

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
        let home = FirstScreen::new(ctx);
        let interface = Interface::new(ctx, Box::new(home), None, None);
        Box::new(interface)
/*        Box::new(Shape{
            shape: ShapeType::Ellipse(0.0, (400.0, 400.0), 0.0),
            color: Color(0, 0, 255, 255)
        })*/
    }
}

#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page);
impl OnEvent for FirstScreen {}

impl AppPage for FirstScreen {
        fn has_nav(&self) -> bool { false }
        fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { Err(self) }
}

impl FirstScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let color = ctx.theme.colors.text.heading;
        let icon = Icon::new(ctx, "pelican_ui", color, 128.0);

        let font_size = ctx.theme.fonts.size;
        let text = Text::new(ctx, "Hello World!", TextStyle::Heading, font_size.h2, Align::Center);
        let subtext = ExpandableText::new(ctx, "First project loaded successfully.", TextStyle::Primary, font_size.md, Align::Center, None);

        let content = Content::new(ctx, Offset::Center, vec![Box::new(icon), Box::new(text), Box::new(subtext)]);

        let header = Header::home(ctx, "My Screen", None);

        FirstScreen(Stack::default(), Page::new(Some(header), content, None))
    }
}
