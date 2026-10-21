use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("nigerian_brewies.csv").expect("cannot create");
    
    let header = "Larger,Stout,Non-alcoholic";
    file.write_all(header.as_bytes()).expect("Failed to create header");
    
    let row_1 = "\n33 Export,Legend,Maltina";
    file.write_all(row_1.as_bytes()).expect("Failed to create row 1");
    
    let row_2 = "\nDesperados,turbo king,amstel malta";
    file.write_all(row_2.as_bytes()).expect("Failed to create row 2");
    
    let row_3 = "\nGoldberg,Williams,Malta gold";
    file.write_all(row_3.as_bytes()).expect("Failed to create row 3");
    
    let row_4 = "\nGulder,,Fayrouz";
    file.write_all(row_4.as_bytes()).expect("Failed to create row 4");
    
    let row_5 = "\nHeineken,,";
    file.write_all(row_5.as_bytes()).expect("Failed to create row 5");
    
    let row_6 = "\nStar,,";
    file.write_all(row_6.as_bytes()).expect("Failed to create row 6");
    
    println!("Nigerian brewies data is saved");
}
