use std::collections::HashMap;
use wascc_host::{host, Actor, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    host::add_actor(Actor::from_gantry(
        "MCDJMLT4P4M25F5VGOATKCOUAU26SSLMCKORT3EVT2WSTERQVUJ6DEW4",
    )?)?;
    host::add_native_capability(NativeCapability::from_file(
        "./examples/.assets/libnats_provider.dylib",
    )?)?;
    host::configure(
        "MCDJMLT4P4M25F5VGOATKCOUAU26SSLMCKORT3EVT2WSTERQVUJ6DEW4",
        "wascc:messaging",
        nats_config("testsubject"),
    )?;

    println!("**> Send a nats message to testsubject to test");

    std::thread::park();

    Ok(())
}

fn nats_config(sub: &str) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("SUBSCRIPTION".to_string(), sub.to_string());
    hm.insert("URL".to_string(), "nats://localhost:4222".to_string());

    hm
}
