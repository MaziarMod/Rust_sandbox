fn main() {
   let country_code = 44;

   let country = match country_code {
       44 => "UK",
       01 => "Canada",
       98 => "Iran",
       1..=1000 => "unknown", // menas 1 to 1000 inclusive
       _ => "invalid" // anything else
   };

   println!("Country code {} --> {}", country_code, country);

   let is_valid = false;

   let status = match is_valid {
       true => "yes",
       false => "no"
   };

   println!("Is valid: {} --> {}", is_valid, status);
}
