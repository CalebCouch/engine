use pelican_ui::*;
use std::sync::{Arc, Mutex};

use pelican_ui::drawable::{Component, Drawable, Shape, ShapeType, Color};
use pelican_ui::layout::{DefaultStack, SizeRequest, Area, Layout};
use pelican_ui::events::OnEvent;


/// Represents how a layout should offset itself
#[derive(Clone, Copy, Default, Debug)]
pub enum Offset {
    /// Aligns to the start (no offset).
    #[default]
    Start,
    /// Aligns to the center.
    Center,
    /// Aligns to the end (maximum offset).
    End,
    /// Applies a fixed custom offset.
    Static(f32)
}

impl Offset {
    /// Returns the computed offset based on the given maximum size and item size.
    pub fn get(&self, max_size: f32, item_size: f32) -> f32 {
        match self {
            Self::Start => 0.0,
            Self::Center => (max_size - item_size) / 2.0,
            Self::End => max_size - item_size,
            Self::Static(offset) => *offset,
        }
    }

    /// Returns the static offset value if available.
    /// For `Center` and `End`, where the offset depends on context, returns `None`.
    pub fn size(&self) -> Option<f32> {
        match self {
            Self::Start => Some(0.0),
            Self::Center | Self::End => None,
            Self::Static(offset) => Some(*offset),
        }
    }
}

type CustomFunc = dyn Fn(Vec<(f32, f32)>) -> (f32, f32);
type FitFunc = fn(Vec<(f32, f32)>) -> (f32, f32);

/// Determines how a layout calculates and sets its own size.
#[derive(Default)]
pub enum Size {
    /// Size will match the largest child. 
    /// If any child grows, this layout grows accordingly.
    #[default]
    Fit,
    /// Size will fill the available space, constrained between a minimum and maximum.
    /// The first value is the minimum size, and the second is the maximum size.
    Fill(f32, f32),
    /// A fixed, unchanging size.
    Static(f32),
    /// A custom size calculated by a user-provided function.
    /// Example usage:
    /// `Size::Custom(Box::new(move |children| (children[1].0, children[1].1)))`
    /// This example uses the first child’s minimum and maximum sizes.
    Custom(Box<CustomFunc>),
}

impl Size {
    /// Creates a `Size::Fill` with a default range from `0.0` to `f32::MAX`.
    /// This means the layout will stretch as much as it can while never being smaller than zero.
    pub fn fill() -> Self {Size::Fill(0.0, f32::MAX)}
    /// Creates a custom size variant using a user-defined function.
    ///
    /// The function receives a vector of `(min, max)` size pairs from child layouts
    /// and returns a new `(min, max)` size for the layout itself.
    ///
    /// # Example
    /// ```
    /// Size::custom(|sizes| sizes.iter().fold((0.0, 0.0), |acc, (min, max)| (
    ///     acc.0.max(*min),
    ///     acc.1.max(*max),
    /// )))
    /// ```
    pub fn custom(func: impl Fn(Vec<(f32, f32)>) -> (f32, f32) + 'static) -> Self {
        Size::Custom(Box::new(func))
    }

    fn get(&self, items: Vec<(f32, f32)>, fit: FitFunc) -> (f32, f32) {
        match self {
            Size::Fit => fit(items),
            Size::Fill(min, max) => (*min, *max),
            Size::Static(s) => (*s, *s),
            Size::Custom(f) => f(items)
        }
    }

    fn max(items: Vec<(f32, f32)>) -> (f32, f32) {
        items.into_iter().reduce(|s, i| (s.0.max(i.0), s.1.max(i.1))).unwrap_or_default()
    }

    fn add(items: Vec<(f32, f32)>) -> (f32, f32) {
        items.into_iter().reduce(|s, i| (s.0+i.0, s.1+i.1)).unwrap_or_default()
    }
}

impl std::fmt::Debug for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Fit => write!(f, "Size::Fit"),
            Size::Fill(min, max) => write!(f, "Size::Fill(min: {}, max: {})", min, max),
            Size::Static(val) => write!(f, "Size::Static({})", val),
            Size::Custom(_) => write!(f, "Size::Custom(<function>)"),
        }
    }
}

/// Represents the padding around a layout element.
/// The four values represent the padding for the top, right, bottom, and left sides, respectively.
#[derive(Clone, Debug, Default)]
pub struct Padding(pub f32, pub f32, pub f32, pub f32);

impl Padding {
    /// Creates a new `Padding` with the same value for all four sides.
    ///
    /// # Arguments
    ///
    /// * `p` - A single value that will be applied to all sides (top, right, bottom, left).
    ///
    /// # Example
    ///
    /// ```
    /// let padding = Padding::new(10.0);
    /// ```
    pub fn new(p: f32) -> Self {Padding(p, p, p, p)}

    fn adjust_size(&self, size: (f32, f32)) -> (f32, f32) {
        let wp = self.0+self.2;
        let hp = self.1+self.3;
        (size.0-wp, size.1-hp)
    }

    fn adjust_offset(&self, offset: (f32, f32)) -> (f32, f32) {
        (offset.0+self.0, offset.1+self.1)
    }

    fn adjust_request(&self, request: SizeRequest) -> SizeRequest {
        let wp = self.0+self.2;
        let hp = self.1+self.3;
        request.add(wp, hp)
    }
}

/// A utility for uniformly expanding items within a layout, considering both their minimum and maximum sizes.
/// The `get` function adjusts the size of each item in the input list to fit within a given maximum size,
/// ensuring that they expand evenly as much as possible while respecting their constraints.
pub struct UniformExpand;

impl UniformExpand {
    /// Expands the items in the layout to fit within a maximum size, distributing the free space evenly.
    ///
    /// This function adjusts the minimum size of each item up to its maximum size, taking into account any
    /// spacing between the items. The function tries to expand the items as much as possible, respecting
    /// their individual minimum and maximum size constraints. Any remaining free space after the initial
    /// expansion is distributed among the items that can still expand.
    ///
    /// # Arguments
    ///
    /// * `sizes` - A vector of tuples representing the minimum and maximum sizes of each item.
    /// * `max_size` - The maximum available space that the items should fit within.
    /// * `spacing` - The spacing between items. The total spacing will be calculated as `(number of items - 1) * spacing`.
    ///
    /// # Returns
    ///
    /// Returns a vector of the expanded sizes, with each item adjusted based on the available space and its constraints.
    pub fn get(sizes: Vec<(f32, f32)>, max_size: f32, spacing: f32) -> Vec<f32> {
        // Calculate the total spacing and the minimum size required
        let spacing = (sizes.len() - 1) as f32 * spacing;
        let min_size = sizes.iter().fold(0.0, |s, i| s + i.0) + spacing;

        let mut sizes = sizes.into_iter().map(|s| (s.0, s.1)).collect::<Vec<_>>();

        let mut free_space = (max_size - min_size).max(0.0);
        while free_space > 0.0 {
            let (min_exp, count, next) = sizes.iter().fold((None, 0.0, free_space), |(mut me, mut c, mut ne), size| {
                let min = size.0;
                let max = size.1;
                if min < max { // Item can expand
                    match me {
                        Some(w) if w < min => {
                            ne = ne.min(min - w); // Next size could be the min size of the next expandable block
                        },
                        Some(w) if w == min => {
                            ne = ne.min(max - min); // Next size could be the max size of one of the smallest items
                            c += 1.0;
                        },
                        Some(w) if w > min => {
                            ne = ne.min(max - min).min(w - min); // Next size could be the max size of one of the smallest items
                            me = Some(min);
                            c = 1.0;
                        },
                        _ => {
                            ne = ne.min(max - min); // Next size could be the max size of one of the smallest items
                            me = Some(min);
                            c = 1.0;
                        }
                    }
                }
                (me, c, ne)
            });

            if min_exp.is_none() { break; }
            let min_exp = min_exp.unwrap();

            let expand = (next * count).min(free_space); // Next size could be the rest of the free space
            free_space -= expand;
            let expand = expand / count;

            sizes.iter_mut().for_each(|size| {
                if size.0 < size.1 && size.0 == min_exp {
                    size.0 += expand;
                }
            });
        }

        sizes.into_iter().map(|s| s.0).collect()
    }
}

/// Represents a row layout in a UI system.
///
/// This structure defines a row with a specified spacing, alignment (`Offset`), size (`Size`), 
/// and padding (`Padding`). It is used to arrange elements horizontally in a row, with various 
/// configuration options for how the row behaves and how the elements are laid out within it.
#[derive(Debug)]
pub struct Row(f32, Offset, Size, Padding);

impl Row {
    /// Creates a new row with the specified spacing, offset, size, and padding.
    ///
    /// This method initializes a [`Row`] with the provided spacing, alignment ([`Offset`]), size behavior ([`Size`]), 
    /// and padding ([`Padding`]). The scroll value is initialized to `0.0`.
    ///
    /// # Parameters
    /// - `spacing`: The space between items in the column.
    /// - `offset`: The alignment of the items in the column.
    /// - `size`: The size behavior of the column.
    /// - `padding`: The padding around the column content.
    ///
    /// # Returns
    /// A [`Row`] with the specified `spacing`, `offset`, `size`, `padding`, and `scroll` initialized to `0.0`.
    pub fn new(spacing: f32, offset: Offset, size: Size, padding: Padding) -> Self {
        Row(spacing, offset, size, padding)
    }
    /// Creates a centered row with the specified spacing.
    ///
    /// This method will create a row with items aligned to the center and a dynamic size (`Size::Fit`), 
    /// with no padding by default.
    ///
    /// # Parameters
    /// - `spacing`: The spacing between the elements in the row.
    ///
    /// # Returns
    /// A `Row` with the specified spacing, centered alignment, fit size, and default padding.
    pub fn center(spacing: f32) -> Self {
        Row::new(spacing, Offset::Center, Size::Fit, Padding::default())
    }
}


impl Layout for Row {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let spacing = self.0*(widths.len()-1) as f32;
        let width = Size::add(widths);
        let height = self.2.get(heights, Size::max);
        self.3.adjust_request(SizeRequest::new(width.0, height.0, width.1, height.1).add_width(spacing))
    }

    fn build(&self, _ctx: &mut Context, row_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let row_size = self.3.adjust_size(row_size);

        let widths = UniformExpand::get(children.iter().map(|i| (i.min_width(), i.max_width())).collect::<Vec<_>>(), row_size.0, self.0);

        let mut offset = 0.0;
        children.into_iter().zip(widths).map(|(i, width)| {
            let size = i.get((width, row_size.1));
            let off = self.3.adjust_offset((offset, self.1.get(row_size.1, size.1)));
            offset += size.0+self.0;
            Area{offset: off, size}
        }).collect()
    }
}

/// Represents a column layout in a UI system.
///
/// The `Column` structure defines a column with a specified spacing between items, alignment (`Offset`), 
/// size behavior (`Size`), padding (`Padding`), and an optional scrollable parameter. It is used to 
/// arrange elements vertically in a column, with various configuration options for how the column behaves 
/// and how the elements are laid out within it.
#[derive(Debug)]
pub struct Column(f32, Offset, Size, Padding);

impl Column {
    /// Creates a new column with the specified spacing, offset, size, and padding.
    ///
    /// This method initializes a `Column` with the provided spacing, alignment (`Offset`), size behavior (`Size`), 
    /// and padding (`Padding`). The scroll value is initialized to `0.0`.
    ///
    /// # Parameters
    /// - `spacing`: The space between items in the column.
    /// - `offset`: The alignment of the items in the column.
    /// - `size`: The size behavior of the column.
    /// - `padding`: The padding around the column content.
    ///
    /// # Returns
    /// A `Column` with the specified `spacing`, `offset`, `size`, `padding`, and `scroll` initialized to `0.0`.
    pub fn new(spacing: f32, offset: Offset, size: Size, padding: Padding) -> Self {
        Column(spacing, offset, size, padding)
    }

    /// Creates a centered column with the specified spacing.
    ///
    /// This method creates a column with the following default behaviors:
    /// - Items are aligned to the center (`Offset::Center`).
    /// - The size of the column is dynamic and adjusts to fit the content (`Size::Fit`).
    /// - The padding is set to default values (no extra space).
    /// - The scroll value is initialized to `0.0`.
    ///
    /// # Parameters
    /// - `spacing`: The space between elements in the column.
    ///
    /// # Returns
    /// A `Column` with the specified `spacing`, centered alignment, dynamic size, default padding, and scroll value of `0.0`.
    pub fn center(spacing: f32) -> Self {
        Column(spacing, Offset::Center, Size::Fit, Padding::default())
    }
}

impl Layout for Column {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let spacing = self.0*(heights.len()-1) as f32;
        let width = self.2.get(widths, Size::max);
        let height = Size::add(heights);
        self.3.adjust_request(SizeRequest::new(width.0, height.0, width.1, height.1).add_height(spacing))
    }

    fn build(&self, _ctx: &mut Context, col_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let col_size = self.3.adjust_size(col_size);

        let heights = UniformExpand::get(children.iter().map(|i| (i.min_height(), i.max_height())).collect::<Vec<_>>(), col_size.1, self.0);

        let mut offset = 0.0;
        children.into_iter().zip(heights).map(|(i, height)| {
            let size = i.get((col_size.0, height));
            let off = self.3.adjust_offset((self.1.get(col_size.0, size.0), offset));
            offset += size.1+self.0;
            Area{offset: off, size}
        }).collect()
    }
}

/// Represents a stack layout, where items are arranged vertically or horizontally, based on the configuration of the offsets and sizes.
///
/// The `Stack` structure defines how a group of items should be arranged in a stacked layout. It supports 
/// alignment and size configuration both for the main axis and cross axis, as well as padding around the layout.
///
/// # Fields
/// - `main_offset`: Defines the alignment of items along the main axis (e.g., vertical or horizontal axis).
/// - `cross_offset`: Defines the alignment of items along the cross axis (perpendicular to the main axis).
/// - `main_size`: Specifies how the stack should size itself along the main axis (e.g., fit to content, fixed size, fill within boundaries).
/// - `cross_size`: Specifies how the stack should size itself along the cross axis (similar to `main_size`).
/// - `padding`: The padding around the stack layout (top, bottom, left, right).
///
/// # Behavior
/// The stack layout allows flexibility in arranging items by offering different alignment and sizing options 
/// for both the main and cross axes, along with padding. It is particularly useful when you want to align items 
/// in a vertical or horizontal stack while maintaining control over their size and position.
///
/// # Methods
/// - `center`: Creates a stack layout where both axes (main and cross) are centered, with dynamic sizing to fit content.
/// - `fill`: Creates a stack layout where both axes are filled with available space, with optional resizing for the contents.
#[derive(Debug, Default)]
pub struct Stack(pub Offset, pub Offset, pub Size, pub Size, pub Padding);

impl Stack {
    /// Creates a centered stack layout.
    ///
    /// This method creates a stack with the following default behaviors:
    /// - Items are aligned to the center along both the main axis and cross axis (`Offset::Center`).
    /// - The size of the stack adjusts to fit the content along both axes (`Size::Fit`).
    /// - Padding is set to default values (no extra space).
    ///
    /// # Returns
    /// A `Stack` with centered alignment and dynamic sizing.
    pub fn center() -> Self {
        Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding::default())
    }

    /// Creates a stack layout that fills available space.
    ///
    /// This method creates a stack with the following behaviors:
    /// - Items are aligned to the center along both the main axis and cross axis (`Offset::Center`).
    /// - The size of the stack fills available space along both axes (`Size::Fill`).
    /// - Padding is set to default values (no extra space).
    ///
    /// # Returns
    /// A `Stack` with filled sizing and centered alignment.
    pub fn fill() -> Self {
        Stack(Offset::Center, Offset::Center, Size::fill(), Size::fill(), Padding::default())
    }
}

impl Layout for Stack {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|r|
            ((r.min_width(), r.max_width()), (r.min_height(), r.max_height()))
        ).unzip();
        let width = self.2.get(widths, Size::max);
        let height = self.3.get(heights, Size::max);
        self.4.adjust_request(SizeRequest::new(width.0, height.0, width.1, height.1))
    }

    fn build(&self, _ctx: &mut Context, stack_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let stack_size = self.4.adjust_size(stack_size);
        children.into_iter().map(|i| {
            let size = i.get(stack_size);
            let offset = (self.0.get(stack_size.0, size.0), self.1.get(stack_size.1, size.1));
            Area{offset: self.4.adjust_offset(offset), size}
        }).collect()
    }
}

/// Represents a wrap layout, where items are wrapped into rows or columns, allowing for flexible positioning and spacing.
///
/// The `Wrap` structure arranges items within a container and automatically wraps them when the available space 
/// is exhausted. It provides control over the spacing between items and the alignment of items along both axes. 
/// This is useful for creating responsive layouts where items adjust dynamically to available space.
///
/// # Fields
/// - `width_spacing`: The space between items in the horizontal direction (width) when they are arranged in rows.
/// - `height_spacing`: The space between items in the vertical direction (height) when they are arranged in columns.
/// - `vertical_offset`: Defines how items are aligned vertically within each row or column (i.e., main axis alignment).
/// - `horizontal_offset`: Defines how items are aligned horizontally within each row or column (i.e., cross axis alignment).
/// - `padding`: The padding around the entire wrap layout, providing space between the layout's edges and its children.
///
/// # Behavior
/// The wrap layout allows for items to "wrap" into a new row or column when the available space is insufficient. 
/// The `width_spacing` and `height_spacing` control the amount of space between the items along the horizontal and 
/// vertical axes, respectively. The `vertical_offset` and `horizontal_offset` define how the items are positioned 
/// within each row or column. Padding can also be applied around the entire layout to provide spacing from its edges.
///
/// # Methods
/// - `center`: Creates a wrap layout where both axes (vertical and horizontal) are centered, with user-defined spacing.
/// 
/// # Example
/// ```rust
/// let wrap_layout = Wrap::center(10.0, 5.0);
/// ```
#[derive(Debug)]
pub struct Wrap(pub f32, pub f32, pub Offset, pub Offset, pub Padding);

impl Wrap {
    /// Creates a wrap layout.
    ///
    /// This method creates a wrap layout with the following behaviors:
    /// - Horizontal spacing between items is specified by `w_spacing`.
    /// - Vertical spacing between items is specified by `h_spacing`.
    /// - Both vertical and horizontal alignment is centered (`Offset::Center`).
    /// - Padding is set to default values (no extra space).
    ///
    /// # Parameters
    /// - `w_spacing`: The space between items along the horizontal axis (width).
    /// - `h_spacing`: The space between items along the vertical axis (height).
    ///
    /// # Returns
    /// A `Wrap` layout with specified spacing and centered alignment.
    pub fn new(w_spacing: f32, h_spacing: f32) -> Self {
        Wrap(w_spacing, h_spacing, Offset::Center, Offset::Center, Padding::default())
    }
}

impl Layout for Wrap {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let ((min_w, max_w), (min_h, max_h)): ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = children.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();

        let w_spacing = self.0*(min_w.len()-1) as f32;
        let h_spacing = self.1*(min_h.len()-1) as f32;

        let min_width = min_w.into_iter().reduce(|s, i| s.max(i)).unwrap_or_default();
        let max_width = max_w.into_iter().reduce(|s, i| s+i).unwrap_or_default();

        let min_height = min_h.into_iter().reduce(|s, i| s.max(i)).unwrap_or_default();
        let max_height = max_h.into_iter().sum();

        // println!("min: {:?} max: {:?}", min_height, max_height);

        self.4.adjust_request(SizeRequest::new(min_width, min_height, max_width, max_height).add(w_spacing, h_spacing))
    }

    fn build(&self, _ctx: &mut Context, maximum_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let mut taken_width = self.4.1;
        let mut height_offset = self.4.2;
        let mut items: Vec<SizeRequest> = Vec::new();
        children.iter().map(|child| {
            if (taken_width + child.min_width()) > maximum_size.0 {
                let heights: Vec<_> = items.iter().map(|c| c.min_height()).collect();
                height_offset += heights.into_iter().reduce(|s, i| s.max(i)).unwrap_or_default() + self.1;
                taken_width = self.4.1;
            };

            let area = Area {offset: (taken_width, height_offset), size: (child.min_width(), child.min_height())};
            // println!("area: {:?}", area);
            taken_width += child.min_width() + self.0; 
            items.push(*child);
            area
        }).collect()
    }
}

/// documentation tbd
#[derive(Debug, Default)]
pub struct Scroll(Offset, Offset, Size, Size, Padding, Arc<Mutex<f32>>); // allow for horizontal scroll (FUTURE)

impl Scroll {
    pub fn new(offset_x: Offset, offset_y: Offset, size_x: Size, size_y: Size, padding: Padding) -> Self {
        Scroll(offset_x, offset_y, size_x, size_y, padding, Arc::new(Mutex::new(0.0)))
    }

    pub fn center() -> Self {
        Scroll(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding::default(), Arc::new(Mutex::new(0.0)))
    }

    pub fn adjust_scroll(&mut self, val: f32) {
        *self.5.lock().unwrap() += val;
    }

    pub fn set_scroll(&mut self, val: f32) {
        *self.5.lock().unwrap() = val;
    }

    pub fn offset(&mut self) -> &mut Offset { &mut self.1 }
}

impl Layout for Scroll {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|r|
            ((r.min_width(), r.max_width()), (r.min_height(), r.max_height()))
        ).unzip();
        let width = self.2.get(widths, Size::max);
        let height = self.3.get(heights, Size::max);
        self.4.adjust_request(SizeRequest::new(width.0, height.0, width.1, height.1))
    }

    fn build(&self, _ctx: &mut Context, scroll_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let scroll_size = self.4.adjust_size(scroll_size);
        let children_height: f32 = children.iter().map(|i| i.min_height()).sum();
        let scroll_val = self.5.lock().unwrap().min(children_height - scroll_size.1).max(0.0);
        *self.5.lock().unwrap() = scroll_val;
        children.into_iter().map(|i| {
            let size = i.get(scroll_size);
            let offset = (self.0.get(scroll_size.0, size.0), self.1.get(scroll_size.1, size.1)-scroll_val);
            Area{offset: self.4.adjust_offset(offset), size}
        }).collect()
    }
}

        //1. get max size of all children added up
        //2. subtract size of scroll element height from above
        //3. This is max scroll
        //4. scroll_val min max_scroll
        //5. set scroll_val to above
        //6. add scroll_val to height offset of all objects


/// A bin layout that encapsulates a single layout and a single drawable component.
///
/// The `Bin` layout is a container that holds one layout and one drawable component. The layout determines how the
/// drawable component is arranged and positioned within the container. This is useful when you want to create a simple
/// layout system that contains a single component, but still allows the component to be controlled via the layout system.
///
/// # Type Parameters
/// - `L`: The layout type that determines how the contained component is arranged.
/// - `D`: The drawable component that will be arranged by the layout. This component must implement the [`Drawable`] trait.
///
/// # Fields
/// - `L`: The layout used to arrange the contained drawable component.
/// - `D`: The drawable component to be displayed in the layout.
///
/// # Methods
/// - `inner`: Returns a mutable reference to the contained drawable component, allowing manipulation of the drawable's properties.
///

#[derive(Debug, Component)]
pub struct Bin<L: Layout, D: Drawable>(pub L, pub D);

impl<L: Layout, D: Drawable> OnEvent for Bin<L, D> {}

impl<L: Layout, D: Drawable> Bin<L, D> {
    /// Returns a mutable reference to the contained drawable component.
    ///
    /// This method allows access to the component within the bin for manipulation, such as changing its properties
    /// or updating its state.
    ///
    /// # Returns
    /// - A mutable reference to the contained `Drawable` component (`&mut D`).
    pub fn inner(&mut self) -> &mut D {
        &mut self.1
    }
}

/// A component that optionally displays another `Drawable` component.
///
/// The `Opt` component allows a `Drawable` component to be optionally shown or hidden while still maintaining the layout structure. 
/// The component will still occupy a slot in the layout even if it is hidden, which may lead to extra spacing in certain scenarios. 
/// However, it allows for dynamic display toggling without altering the layout significantly.
///
/// - `Stack`: The layout used for positioning the components.
#[derive(Debug, Component)]
pub struct Opt<D: Drawable + 'static>(Stack, Option<D>, #[skip] Option<D>);

impl<D: Drawable + 'static> OnEvent for Opt<D> {}

impl<D: Drawable + 'static> Opt<D> {
    /// Creates a new `Opt` component with an optional `Drawable` item.
    /// The item will be shown or hidden based on the `display` flag.
    ///
    /// # Parameters
    /// - `item`: The `Drawable` component to show or hide.
    /// - `display`: A boolean flag indicating whether to show (`true`) or hide (`false`) the component.
    ///
    /// # Returns
    /// An `Opt` component configured with the specified item and display status.
    ///
    /// # Example
    /// ```
    /// let button = Button::new("Click me!");
    /// let opt_component = Opt::new(button, true); // Component is displayed.
    /// ```
    pub fn new(item: D, display: bool) -> Self {
        match display {
            true => Opt(Stack::default(), Some(item), None),
            false => Opt(Stack::default(), None, Some(item)),
        }
    }

    /// Toggles the visibility of the component inside the `Opt`.
    /// If `display` is `true`, the first `Option<D>` is shown, otherwise the second `Option<D>` is shown.
    ///
    /// # Parameters
    /// - `display`: If `true`, show the first component; if `false`, show the second.
    ///
    /// # Example
    /// ```
    /// let mut opt_component = Opt::new(button, true);
    /// opt_component.display(false); // Hide the component
    /// opt_component.display(true);  // Show the component
    /// ```
    pub fn display(&mut self, display: bool) {
        match display {
            true if self.1.is_none() => self.1 = self.2.take(),
            false if self.2.is_none() => self.2 = self.1.take(),
            _ => {}
        }
    }

    /// Returns a mutable reference to the inner `Drawable` component that is currently displayed.
    /// This method will either return the component from `self.1` or `self.2`, depending on which is active.
    ///
    /// # Returns
    /// A mutable reference to the displayed `Drawable` component.
    ///
    /// # Example
    /// ```
    /// let mut opt_component = Opt::new(button, true);
    /// let inner_button = opt_component.inner();
    /// inner_button.set_label("New Label");
    /// ```
    pub fn inner(&mut self) -> &mut D {
        self.1.as_mut().unwrap_or_else(|| self.2.as_mut().unwrap())
    }

    pub fn is_showing(&self) -> bool {
        self.1.is_some()
    }
}


/// A component that conditionally displays one of two `Drawable` components.
///
/// The `EitherOr` component holds two `Drawable` components and uses an `Opt` component to conditionally display either the left or right component, based on the given display condition. This structure is useful when you need to toggle between two different components while still maintaining the layout's consistency.
#[derive(Debug, Component)]
pub struct EitherOr<L: Drawable + 'static, R: Drawable + 'static>(Stack, Opt<L>, Opt<R>);

impl<L: Drawable + 'static, R: Drawable + 'static> OnEvent for EitherOr<L, R> {}

impl<L: Drawable + 'static, R: Drawable + 'static> EitherOr<L, R> {
    /// Creates a new `EitherOr` component with two `Drawable` components (left and right).
    /// Initially, the left component is displayed and the right one is hidden.
    ///
    /// # Parameters
    /// - `left`: The `Drawable` component to be shown on the left side.
    /// - `right`: The `Drawable` component to be shown on the right side.
    ///
    /// # Returns
    /// An `EitherOr` component containing both left and right components, with the left one displayed.
    pub fn new(left: L, right: R) -> Self {
        EitherOr(Stack::default(), Opt::new(left, true), Opt::new(right, false))
    }

    /// Toggles the display of the left and right components.
    /// If `display_left` is `true`, the left component will be displayed and the right one hidden.
    /// If `display_left` is `false`, the right component will be displayed and the left one hidden.
    ///
    /// # Parameters
    /// - `display_left`: A boolean flag indicating whether the left component should be shown (`true`) or the right component (`false`).
    pub fn display_left(&mut self, display_left: bool) {
        self.1.display(display_left);
        self.2.display(!display_left);
    }

    /// Returns a mutable reference to the left `Drawable` component.
    /// This method provides access to the left component, which can be modified directly.
    ///
    /// # Returns
    /// A mutable reference to the left `Drawable` component.
    pub fn left(&mut self) -> &mut L { self.1.inner() }

    /// Returns a mutable reference to the right `Drawable` component.
    /// This method provides access to the right component, which can be modified directly.
    ///
    /// # Returns
    /// A mutable reference to the right `Drawable` component.
    pub fn right(&mut self) -> &mut R { self.2.inner() }
}
