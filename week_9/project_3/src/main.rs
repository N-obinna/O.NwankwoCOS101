fn main() {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South",
        "South",
        "South",
    ];

    println!("EFCC MERGED RECORDS");
    println!("====================");

    for i in 0..commissioners.len() {
        println!(
            "Commissioner: {} | Ministry: {} | Zone: {}",
            commissioners[i], ministries[i], zones[i]
        );
    }
}