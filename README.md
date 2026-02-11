### 1. System Preparation

Install the necessary packages for RDP, VPN, and smart card support:

```
paru -Syu
paru -S base-devel zenity polkit openvpn freerdp opensc ccid pcsclite
sudo systemctl enable --now pcscd
```

### 2. Configuration File Setup
VPN Adjustment (Arch-based distros)

For CachyOS and other Arch-based distributions, you must modify your OpenVPN configuration:

    Open your .ovpn file.

    Locate the line group nogroup.

    Change it to group nobody.

    Note: On most other distributions, nogroup is the default, but it is worth double-checking.

Script Configuration

In the same directory as your executable file, create a text file named config. This file must contain exactly 4 lines:

    Server IP Address

    Domain Login

    Domain Password
