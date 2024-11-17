set arch riscv:rv64
define hook-quit
    set confirm off
end
set disassemble-next-line on
target remote localhost:3333
mon reset halt
load target/riscv64gc-unknown-none-elf/debug/ada
