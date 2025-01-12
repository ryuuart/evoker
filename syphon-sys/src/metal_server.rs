use crate::server_base::*;
use objc2::{
    extern_class, extern_methods, mutability,
    rc::{Allocated, Retained},
    runtime::ProtocolObject,
    ClassType,
};
use objc2_foundation::{NSDictionary, NSString};
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice};

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
        fn init_with_name(
            this: Allocated<Self>,
            name: &NSString,
            device: *mut ProtocolObject<dyn MTLDevice>,
            options: Option<&NSDictionary>,
        ) -> Retained<Self>;
    }
);

impl SyphonMetalServer {
    pub fn new(name: &str) -> Retained<Self> {
        let instance = SyphonMetalServer::alloc();
        let name = NSString::from_str(name);
        let device = unsafe { MTLCreateSystemDefaultDevice() };
        let options = None;

        Self::init_with_name(instance, &name, device, options)
    }
}
