use std::fs::File;
use std::io::Write;

fn main() {
    let commissioner = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministry = vec![
        "Internal affairs",
        "Justice",
        "Defense",
        "Power & steel",
        "Petroleum",
    ];
    let zone = vec![
        "South West",
        "North East",
        "South South",
        "South East",
        "North West",
    ];

    let mut file = File::create("merged.txt").expect("create failed");

    file.write_all(b"EFCC merged file of convicted ministers\n").expect("Write failed");
    file.write_all(b"commissioner | ministry | Zone\n").expect("write failed");

    for i in 0..commissioner.len() {
        println!("{} | {} | {}", commissioner[i], ministry[i], zone[i]);

        file.write_all(commissioner[i].as_bytes()).expect("write failed");
        file.write_all(b" | ").expect("write failed");
        file.write_all(ministry[i].as_bytes()).expect("write failed");
        file.write_all(b" | ").expect("write failed");
        file.write_all(zone[i].as_bytes()).expect("write failed");
        file.write_all(b"\n").expect("write failed");
    }

    println!("\nData written to file.");
}