use ecow::eco_format;
use objc2::mutability::Mutable;
use objc2::rc::Id;
use objc2::{class, extern_class, msg_send_id, ClassType};
use objc2_foundation::{NSData, NSObject};
use tiny_skia::Pixmap;

use crate::config;
use crate::world::SystemWorld;

extern_class!(
    pub struct Syphilm;

    unsafe impl ClassType for Syphilm {
        type Super = NSObject;
        type Mutability = Mutable;
        const NAME: &'static str = "Syphilm";
    }
);

impl Syphilm {
    pub fn init_with_data(data: &NSData) -> Id<Self> {
        let syphilm_class = class!(Syphilm);
        unsafe { msg_send_id![msg_send_id![syphilm_class, alloc], initWithData :data] }
    }
}

pub fn render_document() -> Pixmap {
    let config = config::load_config("config.toml");

    let ppi = 3.0;

    let world = SystemWorld::new(&config.input, &config.world_config, &config.process_config)
        .map_err(|err| eco_format!("{err}"))
        .unwrap();
    let document = typst::compile(&world).output.unwrap();

    let page = &document.pages[0];

    typst_render::render(page, ppi)
}
