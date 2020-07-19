//! Storage for DMA system base and write-back addresses.
//! 
//! The DMA system requires two contiguous blocks of memory to hold the first descriptor and write-back location for
//! each channel. This module provides `StorageN` structures to hold these blocks of memory, where 1 <= N <= 32.
use core::marker::PhantomData;
use typenum::consts::*;
use typenum::{Unsigned, IsLessOrEqual};
use crate::TransferDescriptor;

/// The maximum amount of channels that can exist.
#[cfg(feature = "samd5x")]
pub type CHANMAX = U32;
/// The maximum amount of channels that can exist.
#[cfg(feature = "samd21")]
pub type CHANMAX = U12;

mod sealed {
    use super::*;
    
    pub trait Sealed {}

    impl<T: Unsigned + IsLessOrEqual<CHANMAX, Output = True>> Sealed for UnsafeStorage<T> {}
    impl Sealed for Storage1 {}
    impl Sealed for Storage2 {}
    impl Sealed for Storage3 {}
    impl Sealed for Storage4 {}
    impl Sealed for Storage5 {}
    impl Sealed for Storage6 {}
    impl Sealed for Storage7 {}
    impl Sealed for Storage8 {}
    impl Sealed for Storage9 {}
    impl Sealed for Storage10 {}
    impl Sealed for Storage11 {}
    impl Sealed for Storage12 {}
    
    #[cfg(feature = "samd5x")]
    mod samd5x {
        use super::*;

        impl Sealed for Storage13 {}
        impl Sealed for Storage14 {}
        impl Sealed for Storage15 {}
        impl Sealed for Storage16 {}
        impl Sealed for Storage17 {}
        impl Sealed for Storage18 {}
        impl Sealed for Storage19 {}
        impl Sealed for Storage20 {}
        impl Sealed for Storage21 {}
        impl Sealed for Storage22 {}
        impl Sealed for Storage23 {}
        impl Sealed for Storage24 {}
        impl Sealed for Storage25 {}
        impl Sealed for Storage26 {}
        impl Sealed for Storage27 {}
        impl Sealed for Storage28 {}
        impl Sealed for Storage29 {}
        impl Sealed for Storage30 {}
        impl Sealed for Storage31 {}
        impl Sealed for Storage32 {}
    }
}

macro_rules! dma_storage {
    ($n:tt) => {
        paste::item! {
            /// Storage type for base and write-back memory.
            #[derive(Default)]
            #[repr(align(16))]
            pub struct [<Storage $n>] {
                baseaddr: [$crate::TransferDescriptor; $n],
                wbaddr: [$crate::TransferDescriptor; $n]
            }

            impl $crate::DmaStorage for [<Storage $n>] {
                type Size = typenum::consts::[<U $n>];

                fn baseaddr(&self) -> *const $crate::TransferDescriptor {
                    self.baseaddr.as_ptr()
                }

                fn wbaddr(&self) -> *const $crate::TransferDescriptor {
                    self.wbaddr.as_ptr()
                }
            }
        }
    };
}

#[cfg(feature = "samd5x")]
mod samd5x {
    dma_storage!(32);
    dma_storage!(31);
    dma_storage!(30);
    dma_storage!(29);
    dma_storage!(28);
    dma_storage!(27);
    dma_storage!(26);
    dma_storage!(25);
    dma_storage!(24);
    dma_storage!(23);
    dma_storage!(22);
    dma_storage!(21);
    dma_storage!(20);
    dma_storage!(19);
    dma_storage!(18);
    dma_storage!(17);
    dma_storage!(16);
    dma_storage!(15);
    dma_storage!(14);
    dma_storage!(13);
}

#[cfg(feature = "samd5x")]
pub use samd5x::*;

dma_storage!(12);
dma_storage!(11);
dma_storage!(10);
dma_storage!(9);
dma_storage!(8);
dma_storage!(7);
dma_storage!(6);
dma_storage!(5);
dma_storage!(4);
dma_storage!(3);
dma_storage!(2);
dma_storage!(1);

/// Trait for accessing the base and write-back addresses of a storage unit.
pub trait DmaStorage: sealed::Sealed {
    /// The number of channels supported.
    type Size: Unsigned + IsLessOrEqual<CHANMAX, Output = True>;

    /// Get the address for the base descriptor memory.
    fn baseaddr(&self) -> *const TransferDescriptor;
    /// Get the address for the write-back descriptor memory.
    fn wbaddr(&self) -> *const TransferDescriptor;
}

/// A user allocated storage type.
/// 
/// If you prefer to allocate the base and write-back memory regions, you can use this type to do so. However, to 
/// prevent undefined behaviour or memory bugs, make sure to uphold the following invariants.
/// 
/// 1. The base and write-back addresses MUST point to contiguous blocks of memory.
/// 2. The base and write-back addresses MUST be 128-bit aligned.
/// 2. The length of the base and write-back memory regions MUST be equal.
/// 3. The base and write-back memory regions MUST be statically allocated.
/// 4. The generic type T MUST be an unsigned typenum less than or equal to the length of the base and write-back 
///    memory regions.
/// 
/// The base and write-back memory regions' length may be greater than 32, however the DMA system will ignore any further
/// memory past 32 `TransferDescriptor`'s. The base and write-back memory regions may overlap, but it is not 
/// recommended.
pub struct UnsafeStorage<T: Unsigned + IsLessOrEqual<CHANMAX, Output = True>> {
    baseaddr: &'static mut [TransferDescriptor],
    wbaddr: &'static mut [TransferDescriptor],
    _index: PhantomData<T>
}

impl<T: Unsigned + IsLessOrEqual<CHANMAX, Output = True>> UnsafeStorage<T> {
    /// Create a custom DMA memory storage unit.
    /// 
    /// # Safety
    /// 
    /// This operation is unsafe as it requires you uphold certain invariants, as explained in the description of the 
    /// `UnsafeStorage` type.
    pub unsafe fn new(baseaddr: &'static mut [TransferDescriptor], wbaddr: &'static mut [TransferDescriptor]) -> UnsafeStorage<T> {
        UnsafeStorage {
            baseaddr,
            wbaddr,
            _index: PhantomData
        }
    }

    /// Get back the memory pointers to the base and write-back regions.
    pub fn into_inner(self) -> (&'static mut [TransferDescriptor], &'static mut [TransferDescriptor]) {
        (self.baseaddr, self.wbaddr)
    }
}

impl<T: Unsigned + IsLessOrEqual<CHANMAX, Output = True>> DmaStorage for UnsafeStorage<T> {
    type Size = T;

    fn baseaddr(&self) -> *const TransferDescriptor {
        self.baseaddr.as_ptr()
    }

    fn wbaddr(&self) -> *const TransferDescriptor {
        self.wbaddr.as_ptr()
    }
}
