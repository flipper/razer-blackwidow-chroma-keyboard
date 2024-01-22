const VENDOR_ID: u16 = 0x1532;
const PRODUCT_ID: u16 = 0x0203;

const REQUEST_TYPE: u8 = 0x21;
const REQUEST: u8 = 0x09;
const VALUE: u16 = 0x0300;
const INTERFACE: u16 = 0x2;

const PAYLOAD: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x04,
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x04, 0x00
];

const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

fn main() -> Result<(), rusb::Error> {
    let mut handle = match rusb::open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID) {
        Some(handle) => handle,
        None => return Err(rusb::Error::NotFound),
    };

    handle.claim_interface(INTERFACE as u8)?;

    let length = handle.write_control(REQUEST_TYPE, REQUEST, VALUE, INTERFACE, PAYLOAD, TIMEOUT)?;

    handle.release_interface(INTERFACE as u8)?;

    println!("Wrote {} bytes", length);

    Ok(())
}
