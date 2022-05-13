// Enums are types which have a few definite values

enum Color {
  Red,
  Green,
  Blue,
  RgbColor(u8,u8,u8),
  CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8},
}

fn getColor(myColor:Color) {

  match myColor {
    Color::Red => {println!(" ====> Red")}
    Color::Green => {println!(" ====> Green")}
    Color::Blue => {println!(" ====> Blue")} 
    Color::RgbColor(0, 0, 0) | Color::CmykColor {cyan:_, magenta:_, yellow:_, black: 255 } => {println!(" ====> Black")} //tuple
    Color::RgbColor(r, g, b) => {println!(" ====> rgb({}, {}, {})", r, g, b)} //struct
    _ => ()
  }
}

fn main() {

  let mut my_color:Color = Color::Green;
  getColor(my_color);

  my_color = Color::RgbColor(0, 0, 0);
  getColor(my_color);

  my_color = Color::RgbColor(120, 150, 200);
  getColor(my_color);

  my_color = Color::CmykColor{cyan: 0, magenta: 0, yellow: 0, black: 255};
  getColor(my_color);
}
