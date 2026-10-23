use anyhow::Result;
use ddp_rs::connection;
use ddp_rs::protocol;

// Testing a longer LED strip with offset

fn main() -> Result<()> {
    let mut conn = connection::DDPConnection::try_new(
        "192.168.178.102:4048",
        protocol::PixelConfig::default(),
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:4048")?,
    )?;

    let framerate = 60;
    let c: u64 = 1000 / framerate;
    let mut gg = 0;

    loop {
        let clr = test_section(gg, 400)?;
        conn.write_offset(&clr, 0)?;
        gg += 1;
        if gg >= 400 {
            gg = 0;
        }
        std::thread::sleep(std::time::Duration::from_millis(c));
    }
}

#[allow(dead_code)]
fn testfill_all(len: u64) -> Result<Vec<u8>> {
    let mut vec = Vec::with_capacity((len * 3) as usize);
    for _ in 0..len {
        vec.push(255_u8);
        vec.push(0_u8);
        vec.push(0_u8);
    }
    Ok(vec)
}

fn test_section(curr_sec: u64, len: u64) -> Result<Vec<u8>> {
    let mut vec = Vec::with_capacity((len * 3) as usize);

    for i in 0..len {
        if i == curr_sec || i == curr_sec + 3 {
            vec.push(255_u8);
            vec.push(0_u8);
            vec.push(0_u8);
        } else {
            vec.push(0_u8);
            vec.push(0_u8);
            vec.push(0_u8);
        }
    }

    Ok(vec)
}

