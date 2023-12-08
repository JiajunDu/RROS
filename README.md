# RROS

[![Documentation](https://img.shields.io/badge/view-docs-blue)](https://bupt-os.github.io/website/docs/tutorial/docker/)
[![.github/workflows/ci.yaml](https://github.com/BUPT-OS/RROS/actions/workflows/ci.yaml/badge.svg)](https://github.com/BUPT-OS/RROS/actions/workflows/ci.yaml)
[![Zulip chat](https://img.shields.io/badge/chat-on%20zulip-brightgreen)](https://rros.zulipchat.com/)
![RROS](https://img.shields.io/badge/RROS-0.0.1-orange)
[![en](https://img.shields.io/badge/lang-en-yellow.svg)](https://github.com/BUPT-OS/RROS/blob/master/README.md)
[![zh](https://img.shields.io/badge/lang-中文-yellow.svg)](https://github.com/BUPT-OS/RROS/blob/master/README.zh.md)


RROS is a dual-kernel OS, consisting of a real-time kernel (in Rust) and a general-purpose kernel (Linux). RROS is compatible with almost all native Linux programs and offers real-time performance superior to RT-Linux. It is also being experimented with as the host OS for in-orbit satellites ([Tiansuan Project](http://www.tiansuan.org.cn/)).

Here is an [architecture diagram](https://bupt-os.github.io/website/architecture.png) and a [TODO: demo video](#) of RROS.

## News

- [2023.11.30] RROS is presented in Xenomai Workshop 2023 ([photos](#)).
- [2023.11.28] :fire: RROS is open-sourced!

## Why RROS

RROS is primarily intended for satellites (onboard computers, payloads, etc). The key incentive is the trend that nowadays satellites serve both traditional satellite-borne real-time tasks (e.g., communication and positioning) and general-purpose tasks that need mature, complicated software support (e.g., data compression and machine learning). That catalyzes the dual-kernel architecture of RROS. Taking a step further, the real-time kernel of RROS is fully implemented in Rust for better safety and robustness. However, RROS can be used in scenarios like automatic cars, IoTs, industrial control, etc.

The advantages of RROS are:

* **Hard real-time**: 
RROS offers superior real-time performance compared to RT-Linux. RROS is designed with an efficient task scheduler that can quickly respond to external events, reducing task switching and processing delays.
* **Compatibility**: 
RROS is compatible with almost every Linux program, allowing seamless migration of complex Linux applications such as TensorFlow and Kubernetes. You can also easily modify your general Linux programs into a more real-time counterpart.
* **Easy to use**: 
RROS facilitates easy programming and debugging of real-time programs. RROS uses the libevl interface to call real-time APIs for user programs, allowing you to use tools like gdb, kgdb, and QEMU.
* **Robustness**:
The real-time kernel of RROS is carefully written in Rust, making it safer and more robust, especially for memory and concurrency issues.

## Quick start

On Linux (Debian-like distros), do the following:

> Running RROS needs the following requirements: TODO: add the requirements of RROS. If you don't have a proper environment, you can follow our [environment document](https://bupt-os.github.io/website/docs/tutorial/environment/).

1. Clone this repository:

   ```bash
   
   git clone https://github.com/BUPT-OS/RROS.git
   ```

2. Install Rust toolchain:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   switch to `beta-2021-06-23-x86_64-unknown-linux-gnu`. Currently, we only support this compiler version.

   ```bash
   rustup toolchain install beta-2021-06-23-x86_64-unknown-linux-gnu
   ```

   Set the rust toolchain for this project:

   ```bash
   cd RROS
   rustup override set beta-2021-06-23-x86_64-unknown-linux-gnu
   ```

   Add the rust `no-std` component.

   ```bash
   rustup component add rust-src
   ```
   
3. Select compile options

   Create a default configuration:

   ```bash
   export CROSS_COMPILE=aarch64-linux-gnu-
   export ARCH=arm64

   make LLVM=1 defconfig
   make LLVM=1 menuconfig
   ```

   select the following options:

   ```
   General Setup --->  Rust Support
   Kernel Features ---> Bupt real-time core
   ```

   You may need to cancel the option versioning support to enable `Rust support`:

   ```
   Enable loadable module support ---> Module versioning support.
   ```

4. Compile the kernel

   ```bash
   export CROSS_COMPILE=aarch64-linux-gnu-
   export ARCH=arm64
   make LLVM=1 -j
   ```

   If you want to boot on Raspberry PI 4, you need to generate dtbs and modules additionally.

   ```bash
   export INSTALL_MOD_PATH=/path/to/mod
   export INSTALL_DTBS_PATH=/path/to/dtbs
   make modules_install dtbs_install -j
   ```

   And move `broadcom`, `lib`, `overlays`, and `Image` to the boot partition of the SD card.

5. Run on simulator

   You need a filesystem to boot the kernel on QEMU. 

   Here's an example of how to run on QEMU:

   ```bash
   qemu-system-aarch64 -nographic  \
       -kernel Image \
       -drive file=ubuntu-20.04-server-cloudimg-arm64.img \
       -drive file=cloud_init.img,format=raw \
       -initrd ubuntu-20.04-server-cloudimg-arm64-initrd-generic \
       -machine virt-2.12,accel=kvm \
       -cpu host  -enable-kvm \
       -append "root=/dev/vda1 console=ttyAMA0"  \
       -device virtio-scsi-device \
       -smp 4 \
       -m 4096
   ```

## Documentation

Check out our detailed [documentation](https://bupt-os.github.io/website/docs/).

## Communication & Contribution

Contact us at [Zulip Forum](https://rros.zulipchat.com/) or with email `buptrros AT gmail.com`.

Contributions are also very welcomed! [Check it out](https://bupt-os.github.io/website/docs/contributing/contributing/).

## Roadmap

See [here](https://bupt-os.github.io/website/docs/roadmap/roadmap) for our future roadmap.

## Who are we

We are a [research group](https://bupt-os.github.io/website/docs/team/team/) at BUPT.

## Release

The RROS relies on both the dovetail and the Rust for Linux(RFL), neither of which currently provides patches. Integrating one into the other at a high frequency is challenging. As a result, RROS is currently tied to Linux kernel version 5.13, built on top of linux-dovetail-v5.13, and readily compatible with RFL patch v1. Fortunately, RFL is swiftly making its way into the mainline Linux kernel. We plan to release new versions once most of the RFL APIs we depend on are available in the linux-dovetail mainline. At that point, we will further consider Long-Term Support (LTS) versions.

## Acknowledgements

RROS has benefitted from the following projects/resources.
- [Evl/xenomai (linux-evl)](https://evlproject.org/core/). We learned from evl core how to implement a dual kernel and use dovetail for interrupt virtualization and libevl for user library. Thanks, @Philippe for his genius work and patient explanation in the riot!
- [Rust-for-Linux](https://github.com/Rust-for-Linux/linux): We use RFL to write RROS in Linux. We ask a lot of questions on RFL Zulip and constructing safety abstractions. Kudos to @ojeda, @Wedson,  @Alex, @boqun, @Gary, @Björn in the RFL community for patiently helping us. We hope to bring more safety abstraction back to the RFL in return!
- [Muduo](https://www.cnblogs.com/wsg1100/p/13836497.html): His detailed blog gives us insights for xenomai/evl project.
- All prospective contributors to RROS in the future!

## License

The source code of RROS is under the License of GPL-2.0.
