use pelican_ui::*;
use pelican_ui::drawable::{Component, Drawable, Shape, ShapeType, Color};
use pelican_ui::layout::{DefaultStack, SizeRequest, Area, Layout};
use pelican_ui::events::OnEvent;


#[derive(Component, Debug)]
pub struct FancyRect(DefaultStack, Shape, Shape);
impl OnEvent for FancyRect {}


#[derive(Clone)]
struct App;

impl Application for App {
    async fn new(_ctx: &mut Context) -> Box<dyn Drawable> {
        Box::new(

            FancyRect(, 
                Shape{
                    shape: ShapeType::Ellipse(10.0, (200.0, 200.0)),
                    color: Color(255, 255, 0, 255)
                },
                Shape{
                    shape: ShapeType::Rectangle(20.0, (200.0, 200.0)),
                    color: Color(255, 0, 0, 255)
                },
            )
        )
    }
}
impl Services for App {}

start!(App);
