# KRNL

An experimental x86_64 kernel written in Rust, booted via Limine.

## Status 

Early stage/in progress of development.

Currently boots into a blank screen with a white box(framebuffer) at left top

## Building process

Requirements: 
 - Rust nightly (look at `rust-toolchain.toml`)
 - xorriso (for iso creation)
 - qemu (for booting and testing it)

Building code

```bash
    cargo build -p kernel
```

## Post cargo build

**DO NOT SKIP THIS PART**
```bash
cp target/x86_64-unknown-none/debug/kernel iso/boot/kernel
```

## ISO creation
Run this ->

You can change the output name it's `krnl.iso` now but like i said you can change it
```bash
xorriso -as mkisofs -b boot/limine-bios-cd.bin -no-emul-boot  -boot-load-size 4 -boot-info-table -o krnl.iso iso
```

## Running via qemu

If you didn't change the name it will be like this

```bash
qemu-system-x86_64 -cdrom krnl.iso
```

## LICENSE

This project is licensed under the GNU General Public License v2.0 — see the
[LICENSE](LICENSE) file for details.