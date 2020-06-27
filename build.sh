#!/bin/bash
mkdir -p target/deploy/arm32
mkdir -p target/deploy/arm64
mkdir -p target/deploy/x86
mkdir -p target/deploy/x64

# arm32
cross build --target armv7-unknown-linux-gnueabihf --release
arm-linux-gnueabi-strip target/armv7-unknown-linux-gnueabihf/release/raspi_monitor
gzip -9 -c target/armv7-unknown-linux-gnueabihf/release/raspi_monitor > target/deploy/arm32/raspi_monitor.gz
echo $(sha1sum target/armv7-unknown-linux-gnueabihf/release/raspi_monitor) > target/deploy/arm32/sha1
echo $(sha1sum target/deploy/arm32/raspi_monitor.gz) >> target/deploy/arm32/sha1

# arm64
cross build --target aarch64-unknown-linux-gnu --release
aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/raspi_monitor
gzip -9 -c target/aarch64-unknown-linux-gnu/release/raspi_monitor > target/deploy/arm64/raspi_monitor.gz
echo $(sha1sum target/aarch64-unknown-linux-gnu/release/raspi_monitor) > target/deploy/arm64/sha1
echo $(sha1sum target/deploy/arm64/raspi_monitor.gz) >> target/deploy/arm64/sha1

# x86
cross build --target i586-unknown-linux-gnu --release
i586-elf-strip target/i586-unknown-linux-gnu/release/raspi_monitor
gzip -9 -c target/i586-unknown-linux-gnu/release/raspi_monitor > target/deploy/x86/raspi_monitor.gz
echo $(sha1sum target/i586-unknown-linux-gnu/release/raspi_monitor) > target/deploy/x86/sha1
echo $(sha1sum target/deploy/x86/raspi_monitor.gz) >> target/deploy/x86/sha1

# x64
cargo build --release
strip target/release/raspi_monitor
gzip -9 -c target/release/raspi_monitor > target/deploy/x64/raspi_monitor.gz
echo $(sha1sum target/release/raspi_monitor) > target/deploy/x64/sha1
echo $(sha1sum target/deploy/x64/raspi_monitor.gz) >> target/deploy/x64/sha1
