use pelican_ui::*;
use pelican_ui::drawable::{Component, Drawable, Shape, ShapeType, Color};
use pelican_ui::layout::{DefaultStack, SizeRequest, Area, Layout};
use pelican_ui::events::OnEvent;

/// Represents a basic rectangle with no rounded corners or stroke.
#[derive(Debug)]
pub struct Rectangle(Shape);

impl Rectangle {
    /// Creates a new `Rectangle` with a specified color.
    ///
    /// # Parameters:
    /// - `color`: The color of the rectangle.
    ///
    /// # Returns:
    /// A new `Rectangle` component.
    pub fn new(color: Color) -> Self {
        Rectangle(Shape { shape: ShapeType::Rectangle(0.0, (0.0, 0.0)), color })
    }

    /// Returns a mutable reference to the shape of the `Rectangle`.
    pub fn shape(&mut self) -> &mut Shape { &mut self.0 }
}

impl OnEvent for Rectangle {}
impl Component for Rectangle {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![&mut self.0] }
    fn children(&self) -> Vec<&dyn Drawable> { vec![&self.0] }
    fn request_size(&self, _ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::fill()
    }
    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        if let ShapeType::Rectangle(_, s) = &mut self.0.shape {
            *s = size;
        }
        vec![Area { offset: (0.0, 0.0), size }]
    }
}
