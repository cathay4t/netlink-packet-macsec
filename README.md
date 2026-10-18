# netlink-packet-macsec

MACsec generic netlink packet definitions for the
[rust-netlink](https://github.com/rust-netlink) project.

This crate only parses and emits the netlink payload of the `macsec` generic
netlink family (`include/uapi/linux/if_macsec.h` in the Linux kernel). To
talk to the kernel, use a netlink generic connection, for example the
`genetlink` crate.

```rust,no_run
use netlink_packet_macsec::{
    MacsecAttribute, MacsecCmd, MacsecMessage, MacsecSecyAttribute,
};

let message = MacsecMessage {
    cmd: MacsecCmd::GetTxsc,
    attributes: vec![MacsecAttribute::Secy(vec![
        MacsecSecyAttribute::Sci(0x0ed1490827840001),
    ])],
};
```

License: MIT
