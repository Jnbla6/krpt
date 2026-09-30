# krpt

A fast, secure command-line utility built in Rust that generates **standalone, self-decrypting executables**. `krpt` leverages a two-key envelope encryption architecture, AES-256 block ciphers, and PBKDF2 key derivation to package your data so it can be decrypted anywhere—without requiring the recipient to install `krpt` or any external dependencies.

## The Philosophy: Minimalist Execution

True security thrives in simplicity. `krpt` strips away bloated graphical interfaces, convoluted command-line flags, and fragile dependency chains. Embracing the core UNIX philosophy, `krpt` reduces the decryption process to its absolute rawest form.

The recipient does not install software. They do not run a setup script. They simply open the executable, append their cryptographic access key on line 2 directly after the `(# )` marker, and run the file. The binary acts as an autonomous engine, evaluating the key and unpacking the data through pure technical minimalism.

## Features

* **Zero-Dependency Air-Gap Support:** The self-contained nature of the compiled payload makes it perfect for highly restricted or air-gapped environments. Transport the encrypted file via USB and decrypt it on a target machine with no internet access and no pre-installed libraries.
* **Envelope Encryption (Two-Key Methodology):** Implements industry best practices by separating the Data Encryption Key (Main Key) from the Key Encryption Key (Second Key) for superior cryptographic isolation.
* **Cryptographic Hardening:** Powered by `src/hashing_keys.rs`, the utility enforces PBKDF2 (Password-Based Key Derivation Function 2) and cryptographic salting. This intentionally delays key derivation, making brute-force and dictionary attacks computationally unfeasible.
* **Dynamic Binary Injection:** Rather than outputting standard ciphertext, `src/stubinject.rs` injects the encrypted payload directly into a pre-compiled Rust binary stub (`krpt_stub`).
* **Memory-Safe Cryptography:** Built entirely in Rust, the execution is protected from standard buffer overflows or memory leaks during the encryption and decryption cycles.

## The Two-Key Methodology (Envelope Encryption)

To ensure maximum security and prevent cryptanalysis against your primary data, `krpt` utilizes a two-key envelope architecture. This is highly secure, but entirely automated for the end user via an interactive terminal flow.

1. **The Main Key (Data Encryption Key):** You provide a main key to do the heavy lifting of encrypting the actual file payload using AES-256.
2. **The Second Key (Access Key):** Instead of using a standard password directly on the massive data payload (which is a security risk), `krpt` uses this Second Key to encrypt the *Main Key*.

By never using the user's password to encrypt large volumes of data, it severely limits an attacker's ability to perform known-plaintext attacks.

## Usage: The Two-Step Lifecycle

### 1. Encrypting a File

Pass your target file into `krpt`. The interactive CLI will prompt you for both keys and automatically build your self-decrypting payload.

```text
$ cargo run <target_file.ext>
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/krypt <target_file.ext>`
enter the main key to encrypt the file
<YOUR_MAIN_KEY>
enter the second key to encrypt the main key
note! that you will use this key to decrypt the file
<YOUR_ACCESS_KEY>
building the final file...
Success! Executable text script generated at: <target_file.ext>.krpt

```

### 2. Decrypting the File (The Minimalist Way)

**The recipient only needs to know the Second Key** (`<YOUR_ACCESS_KEY>`).

1. Open the generated `.krpt` file in any standard text editor (nano, vim, notepad, etc.).
2. Navigate to **line 2** and locate the `(# )` marker.
3. Insert your **Second Key (Access Key)** directly after the marker (e.g., `(# <YOUR_ACCESS_KEY>)`).
4. Save the file and execute it directly from the terminal.

```text
$ ./<target_file.ext>
Success! File decrypted successfully as: decrypted_file.out

```

*The binary self-evaluates the Second Key, decrypts the Main Key, unlocks the payload, and cleanly writes the original plain file to disk.*

## Architecture

The project is strictly modularized to separate core cryptographic operations from payload generation:

* **`src/stubinject.rs` & `src/krpt_stub**`: Handles the binary injection, binding the encrypted data into the executable engine.
* **`src/encrypt.rs`**: Manages the core AES-256 block cipher implementation for both the Envelope and Data encryption.
* **`src/hashing_keys.rs`**: Handles PBKDF2 key derivation and salting logic to harden the Second Key.
* **`src/takesfile.rs`**: Manages the I/O ingestion of the target files.
