# keeb


## Heavly based off of
https://github.com/bschwind/key-ripper/

## Dependencies

```
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs
```

## Flash Code

Hold the "USB Boot" button (near the QSPI chip), and either press the reset button or re-insert the USB cable to put the board in USB mass-storage bootloader mode.

```
cargo run --release
```
## TODO
- Add ability to have split keyboard
- Add led code
- Turn into OS for safer code
- Impliment multiple layers
- Use both cores

