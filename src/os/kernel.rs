// Safe and clean interface to kernel_sys.rs
// Use this whenever interfacing with kernel API.
// If API not here, add it from kernel_sys.rs.

// Do not edit kernel_sys.rs, it is being generated
// at build time.

use core::sync::atomic::AtomicUsize;
use core::{
    cell::{RefCell, UnsafeCell},
    default::Default,
    marker::{Send, Sync},
    mem,
    ops::{Deref, DerefMut},
};

use core::fmt::{self, Formatter};

use alloc::boxed::Box;

use core::fmt::Debug;

use super::kernel_sys as ksys;

// Re-exports
//
pub use super::kernel_sys::modeventhand_t as ModuleEventHandler;
pub use super::kernel_sys::modeventtype as ModuleEventType;
pub use super::kernel_sys::module_t as Module;
pub use super::kernel_sys::moduledata_t as ModuleData;

pub use super::kernel_sys::gid_t as Gid;
pub use super::kernel_sys::uid_t as Uid;

pub use super::kernel_sys::M_DEVBUF;
pub use super::kernel_sys::M_WAITOK;

pub use super::kernel_sys::GID_WHEEL;
pub use super::kernel_sys::UID_ROOT;

pub use super::kernel_sys::MAKEDEV_CHECKNAME;
pub use super::kernel_sys::MAKEDEV_ETERNAL;
pub use super::kernel_sys::MAKEDEV_ETERNAL_KLD;
pub use super::kernel_sys::MAKEDEV_NOWAIT;
pub use super::kernel_sys::MAKEDEV_REF;
pub use super::kernel_sys::MAKEDEV_WAITOK;
pub use super::kernel_sys::MAKEDEV_WHTOUT;

pub use super::kernel_sys::free;
pub use super::kernel_sys::malloc;
pub use super::kernel_sys::printf;
pub use super::kernel_sys::tprintf;
pub use super::kernel_sys::uprintf;

pub use super::kernel_sys::size_t;

/*pub use super::characterdevice::{CharacterDevice, CDev};
pub use super::uio::{UioReader, UioWriter};
pub use super::module::{ModuleEvents, SharedModule};
*/

unsafe impl Sync for ksys::cdev {}
unsafe impl Send for ksys::cdev {}

unsafe impl Sync for ksys::cdevsw {}
unsafe impl Send for ksys::cdevsw {}

impl Default for ksys::cdev {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Debug for ksys::cdev {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "cdev {{ .. }}")
    }
}

impl Default for ksys::cdevsw {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
impl Debug for ksys::cdevsw {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "cdevsw {{ .. }}")
    }
}

impl Default for ksys::uio {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
