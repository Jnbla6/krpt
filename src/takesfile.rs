// function to read the plain file and pass it to the encrypt function

use std::io::{self, Read, Write, Cursor};
use std::fs::File;

fn takesdata(mut reader: impl Read, mut write: impl Write) -> io::Result<()> {
    
}