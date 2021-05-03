## General links

- [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)

## CC3200

- [Product Homepage](https://www.ti.com/product/CC3200): SimpleLink™ 32-bit ARM
  Cortex-M4 Wi-Fi wireless MCU with 2 TLS/SSL and 256kB RAM
- [Datasheet](https://www.ti.com/lit/gpn/cc3200): Contains memory map (p. 54-55)
- [Technical Reference Manual](https://www.ti.com/lit/pdf/swru367): Main
  documentation for peripherals
- [Launchpad Hardware User's Guide](https://www.ti.com/lit/pdf/swru372): Contains
  pin-out of the Launchpad board

## PAC (Peripheral Access Crate)

- [SVD File Format](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_Format_pg.html):
  Cortex Microcontroller Software Interface Standard (CMSIS) XML format to
  describe the peripherals of a microcontroller
- [svd2rust](https://github.com/rust-embedded/svd2rust): Generate Rust code from
  SVD files to easily access registers
- [msp432p401r](https://github.com/pcein/msp432p401r): Texas Instruments MSP432
  generated code using `svd2rust`
- [dslite2svd](https://github.com/m-labs/dslite2svd): Energia (an Arduino fork
  for TI devices) has a custom XML format to describe the peripherals. This
  project transform the files into standard SVD files. Although Energia supports
  the CC3200, the included XML does not have the peripherals listed, only very
  generic information is available. However, it has some peripherals for the
  CC3220S and CC3220SF.

## HAL (Hardware Abstraction Layer)

- [embedded-hal](https://github.com/rust-embedded/embedded-hal): Traits to
  implement 
- [tm4c-hal](https://github.com/rust-embedded-community/tm4c-hal)
- [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal): Its datasheet is
  available [here](https://www.st.com/resource/en/reference_manual/dm00031020-stm32f405415-stm32f407417-stm32f427437-and-stm32f429439-advanced-armbased-32bit-mcus-stmicroelectronics.pdf)

## Cortex-M

- [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart):
  Example usage of the different Cortex-M Rust libraries.

## Debugging

### Requirements on Windows:
- [OpenOCD](http://openocd.org/): `choco install openocd`
- [arm-none-eabi-gdb](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm):
  `choco install gcc-arm-embedded`
- [Zadig](https://zadig.akeo.ie/):
  - Download the executable
  - Options > List All Devices
  - Select the "USB <-> JTAG/SWD (Interface 0)
  - Select WinUSB
  - Click "Replace Driver"
  - For more info, see [CC3200 LaunchPad Getting Started Guide](https://www.ti.com/lit/ug/swru376e/swru376e.pdf),
    p. 24-25.

### Debugging in the command line

- In a first terminal, `openocd -f cc3200.cfg`
  - This starts the OpenOCD. It communicates with the MCU and acts as a GDB
    server
  - Keep this running at all time. If you need to restart it, you might have to
    disconnect and reconnect the board.
- In a second terminal, `cargo run`.
  - This compiles the code and start a GDB client properly initialized for
    debugging.
  - The MCU will break on main.
  - Have a look at [cc3200.gdb](./cc3200.gdb) file to see what initialization is
    required.
  - You can run `cargo run --release` to debug the release version of the
    program.

### Debugging in Visual Studio Code

Requirements:

- [Visual Studio Code](https://code.visualstudio.com/)
- [Cortex-Debug Extension](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug)

The [.vscode](./.vscode) directory contains [task definitions](./.vscode/tasks.json)
and [launch configurations](./.vscode/launch.json) for both Debug and Release.

- Launch openocd: `openocd -f cc3200.cfg` and keep it running.
- In VS Code, select one of the two debugging target and click the start button.
  - It will build the target, start and initialize a GDB session, upload the
    code and break on `main`.
  - It also offers a "Cortex Peripherals" view extracted from the SVD files
    where you can read and edit all the peripheral registers.

## Flashing the final program

- Generate the binary file to flash:
  `cargo objcopy --release --bin cc3200-example -- -O binary cc3200.bin`
- Put jumper J15 to set SOP pins to FLASH mode
- Connect the CC3200 to your computer
- In Uniflash 3.4:
  - Start a new configuration of "C32xx Serial(UART) Interface"
  - Set the COM port to the one of the CC3200 (you can find it in Windows Device
    Manager)
  - In "System Files" > `/sys/mcuimg.in`, set the "Url" to the binary file
    created, tick the "Update" and "Verify" boxes.
  - In "CC31xx/CC32xx Flash Setup and Control", click "Format".
- Disconnect the CC3200.
- Remove jumper J15 to let it run automatically when powering up.
- Reconnect the CC3200 and enjoy your program running!
