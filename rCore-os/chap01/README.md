# rCore-Tutorial-v3
rCore-Tutorial version 3.x

+ [代码说明](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/index.html)

+ 运行
```shell
cd os
make run
```


+ 目录说明
```shell
├── bootloader(内核依赖的运行在 M 特权级的 SBI 实现，本项目中我们使用 RustSBI)
│   ├── rustsbi-k210.bin(可运行在 k210 真实硬件平台上的预编译二进制版本)
│   └── rustsbi-qemu.bin(可运行在 qemu 虚拟机上的预编译二进制版本)
├── LICENSE
├── os(我们的内核实现放在 os 目录下)
│   ├── Cargo.toml(内核实现的一些配置文件)
│   ├── Makefile
│   └── src(所有内核的源代码放在 os/src 目录下)
│       ├── console.rs(将打印字符的 SBI 接口进一步封装实现更加强大的格式化输出)
│       ├── entry.asm(设置内核执行环境的的一段汇编代码)
│       ├── lang_items.rs(需要我们提供给 Rust 编译器的一些语义项，目前包含内核 panic 时的处理逻辑)
│       ├── linker-k210.ld(控制内核内存布局的链接脚本以使内核运行在 k210 真实硬件平台上)
│       ├── linker-qemu.ld(控制内核内存布局的链接脚本以使内核运行在 qemu 虚拟机上)
│       ├── main.rs(内核主函数)
│       └── sbi.rs(调用底层 SBI 实现提供的 SBI 接口)
├── README.md
├── rust-toolchain(控制整个项目的工具链版本)
└── tools(自动下载的将内核烧写到 k210 开发板上的工具)
   ├── kflash.py
   ├── LICENSE
   ├── package.json
   ├── README.rst
   └── setup.py
```
## Dependency

### Binaries

* rustc 1.56.0-nightly (b03ccace5 2021-08-24)

* qemu: 5.0.0

* rustsbi: qemu[7d71bfb7] k210[563144b0]
