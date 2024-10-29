pub fn antivm() -> bool {
    use std::process::Command;

    // List of vm and debuging tools 
    let vm_processes = [
        "WireShark",
        "IDA64", 
        "processhacker",
        "vmware",
        "VirtualBox",
        "qemu",
        "VBoxService",
        "dnSpy",
    ];

    let output = Command::new("tasklist").output(); // runs "tasklist" in cmd, im not sure but may only work on windows OS

    // checks output if its succesfull
    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout); // gets the output as a string
        // looks for the process
        for &process in &vm_processes {
            if output_str.contains(process) { 
                return true; // returns true if it finds its in a vm
            }
        }
    }

    // returns false if it does not find its in a vm
    false
}
