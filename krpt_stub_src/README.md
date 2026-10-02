# krpt_stub

The Rust source code for the decryption engine embedded inside every generated `.krpt` file.

`krpt_stub` is intentionally small and direct. It receives the Access Key and encrypted payload path from the generated shell wrapper, recovers the Main Key, decrypts the file payload, and writes the original data back to `decrypted_file.out`.

## The Philosophy: Keep the Decryption Engine Self-Contained

The main `krpt` program is responsible for encryption and packaging. The stub has one job: reverse that process without requiring the recipient to install `krpt`.

When `src/stubinject.rs` generates a `.krpt` file, it embeds the compiled `src/krpt_stub` binary directly into the script. At runtime, the script extracts that binary into a temporary directory and passes it:

```text
krpt_stub <ACCESS_KEY> <payload.bin>
```

The stub then handles the full decryption flow locally.

## How It Works

1. Reads the Access Key and payload path from the command-line arguments.
2. Reads the encrypted payload and validates the minimum metadata size.
3. Extracts the 16-byte salt, encrypted file payload, encrypted Main Key, and payload-size metadata.
4. Hashes the Access Key together with the salt using SHA-256.
5. Decrypts the 48-byte encrypted Main Key using AES-256-ECB and PKCS#7 padding.
6. Uses the recovered 32-byte Main Key to decrypt the file payload in 1040-byte encrypted chunks.
7. Writes each decrypted chunk directly to `decrypted_file.out`.

## Build

From the root of the `krpt` repository:

```bash
cargo build --release --manifest-path krpt_stub_src/Cargo.toml
```

The compiled binary will be generated at:

```text
krpt_stub_src/target/release/krpt_stub
```

Copy it into the main source directory:

```bash
cp krpt_stub_src/target/release/krpt_stub src/krpt_stub
```

The main `krpt` binary uses:

```rust
let stub_bytes = include_bytes!("krpt_stub");
```

so `src/krpt_stub` remains the compiled binary embedded into generated `.krpt` files, while this directory keeps its source code available and reproducible.

## Structure

```text
krpt_stub_src/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

The stub is kept as a standalone Rust crate to keep the encryption application and the self-contained decryption engine isolated from each other.
