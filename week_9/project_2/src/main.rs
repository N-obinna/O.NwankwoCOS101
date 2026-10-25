use std::io::Write;

fn main() {
      let students = vec![
   "Student Name,Matric Number,Department,Level",
   "Oluchi Mordi,ACC10211111,Accounting,300",
   "Adamas Aliyu,ECO10110101,Economics,100",
   "Shaia Balade,CSC10328828,Computer,200",
   "Adekunle Gold,EEE1020202,Electrical,200",
   "Bianca Edemoh,MEE10202001,Mechanical,100",
   ];
let mut file = std::fs::File::create("PAU-SMIS.csv").expect("cannot create file");
for row in students{
    println!("{}",row);
    let line = format! ("{}\n",row);
    file.write_all(line.as_bytes()).expect("failed to write row");
}
println!("\nPAU-SMIS is saved");

}