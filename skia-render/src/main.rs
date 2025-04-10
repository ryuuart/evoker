use std::borrow::BorrowMut;
use std::ops::Deref;
use std::{borrow::Borrow, mem};

use nannou::prelude::*;
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_core_foundation::{CGFloat, CGPoint, CGRect, CGSize};
use objc2_foundation::NSRect;
use objc2_metal::{MTLCommandQueue, MTLTexture};
use studio::MetalContext;
use syphon::metal_server::SyphonMetalServer;
use wgpu::TextureUsages;
use wgpu_hal::{api, Device, TextureUses};

mod studio;

struct Model {
    _window: WindowId,
    _syphon_server: Retained<SyphonMetalServer>,
    _metal_context: MetalContext,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 1024)
        .title("nannou")
        .view(view)
        .event(event)
        .build()
        .unwrap();
    let _metal_context = MetalContext::new();
    let _syphon_server =
        SyphonMetalServer::from_device("Open World Game", &_metal_context.metal_device);

    Model {
        _window,
        _syphon_server,
        _metal_context,
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        self._syphon_server.stop();
    }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let wgpu_texture = frame.texture();
    unsafe {
        wgpu_texture.as_hal::<api::Metal, _>(|metal_texture| {
            if let Some(metal_texture) = metal_texture {
                let metal_texture = metal_texture.raw_handle();
                let metal_texture: *const ProtocolObject<dyn MTLTexture> =
                    mem::transmute(metal_texture.as_ref());

                _model._syphon_server.publish_frame_texture(
                    metal_texture,
                    Retained::as_ptr(&_model._metal_context.command_queue.commandBuffer().unwrap()),
                    NSRect::new(
                        CGPoint { x: 0.0, y: 0.0 },
                        CGSize::new(
                            CGFloat::from_u32(wgpu_texture.width()).unwrap(),
                            CGFloat::from_u32(wgpu_texture.height()).unwrap(),
                        ),
                    ),
                    true,
                );
            };
        })
    };
}

fn main() {
    nannou::app(model).run();
}
