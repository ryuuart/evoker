use crate::server_base::*;
use objc2::{
    extern_class, extern_methods,
    rc::{Allocated, Retained},
    runtime::ProtocolObject,
};
use objc2_foundation::{NSDictionary, NSRect, NSString};
use objc2_metal::{MTLCommandBuffer, MTLCreateSystemDefaultDevice, MTLDevice, MTLTexture};

extern_class!(
    #[unsafe(super(SyphonServerBase))]
    pub struct SyphonMetalServer;
);

impl SyphonMetalServer {
    extern_methods!(
        #[unsafe(method(alloc))]
        fn alloc() -> Allocated<Self>;

        /// Creates a new server with the specified human-readable name (which need not be unique) for a
        /// [MTLDevice] and given options. The server will be started immediately. Init may fail and
        /// return nil if the server could not be started.
        #[unsafe(method(initWithName:device:options:))]
        fn init_with_name(
            this: Allocated<Self>,
            name: &NSString,
            device: *const ProtocolObject<dyn MTLDevice>,
            options: Option<&NSDictionary>,
        ) -> Retained<Self>;

        /// Publishes the part of the texture described in region of the texture to clients.
        /// The texture is copied and can be safely modified once this method has returned.
        #[unsafe(method(publishFrameTexture:onCommandBuffer:imageRegion:flipped:))]
        pub fn publish_frame_texture(
            &self,
            texture_to_publish: *mut ProtocolObject<dyn MTLTexture>,
            command_buffer: *mut ProtocolObject<dyn MTLCommandBuffer>,
            image_region: &NSRect,
            is_flipped: bool,
        );

        #[unsafe(method(stop))]
        /// Stops the server instance. Use of this method is optional and releasing all
        /// references to the server has the same effect.
        pub fn stop(&self);
    );

    /// Convenience method to create a new server in one go
    pub fn new(name: &str) -> Retained<Self> {
        let instance = SyphonMetalServer::alloc();
        let name = NSString::from_str(name);
        let device = MTLCreateSystemDefaultDevice().expect("Couldn't use Metal GPU device.");
        let options = None;

        Self::init_with_name(instance, &name, Retained::as_ptr(&device), options)
    }

    /// Useful for when we need to control the pipeline outside with the device
    pub fn from_device(
        name: &str,
        device: *mut ProtocolObject<dyn MTLDevice>,
    ) -> Retained<SyphonMetalServer> {
        let instance = SyphonMetalServer::alloc();
        let name = NSString::from_str(name);
        let options = None;

        Self::init_with_name(instance, &name, device, options)
    }
}
