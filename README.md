
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Data Layout - LLVM Language Reference Manual](https://llvm.org/docs/LangRef.html#data-layout)
- [Programmable Interval Timer (PIT)](https://wiki.osdev.org/Programmable_Interval_Timer)
- [PS/2 Keyboard](https://wiki.osdev.org/PS/2_Keyboard)
- [Slab allocation](https://en.wikipedia.org/wiki/Slab_allocation)
- [Buddy memory allocation](https://en.wikipedia.org/wiki/Buddy_memory_allocation)
- [メモリ管理、Buddyシステム、kmalloc、スラブアロケータ](http://www.coins.tsukuba.ac.jp/~yas/coins/os2-2010/2011-01-11/)
- [スラブアロケータ](https://wiki.bit-hive.com/linuxkernelmemo/pg/%E3%82%B9%E3%83%A9%E3%83%96%E3%82%A2%E3%83%AD%E3%82%B1%E3%83%BC%E3%82%BF)

```sh
rustup component add rust-src
rustup component add llvm-tools-preview

qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/bootimage-mini_os.bin
```
