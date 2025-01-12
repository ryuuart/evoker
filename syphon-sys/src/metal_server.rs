use crate::server_base::*;
use objc2::{
    extern_class, extern_methods, mutability,
    rc::{Allocated, Retained},
    runtime::ProtocolObject,
    ClassType,
};
use objc2_foundation::{NSDictionary, NSRect, NSString};
use objc2_metal::{MTLCommandBuffer, MTLCreateSystemDefaultDevice, MTLDevice, MTLTexture};

extern_class!(
    pub struct SyphonMetalServer;

    unsafe impl ClassType for SyphonMetalServer {
        type Super = SyphonServerBase;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_methods!(
    unsafe impl SyphonMetalServer {
        #[method_id(initWithName:device:options:)]
        /// Creates a new server with the specified human-readable name (which need not be unique) for a
        /// [MTLDevice] and given options. The server will be started immediately. Init may fail and
        /// return nil if the server could not be started.
        fn init_with_name(
            this: Allocated<Self>,
            name: &NSString,
            device: *mut ProtocolObject<dyn MTLDevice>,
            options: Option<&NSDictionary>,
        ) -> Retained<Self>;

        #[method(publishFrameTexture:onCommandBuffer:imageRegion:flipped:)]
        /// Publishes the part of the texture described in region of the texture to clients.
        /// The texture is copied and can be safely modified once this method has returned.
        pub fn publish_frame_texture(
            &self,
            texture_to_publish: *mut ProtocolObject<dyn MTLTexture>,
            command_buffer: *mut ProtocolObject<dyn MTLCommandBuffer>,
            image_region: &NSRect,
            is_flipped: bool,
        );

        #[method(stop)]
        /// Stops the server instance. Use of this method is optional and releasing all
        /// references to the server has the same effect.
        pub fn stop(&self);
    }
);

impl SyphonMetalServer {
    /// Convenience method to create a new server in one go
    pub fn new(name: &str) -> Retained<Self> {
        let instance = SyphonMetalServer::alloc();
        let name = NSString::from_str(name);
        let device = unsafe { MTLCreateSystemDefaultDevice() };
        let options = None;

        Self::init_with_name(instance, &name, device, options)
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
