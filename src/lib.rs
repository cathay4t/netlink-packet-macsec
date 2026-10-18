// SPDX-License-Identifier: MIT

//! The `netlink-packet-macsec` crate is designed to parse and emit generic
//! netlink packets of the MACsec family. The goal of this crate is saving
//! crate users from reading the kernel netlink code.
//!
//! The MACsec generic netlink family is defined in
//! `include/uapi/linux/if_macsec.h` of the Linux kernel.
//!
//! ```no_run
//! use netlink_packet_macsec::{
//!     MacsecAttribute, MacsecCmd, MacsecMessage, MacsecSecyAttribute,
//! };
//!
//! let message = MacsecMessage {
//!     cmd: MacsecCmd::GetTxsc,
//!     attributes: vec![MacsecAttribute::Secy(vec![
//!         MacsecSecyAttribute::Sci(0x0ed1490827840001),
//!     ])],
//! };
//! ```

mod attribute;
mod message;
#[cfg(test)]
mod test;

pub use attribute::{
    MacSecCipherId, MacSecOffload, MacSecValidate, MacsecAttribute,
    MacsecOffloadAttribute, MacsecRxscAttribute, MacsecRxscList,
    MacsecRxscStatsAttribute, MacsecSaAttribute, MacsecSaList,
    MacsecSaStatsAttribute, MacsecSecyAttribute, MacsecSecyStatsAttribute,
    MacsecTxscStatsAttribute,
};
pub use message::{MacsecCmd, MacsecMessage};
