#![cfg_attr(not(any(feature = "native-simulator", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "native-simulator", test))]
extern crate alloc;

#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::default_alloc!();

mod error;
use error::Error;
use alloc::vec;

pub fn program_entry() -> i8 {
    ckb_std::debug!("Enter callee contract!");

    match callee() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

pub fn callee() -> Result<(), error::Error> {
    return Err(Error::E);
}
