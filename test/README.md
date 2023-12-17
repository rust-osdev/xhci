# Test program for the `xhci` crate

## What is this?

This is a tiny program to verify that the `xhci` crate defines data structures, register accessors, etc. correctly, and to demostrate how to use the crate. It is a single UEFI binary, and when it runs, it does the following things:

- Find the xHCI controller.
- Initialize the controller.
- Enumerate and initialize all ports to which a device is connected.

This program does not interact with each device because it is beyond the scope of this crate.

You can use it as a reference for your own implementation, but note the following points:

- While this program is a single UEFI binary, usually a UEFI binary is used as a bootloader, and interacting with the xHCI controller is done by the OS kernel.
- This program depends on the identity-mapping that is set up by the UEFI firmware, and thus, it uses Rust pointers as physical addresses directly.

This program is designed based on [eXtensible Host Controller Interface for Universal Serial Bus (xHCI) Requirements Specification May 2019 Revision 1.2](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf).

## Running this program

Just run `make test` in this directory.
