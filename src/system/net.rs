#[cfg(target_os = "linux")]
use {
    crate::helpers::file::file_open,
    std::sync::Mutex,
    std::ops::Add,
};
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn get_ipaddr() -> String {
    let mut ipaddr = String::new();

    let output = Command::new("ipconfig")
        .args(&["/all"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.contains("IPv4 Address") {
            ipaddr = line.split_whitespace().last().unwrap().to_string();
        }
    }

    ipaddr
}

#[cfg(target_os = "linux")]
pub fn get_ipaddr() -> String {
    let mut final_str = String::new();
    let mut selected_intr = String::new();
    const KNOWN_INTERFACES: [&str; 10] = [
        "enp0s", "eth", "wlan", "wlp",
        "enx", "en", "wl", "wwp",
        "wwan", "wwn",
    ];

    let intr_tree: String = file_open("/proc/net/dev");

    let lines = intr_tree.lines()
        .collect::<Vec<&str>>();

    for line in lines.iter().skip(2) {
        let mut line = line.split_whitespace();
        let interface = line.next().unwrap();

        if KNOWN_INTERFACES.iter().any(|&x| interface.contains(x)) {
            selected_intr = interface.to_string().replace(":", "");
            break;
        }
    }

    // Get ip for the selected interface
    let connect_tree: String = file_open("/proc/net/if_inet6");

    let lines = connect_tree.lines()
        .collect::<Vec<&str>>();

    for line in lines.iter() {
        let mut line = line.split_whitespace();
        
        let hexa = line.next().unwrap().trim();
        let interface = line.nth(4).unwrap().trim();

        if interface == selected_intr {
            final_str = format!("{} ({})", hexa, interface);
        };
    }

    final_str
}

// #[cfg(target_os = "linux")]
// pub fn get_ipaddr() -> String {
//     let final_str: Mutex<String> = Mutex::new(String::new());
//     let intr = file_open("/proc/net/route");

//     let lines: &Vec<&str> = &intr.lines().collect();
//     let mut interface = String::new();

//     lines.into_iter().for_each(|line| {
//         if line.contains("00000000") {
//             interface = line.split("\t").collect::<Vec<&str>>()[0].to_string();
//         }
//     });

//     let output = Command::new("ifconfig")
//         .arg(interface.clone())
//         .output()
//         .expect("Failed to execute ifconfig");

//     let output = String::from_utf8(output.stdout).unwrap();

//     let lines: &Vec<&str> = &output.lines().clone().collect();

//     let mut next: bool = false;

//     let process_ip = |line: &str| {
//         let ip = line.split(" ").collect::<Vec<&str>>()[1].to_string();
//         final_str.lock().unwrap().push_str(&ip);
//     };

//     lines.into_iter().for_each(|line| {
//         if next {
//             line.replace("\t", "")
//                 .split("  ")
//                 .collect::<Vec<&str>>()
//                 .into_iter()
//                 .for_each(|item| {
//                     if item.contains("inet") {
//                         process_ip(item);
//                     }
//                 });

//             next = false;
//         }

//         if line.contains(&interface) {
//             next = !next;
//         }
//     });

//     let x = final_str
//         .lock()
//         .unwrap()
//         .to_string()
//         .add(format!(" ({})", interface).as_str()); 
        
//     x
// }