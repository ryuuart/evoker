use objc2::{extern_class, runtime::NSObject};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct SyphonServerBase;
);
