mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ops::Deref;
use std::rc::Rc;

pub use crate::bindings::Gl as InnerGl;
pub use crate::bindings::*;

// Will deep clone everything inside. Except, cloning of Rc only increases its reference-count,
// while it keeps pointing to the same data. The data is destroyed only when all Rc instances are
// deallocated and the shared reference-count reaches zero.
#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(loadfn: F) -> Gl
        where F: FnMut(&'static str) -> *const types::GLvoid
    {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(loadfn))
        }
    }
}

// https://doc.rust-lang.org/book/ch15-02-deref.html
// a mechanism in rust to forward calls to inner implementation:
impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}