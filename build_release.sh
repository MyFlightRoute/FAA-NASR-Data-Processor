# Build script for building on MacOS based devices.
# For other systems, you may need to remove all Apple references from this file.

rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

cross build --release --target x86_64-unknown-linux-musl
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin

cd target/x86_64-apple-darwin/release
cp faa-nasr-data-processor ../../release/faa-nasr-data-processor_x86_64-apple-darwin

cd ../../x86_64-unknown-linux-musl/release
cp faa-nasr-data-processor ../../release/faa-nasr-data-processor_x86_64-unknown-linux-musl

cd ../../x86_64-pc-windows-gnu/release
cp faa-nasr-data-processor.exe ../../release/faa-nasr-data-processor_x86_64-windows-gnu.exe

cd ../../aarch64-apple-darwin/release
cp faa-nasr-data-processor ../../release/faa-nasr-data-processor_aarch64-apple-darwin