mod models;

use std::env;
use std::path::Path;
use std::{collections::HashMap, fs};
use std::time::Duration;
use std::thread;
use models::device::Device;
use models::request::DebugEchoDiscoveryDetails;
use models::request::DebugEchoResult;

/// Name debugEcho discovery handlers use when registering with the Agent
pub const DISCOVERY_HANDLER_NAME: &str = "debugEcho";
/// Label of the environment variable in debugEcho discovery handlers that sets whether debug echo registers
/// as discovering local instances on nodes rather than ones visible to multiple nodes
pub const DEBUG_ECHO_INSTANCES_SHARED_LABEL: &str = "DEBUG_ECHO_INSTANCES_SHARED";
/// Name of environment variable that is set in debug echo brokers. Contains the description of
/// the device.
pub const DEBUG_ECHO_DESCRIPTION_LABEL: &str = "DEBUG_ECHO_DESCRIPTION";
// TODO: make this configurable
pub const DISCOVERY_INTERVAL_SECS: u64 = 4;

/// File acting as an environment variable for testing discovery.
/// To mimic an instance going offline, kubectl exec into the pod running this discovery handler
/// and echo "OFFLINE" > /tmp/debug-echo-availability.txt.
/// To mimic a device coming back online, remove the word "OFFLINE" from the file
/// ie: echo "" > /tmp/debug-echo-availability.txt.
pub const DEBUG_ECHO_AVAILABILITY_CHECK_PATH: &str = "/data/storage/debug-echo-availability.txt";
/// String to write into DEBUG_ECHO_AVAILABILITY_CHECK_PATH to make Other devices undiscoverable
pub const OFFLINE: &str = "OFFLINE";

fn main() {
    println!("Wasi Debug Echo is up and running :)");

    // Input variables
    let mut input: DebugEchoDiscoveryDetails;
    let mut descriptions;

    // check other config options
    let mut offline = fs::read_to_string(DEBUG_ECHO_AVAILABILITY_CHECK_PATH)
            .unwrap()
            .contains(OFFLINE);
    
    let mut first_loop = true;
    let mut count = 0;

    loop {
        thread::sleep(Duration::from_secs(DISCOVERY_INTERVAL_SECS));
        println!("Count: {}", count);
        count +=1;

        input = read_input_file();
        descriptions = input.descriptions;

        let availability =
            fs::read_to_string(DEBUG_ECHO_AVAILABILITY_CHECK_PATH).unwrap_or_default();
        
        if (availability.contains(OFFLINE) && !offline) || offline && first_loop {
            if first_loop {
                first_loop = false;
            }
            // If the device is now offline, return an empty list of instance info
            offline = true;
            // Output empty value
            write_output_file(Vec::new());
        } else if (!availability.contains(OFFLINE) && offline) || !offline && first_loop {
            if first_loop {
                first_loop = false;
            }
            offline = false;
            let devices = descriptions
                .iter()
                .map(|description| {
                    let mut properties = HashMap::new();
                    properties.insert(
                        DEBUG_ECHO_DESCRIPTION_LABEL.to_string(),
                        description.clone(),
                    );
                    Device {
                        id: description.clone(),
                        properties,
                        mounts: Vec::default(),
                        device_specs: Vec::default(),
                    }
                })
                .collect::<Vec<Device>>();
            // Send device list
            write_output_file(devices);
        }
    }
}

pub fn read_input_file () -> DebugEchoDiscoveryDetails  {
    let path = Path::new("/data/storage/in.in");
    let display = path.display();

    let new_contents = fs::read_to_string(path).expect(format!("could not read {}", display).as_str());
    println!("Checked for input and found: ");
    println!("{}", new_contents);

    let new_details: DebugEchoDiscoveryDetails = 
            deserialize_discovery_details(&new_contents).unwrap();

    return new_details;
}

pub fn write_output_file (_devices: Vec<Device>) {
    let path = Path::new("/data/storage/out.out");

    // Write output values on DebugEchoResult
    let output_obj : DebugEchoResult = DebugEchoResult {
        devices: _devices,
    };

    //TODO: handle errors
    let json_output = serde_json::to_string(&output_obj).unwrap();
    println!("output: {}", json_output);

    fs::write(path, json_output);
}

/// This obtains the expected type `T` from a discovery details String by running it through function `f` which will
/// attempt to deserialize JSON the String.
pub fn deserialize_discovery_details<T>(discovery_details: &str) -> Result<T, anyhow::Error>
where
    T: serde::de::DeserializeOwned,
{
    let discovery_handler_config: T = serde_json::from_str(discovery_details)?;
    Ok(discovery_handler_config)
}
