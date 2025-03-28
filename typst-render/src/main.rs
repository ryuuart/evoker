mod args;
mod config;
mod download;
mod package;
mod render;
mod world;

use objc2_app_kit::NSApplication;

use objc2::{msg_send_id, rc::Id};
use objc2_foundation::NSData;
use render::{render_document, Syphilm};

fn main() {
    let image = render_document();
    let image_data = NSData::with_bytes(
        image
            .encode_png()
            .expect("Couldn't encode output to png")
            .as_ref(),
    );

    let syphilm = Syphilm::init_with_data(&image_data);
    let app: Id<NSApplication> = unsafe { msg_send_id![&syphilm, app] };

    unsafe { app.run() };
}
