use pelican_ui::*;
<<<<<<< HEAD
use pelican_ui::drawable::{Drawable, Shape, ShapeType, Color};
use pelican_ui::layout::DefaultStack;
use pelican_ui::events::Event;

#[derive(Debug)]
pub struct TwoObjectsStacked(DefaultStack, Shape, Box<dyn  Drawable>);
impl Event for TwoObjectsStacked {}
=======
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
>>>>>>> a9eaab514d4633bc4a61bc0a2f64c024afd0fb8e

#[derive(Clone)]
struct App;

impl Application for App {
    async fn new(_ctx: &mut Context) -> Box<dyn Drawable> {
<<<<<<< HEAD
        Box::new(Shape{
            shape: ShapeType::Rectangle(20.0, (200.0, 200.0)),
            color: Color(255, 0, 0, 255)
        });
		Box::new(Shape{
            shape: ShapeType::Ellipse(20.0, (200.0, 180.0)),
            color: Color(255, 0, 0, 255)
        })
=======
        Box::new(
            TwoObjectsStacked(DefaultStack,
                Shape{
                    shape: ShapeType::Rectangle(20.0, (200.0, 200.0)),
                    color: Color(0, 0, 255, 255)
                },
                Box::new(RowOfEllipseRect(
                    Row::new(30.0, Offset::Center, Size::Static(300.0), Padding::new(0.0)), 
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
>>>>>>> a9eaab514d4633bc4a61bc0a2f64c024afd0fb8e
    }
}
impl Services for App {}

start!(App);
//NOTE: default stack theory: it stacks elements
//NOTE: Shape is self explanatory
//NOTE: Box<dyn Drawable> dynamically resizes to the item at compiler time.drawable returns the shapes
//NOTE: trait Event detects any inputs from the user
//NOTE: App is the main structure essentially.
//NOTE: Application trait is the window we're drawing on
//NOTE: async new constructor is where you're creating the objects. it takes in ctx and returns a drawable item.
//NOTE: box is an object that initializes new shapes.
//NOTE: inside Shape {} we create the shape with its width and offset/ color with its hex codes
