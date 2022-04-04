// import hashlib
// import uuid
// import io
// from pathlib import Path

//use uuid::Uuid;
use md5::{Md5, Digest};
use std::{io, env, fs, io::prelude::*, fs::File, path::Path};
//use named::named;    
use std::collections::HashMap;
use io::BufWriter;
use hex_literal::hex;

use std::io::{BufRead, BufReader};
use std::str;

//use std::net::TcpStream;

struct HexDigest(String);
// #[derive(Debug)]
// enum HashType {
//     MD5,
// }


fn main() {
    //let key //32-byte has of the mapping's key values
    //let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");

    //let digest = b"abcdefghijklmnopqrstuvwxyz";
    //let key = key_hash(digest);
    //assert_eq!(format!("{:x}", key), "c3fcd3d76192e4007dfb496cca67e13b");
    let mut value = Md5::new();
    value.update(b"abcdefghijklmnopqrstuvwxyz");
    let mut result = value.finalize();
    println!("{:x}\n", result);
    //let val = md5_tem(b"abcdefghijklmnopqrstuvwxyz");
    //assert_eq!(format!("{:x}", result), "c3fcd3d76192e4007dfb496cca67e13b");
}
/*
fn md5_tem(mut val: String) {
    let mut value = Md5::new();
    value.update(val);
    let mut result = value.finalize();
    return result
}*/
/*
#[cfg(test)]
mod test {
    use md5::{Md5, Digest};
    fn check() {
        let mut value = Md5::new();
        value.update(b"abcdefghijklmnopqrstuvwxyz");
        let mut result = value.finalize();
        //println!("{:x}\n", result);
        //let val = md5_tem(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(format!("{:x}", result), "c3fcd3d76192e4007dfb496cca67e13b");
    }  
}*/

/*
// return hashed.hexdigest
// or (_: mapping)
fn key_hash(mapping: &mut HashMap<&str, &str>) -> i32  {
    let mut hashed = Md5::new();    //created for MD5 hash
    for (k, v) in mapping {
        hashed.update(String(v).encode());
    }
    return hashed.hexdigest();
} 


// def key_hash(mapping):
//     """
//     32-byte hash of the mapping's key values sorted by the key name.
//     This is often used to convert a long primary key value into a shorter hash.
//     For example, the JobTable in datajoint.jobs uses this function to hash the primary key of autopopulated tables.
//     """
//     hashed = hashlib.md5()
//     for k, v in sorted(mapping.items()):
//         hashed.update(str(v).encode())
//     return hashed.hexdigest()

// pub struct Key_arg {
//     pub val: u16,
// }
// const DEFAULT: Key_arg = { val: "" };

// #[named(defaults(b = false))]
// fn or(a: bool, b: bool) -> bool {
//     a || b
// }

// fn main() {
//     assert!(or!(a = true));
//     assert!(or!(a = true, b = true));
// }

#[named(defaults(init_string = ""))]
fn uuid_from_stream<S: Stream>(stream: &mut S, init_string: &str) -> u16 {
    let mut hashed = Md5::new();
    let chunk: bool = true;
    let chunk_size = 1 << 14;
    while chunk {
        chunk = stream.read(chunk_size);
        hashed.update(chunk);
    }
    let bytes = hashed.Digest();
    let uuid = Uuid::Uuid(bytes);
    return uuid;
}

// def uuid_from_stream(stream, *, init_string=""):
//     """
//     :return: 16-byte digest of stream data
//     :stream: stream object or open file handle
//     :init_string: string to initialize the checksum
//     """
//     hashed = hashlib.md5(init_string.encode())
//     chunk = True
//     chunk_size = 1 << 14
//     while chunk:
//         chunk = stream.read(chunk_size)
//         hashed.update(chunk)
//     return uuid.UUID(bytes=hashed.digest())


#[named(defaults(buffer = b"", init_string = ""))]
fn uuid_from_buffer(buffer: &str, init_string: &str) -> u16 {
    let buffer = BufWriter::new(buffer);
    init_string = "";
    return uuid_from_stream(buffer, init_string);
}

// def uuid_from_buffer(buffer=b"", *, init_string=""):
//     return uuid_from_stream(io.BytesIO(buffer), init_string=init_string)

#[named(defaults(init_string = ""))]
fn uuid_from_file(filepath: &str, init_string: &str) -> u16 {
    let path = Path::new(filepath);
    let init_string = "";
    return uuid_from_stream(path.open("rb"), init_string);
}

// def uuid_from_file(filepath, *, init_string=""):
//     return uuid_from_stream(Path(filepath).open("rb"), init_string=init_string)
*/