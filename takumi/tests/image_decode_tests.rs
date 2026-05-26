#![cfg(not(target_arch = "wasm32"))]

use takumi::resources::image::ImageSource;

#[test]
fn decodes_avif_input_as_bitmap() {
  let fixture = avif_fixture();
  let image = ImageSource::from_bytes(&fixture).unwrap();

  match image {
    ImageSource::Bitmap(bitmap) => {
      assert_eq!(bitmap.width(), 24);
      assert_eq!(bitmap.height(), 24);
    }
    ImageSource::Gif(_) => unreachable!("AVIF should decode as a static bitmap"),
    #[cfg(feature = "svg")]
    ImageSource::Svg(_) => unreachable!("AVIF should decode as a static bitmap"),
    _ => unreachable!("AVIF should decode as a static bitmap"),
  }
}

fn avif_fixture() -> Vec<u8> {
  const BASE64: &str = "\
AAAAHGZ0eXBhdmlmAAAAAGF2aWZtaWYxbWlhZgAAAbltZXRhAAAAAAAAACFoZGxyAAAAAAAAAABwaWN0AAAAAAAAAAAAAAAAAAAA\
AA5waXRtAAAAAAABAAAARmlsb2MAAAAAREAAAwABAAAAAAHdAAEAAAAAAAAAnwACAAAAAAJ8AAEAAAAAAAAAIwADAAAAAAKfAAEA\
AAAAAAAAvgAAAE1paW5mAAAAAAADAAAAFWluZmUCAAAAAAEAAGF2MDEAAAAAFWluZmUCAAAAAAIAAGF2MDEAAAAAFWluZmUCAAAB\
AAMAAEV4aWYAAAAAw2lwcnAAAACdaXBjbwAAAAxhdjFDgSACAAAAABNjb2xybmNseAABAA0AAIAAAAAUaXNwZQAAAAAAAAAYAAAA\
GAAAABBwaXhpAAAAAAMICAgAAAAMYXYxQ4EAHAAAAAAOcGl4aQAAAAABCAAAADhhdXhDAAAAAHVybjptcGVnOm1wZWdCOmNpY3A6\
c3lzdGVtczphdXhpbGlhcnk6YWxwaGEAAAAAHmlwbWEAAAAAAAAAAgABBIECAwQAAgSFAwaHAAAAKGlyZWYAAAAAAAAADmF1eGwA\
AgABAAEAAAAOY2RzYwADAAEAAQAAAYhtZGF0EgAKCDgRL3YQENACMpABRAAAljCHsadce/TP1AfVdp+4ih08DOGwbD9vCCWEW9a2\
slEerxUgkA0GauYG15tr9AaNn1yY2D1U1n2tj1i9uSIvArtPQCrQQuhu6HcWLkAb/sUnJmGuXlVtpo+PLcSp9apswndWwtw7tjYi\
Az+YWgZx0yY0uJwz1e491Urb9MzIb4WW5tTxOu1FTt/1hzw4EgAKBRgRL3YVMhhEAACrKvtq6WYjKERJIZ9+58+/FwRcJH4AAAAG\
RXhpZgAASUkqAAgAAAAGABIBAwABAAAAAQAAABoBBQABAAAAVgAAABsBBQABAAAAXgAAACgBAwABAAAAAgAAABMCAwABAAAAAQAA\
AGmHBAABAAAAZgAAAAAAAAAvGQEA6AMAAC8ZAQDoAwAABgAAkAcABAAAADAyMTABkQcABAAAAAECAwAAoAcABAAAADAxMDABoAMA\
AQAAAP//AAACoAQAAQAAABgAAAADoAQAAQAAABgAAAAAAAAA";

  decode_base64(BASE64)
}

fn decode_base64(input: &str) -> Vec<u8> {
  let mut out = Vec::with_capacity(input.len() * 3 / 4);
  let mut chunk = [0_u8; 4];
  let mut chunk_len = 0;

  for byte in input.bytes().filter(|byte| !byte.is_ascii_whitespace()) {
    chunk[chunk_len] = byte;
    chunk_len += 1;
    if chunk_len == 4 {
      push_base64_chunk(chunk, &mut out);
      chunk_len = 0;
    }
  }

  out
}

fn push_base64_chunk(chunk: [u8; 4], out: &mut Vec<u8>) {
  let a = decode_base64_value(chunk[0]);
  let b = decode_base64_value(chunk[1]);
  let c = if chunk[2] == b'=' {
    0
  } else {
    decode_base64_value(chunk[2])
  };
  let d = if chunk[3] == b'=' {
    0
  } else {
    decode_base64_value(chunk[3])
  };

  out.push((a << 2) | (b >> 4));
  if chunk[2] != b'=' {
    out.push((b << 4) | (c >> 2));
  }
  if chunk[3] != b'=' {
    out.push((c << 6) | d);
  }
}

fn decode_base64_value(byte: u8) -> u8 {
  match byte {
    b'A'..=b'Z' => byte - b'A',
    b'a'..=b'z' => byte - b'a' + 26,
    b'0'..=b'9' => byte - b'0' + 52,
    b'+' => 62,
    b'/' => 63,
    _ => unreachable!("invalid base64 fixture byte"),
  }
}
