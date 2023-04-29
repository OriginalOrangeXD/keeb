#![deny(unsafe_code)]
#![deny(warrnings)]
#![deny(missing_docs)]
#![no_main]
#![no_std]

use panic_semihosting as _;

mod app {
    use cortex_m_semihosting::
