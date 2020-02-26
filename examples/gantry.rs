use std::collections::HashMap;
use wascc_host::{host, Actor, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    host::add_actor(Actor::from_gantry(
        "MBHRSJORBXAPRCALK6EKOBBCNAPMRTM6ODLXNLOV5TKPDMPXMTCMR4DW",
    )?)?;
    host::add_native_capability(NativeCapability::from_file(
        "./examples/.assets/libwascc_nats.dylib",
    )?)?;
    host::configure(
        "MBHRSJORBXAPRCALK6EKOBBCNAPMRTM6ODLXNLOV5TKPDMPXMTCMR4DW",
        "wascc:messaging",
        nats_config("gantry.local"),
    )?;

    println!("**> Send a nats message to gantry.local to test");

    std::thread::park();

    Ok(())
}

fn nats_config(sub: &str) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("SUBSCRIPTION".to_string(), sub.to_string());
    hm.insert("URL".to_string(), "nats://localhost:4222".to_string());

    hm
}
