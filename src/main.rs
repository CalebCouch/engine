use pelican_ui::*;
use pelican_ui::drawable::{Drawable, Shape, ShapeType, Color};

#[derive(Clone)]
struct App;

impl Application for App {
    async fn new(_ctx: &mut Context) -> Box<dyn Drawable> {
        Box::new(Shape{
            shape: ShapeType::Rectangle(20.0, (200.0, 200.0)),
            color: Color(255, 0, 0, 255)
        })
    }
}
impl Services for App {}

start!(App);
