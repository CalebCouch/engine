use pelican_ui::*;
use pelican_ui::drawable::{Component, Drawable, Shape, ShapeType, Color};
use pelican_ui::layout::{DefaultStack, SizeRequest, Area, Layout};
use pelican_ui::events::OnEvent;

mod layouts;
use layouts::{Row, Stack, Offset, Size, Padding};

mod elements;
use elements::Rectangle;

#[derive(Component, Debug)]
pub struct TwoObjectsStacked(DefaultStack, Shape, Box<dyn Drawable>);
impl OnEvent for TwoObjectsStacked {}

#[derive(Component, Debug)]
pub struct RowOfEllipseRect(Row, Shape, Shape);
impl OnEvent for RowOfEllipseRect {}

#[derive(Clone)]
struct App;

impl Application for App {
    async fn new(_ctx: &mut Context) -> Box<dyn Drawable> {
        Box::new(
            TwoObjectsStacked(DefaultStack,
                Shape{
                    shape: ShapeType::Rectangle(20.0, (200.0, 200.0)),
                    color: Color(0, 0, 255, 255)
                },
                Box::new(RowOfEllipseRect(
                    Row::new(30.0, Offset::Static(20.0), Size::Fit, Padding::new(0.0)), 
                    Shape{
                        shape: ShapeType::Ellipse(10.0, (200.0, 400.0)),
                        color: Color(255, 255, 0, 255)
                    },
                    Shape{
                        shape: ShapeType::Rectangle(20.0, (200.0, 200.0)),
                        color: Color(255, 0, 0, 255)
                    },
                ))
            )
        )
    }
}
impl Services for App {}

start!(App);
