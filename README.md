
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Data Layout - LLVM Language Reference Manual](https://llvm.org/docs/LangRef.html#data-layout)
- [Programmable Interval Timer (PIT)](https://wiki.osdev.org/Programmable_Interval_Timer)

```sh
rustup component add rust-src
rustup component add llvm-tools-preview

qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/bootimage-mini_os.bin
```
