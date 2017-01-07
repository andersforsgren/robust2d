use std::sync::{Once, ONCE_INIT};
use std::marker::PhantomData;
use ffi;

pub struct Library {
    phantom: PhantomData<()>,
}

static INIT: Once = ONCE_INIT;

impl Library {
    pub fn init() -> Self {
        INIT.call_once(|| {
            unsafe {
                ffi::exactinit();
            }
        });
        Library { phantom: PhantomData }
    }
}
