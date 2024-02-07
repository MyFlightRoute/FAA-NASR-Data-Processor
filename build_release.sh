rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-pc-windows-gnu
#rustup target add x86_64-apple-darwin
#rustup target add aarch64-apple-darwin

cross build --release --target x86_64-unknown-linux-musl
cross build --release --target x86_64-pc-windows-gnu
#cross build --release --target x86_64-apple-darwin
#cross build --release --target aarch64-apple-darwin