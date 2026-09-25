<!-- 
SPDX-FileCopyrightText: 2026 Contributors to ddcutil-varlink <https://github.com/digitaltrails/ddcutil-varlink>
SPDX-License-Identifier: GPL-2.0-or-later
-->

# Ddcutil daemons

Varlink and D-Bus ddcutil daemons for control of DDC Monitors/VDUs. Coded in 
Rust using varlink-crate and zbus-crate.  Two daemons are included:

ddcutil-varlink
: A varlink implementation of [com.ddcutil.service.varlink](varlink-frontend/varlink/com.ddcutil.service.varlink).

ddcutil-dbus
: A D-Bus implementation compatible with the C coded [ddcutil-service](https://github.com/digitaltrails/ddcutil-service).

The daemons are designed to run as userspace daemons. Providing libddcutil 
is installed with the correct privileges, the daemons need no additional 
privileges.  It's not recommended to run them as system level daemons, they
have never been tested in system space. 

> [!WARNING]
> When using these services, avoid excessively writing VCP values because each VDU's NVRAM likely has a write-cycle limit/lifespan. The suggested guideline is to limit updates to rates comparable to those observed when using the VDU's onboard controls. Avoid coding that might rapidly or infinitely loop, including when recovering from errors and bugs.
>
> Non-standard manufacturer specific features should only be experimented with caution, some may have irreversible consequences, including bricking the hardware.

> [!WARNING]
> The dbus-frontend is still work in progress, only some methods have been implemented, 
> most calls are stubbed.

> [!Tip]
> All methods in [com.ddcutil.service.varlink](varlink-frontend/varlink/com.ddcutil.service.varlink) 
> have now been implemented.
> 
> The [vdu_controls](https://digitaltrails.github.io/vdu_controls/), a control panel for 
> displays, can be configured to use ddcutil-varlink.
>
> Varlink development is more or less complete at this point.  Possible future work:
> - Packaging, probably initially targeting openSUSE Tumbleweed.
> - Replace varlink-crate by zlink-crate when zlink-crate reaches 1.0.

The aim of these services is to make it easier to create highly-responsive widgets 
and apps for [ddcutil](https://www.ddcutil.com/).   These services are based on [ddcutil-service](https://github.com/digitaltrails/ddcutil-service), a 
similar C-coded D-Bus service.

The services are written in Rust.   Compared to other implementations of similar 
services, the code is quite compact and the abstractions 
are relatively shallow. Providing you know Rust and a little about [varlink](https://varlink.org/)
or D-Bus, the code should be quite easy to follow.  

# Building the daemons

## Compilation

Use the normal Rust cargo command to build the executables
```
# Build everything
cargo build --release
# Or individually
cargo build --release --package ddcutil-backend
cargo build --release --bin ddcutil-varlink
cargo build --release --bin ddcutil-dbus

# Normally the release binaries wind up here:
ls -l ./target/release/
```

## Installation of the executables

Cargo can also be used to install the executables for a single user:
```
cargo install --path varlink-frontend
cargo install --path dbus-frontend

# The above normally installs to $HOME/.cargo/bin
ls -l $HOME/.cargo/bin
```
But you can copy these or the target directory outputs to anywhere
appropriate.

# Using the ddcutil-varlink daemon

Once built, the service should be run under a user account (assuming 
libddcutil is installed the required permissions).  For example, the 
service can be run out of the build directory:
```
RUST_LOG=debug ./target/release/ddcutil-varlink

[2026-09-24T21:55:31Z INFO  ddcutil_varlink] Running with user privileges (UID: 500)
[2026-09-24T21:55:31Z INFO  ddcutil_backend::ddcutil] Initializing ddcutil
(ddci_init                     ) Calling ldbus_start_sleep_watch_thread...
[2026-09-24T21:55:32Z DEBUG ddcutil_backend::ddcutil] Redetect displays
...
```
A socket should then be available on `unix:$XDG_RUNTIME_DIR/ddcutil-varlink.socket`. 
Any type of varlink client can be used to interact with the service. For example, 
systemd commandline client `varlinkctl` may be used as follows:

```
SERVICE="unix:$XDG_RUNTIME_DIR/ddcutil-varlink.socket"
INTERFACE="com.ddcutil.DdcutilInterface"
varlinkctl list-methods $SERVICE
varlinkctl introspect $SERVICE
varlinkctl call $SERVICE "${INTERFACE}.Detect" '{"include_offline":false}'
varlinkctl call $SERVICE "${INTERFACE}.GetVcp" '{"display_number":1,"vcp_code":16}'
varlinkctl call $SERVICE "${INTERFACE}.SetVcp" '{"display_number":5,"vcp_code":16,"new_value":50}'
varlinkctl call $SERVICE "${INTERFACE}.SetVcp" '{"display_number":2,"vcp_code":16,"new_value":70,"options":{"no_verify":true}}'
varlinkctl call $SERVICE "${INTERFACE}.GetMultipleVcp" '{"edid_base64":"AP///////wAi8Gk","vcp_codes":[16,20],"options":{"allow_edid_prefix":true}}'
varlinkctl --more --timeout=infinity call $SERVICE "${INTERFACE}.Subscribe" '{}'
```

## Building and running ddcutil-varlink for debugging

```
# Build and run for debugging
cargo build --debug
RUST_BACKTRACE=1 RUST_LOG=debug ./target/debug/ddcutil-varlink
```


## Installing ddcutil-varlink as a systemd auto-started service

To run the varlink service via systemd for a single user, create the following 
service files and confirm the executable `ExecStart` location. 

```
# $HOME/.local/share/systemd/user/user/ddcutil.service                                                                                                                      ✔  10664  09:32:53
[Unit]
Description=ddcutil Varlink Service
Requires=ddcutil.socket
After=ddcutil.socket
ls 
[Service]
Type=simple
ExecStart=%h/.cargo/bin/ddcutil-varlink
Environment=RUST_LOG=info

[Install]
WantedBy=default.target
```

```
# $HOME/.local/share/systemd/user/user/ddcutil.socket
[Unit]
Description=ddcutil Varlink Service Socket

[Socket]
ListenStream=%t/ddcutil-varlink.socket
SocketMode=0600

[Install]
WantedBy=sockets.target
```

Install the systemd unit and socket for a single user:
```
# Reload the user systemd manager daemon
systemctl --user daemon-reload

# Enable and start the socket unit immediately
systemctl --user enable --now ddcutil.socket

# Verify running
systemctl --user status ddcutil.socket
```

To install for all users, modify the above for installation
in your distro's shared user-unit location, and relocate 
the executable to somewhere like `/usr/bin` or `/usr/local/bin`.

## ddcutil-varlink clients

The GUI [vdu_controls](https://digitaltrails.github.io/vdu_controls/) can optionally use ddcutil-varlink as
its ddcutil interface.

The project's [example-clients](varlink-frontend/example-clients) 
directory contains some example clients.

## Rust varlink implementations

I evaluated Rust libraries _varlink_ and _zlink_.
I found varlink to generate code that worked out of the box, so I've
stuck with it. Trying to work with zlink generated code seemed to 
result in some difficult to resolve build errors.  I may further experiment 
with zlink at some point when it further matures.

# Using ddcutil-dbus daemon

The ddcutil-dbus daemon implements a subset of same interface as the C based
[ddcutil-service](https://github.com/digitaltrails/ddcutil-service).
Any existing client can be used, this includes the GUI [vdu_controls](https://digitaltrails.github.io/vdu_controls/).

Once built, running the executable should make a ddcutil-dbus service findable
via the normal broker, For example, we might run out of the build directory 
with debugging info:
```
RUST_LOG=debug ./target/release/ddcutil-dbus

DdcutilService running on D-Bus...
```
Then we could use busctl to check if it's registered.
```
busctl --user | grep DdcutilService
```

## Installing ddcutil-dbus as a pre-registered D-Bus service

TODO. 

# Acknowledgements

Thanks go out to Sanford Rockowitz ([rockowitz](https://github.com/rockowitz)) 
for [libddcutil, ddcutil](https://www.ddcutil.com/).


