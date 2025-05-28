use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;

pub struct MyApp;

impl App for MyApp {
    async fn background_tasks(ctx: &mut HeadlessContext) -> Tasks {
        vec![]
    }
    async fn plugins(ctx: &mut Context, h_ctx: &mut HeadlessContext) -> (Plugins, Tasks) {
        let (pel_plugin, p_tasks) = PelicanUI::new(ctx, h_ctx).await;

        (std::collections::HashMap::from([
            (std::any::TypeId::of::<PelicanUI>(), Box::new(pel_plugin) as Box<dyn std::any::Any>),
        ]), p_tasks)
    }

    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        Box::new(Button::secondary(ctx, None, "HELLO SP", None, |ctx: &mut Context| log::error!("CLICKED")))
    }
}

create_entry_points!(MyApp);

fn main() {
    desktop_main()
}
