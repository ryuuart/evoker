use objc2::{extern_class, mutability, runtime::NSObject, ClassType};

extern_class!(
    pub struct SyphonServerBase;

    unsafe impl ClassType for SyphonServerBase {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);
