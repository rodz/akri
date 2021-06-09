use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// TODO: make this configurable
pub const DISCOVERY_INTERVAL_SECS: u64 = 10;

/// File acting as an environment variable for testing discovery.
/// To mimic an instance going offline, kubectl exec into the pod running this discovery handler
/// and echo "OFFLINE" > /tmp/debug-echo-availability.txt.
/// To mimic a device coming back online, remove the word "OFFLINE" from the file
/// ie: echo "" > /tmp/debug-echo-availability.txt.
pub const DEBUG_ECHO_AVAILABILITY_CHECK_PATH: &str = "/tmp/debug-echo-availability.txt";
/// String to write into DEBUG_ECHO_AVAILABILITY_CHECK_PATH to make Other devices undiscoverable
pub const OFFLINE: &str = "OFFLINE";


fn main() {
    println!("hello from stdout!");
    eprintln!("hello from stderr!");
    for (key, value) in env::vars() {
        println!("{}={}", key, value);
    }
    let args: Vec<String> = env::args().collect();
    println!("Args are: {:?}", args);
    println!("");

    // open a path using the hostpath volume
    let path = Path::new("/mnt/storage/bacon_ipsum.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(format!("could not read {}", display).as_str());
    println!("{}", contents);
}

/*
impl DiscoveryHandler for DiscoveryHandlerImpl {
    fn discover(
        &self
    ) {
        info!("discover - called for debug echo protocol");
    }
}
*/
