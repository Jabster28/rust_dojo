//! Just a quick crate to test all of the library's functions. 

use dojo::csv;

pub fn main() {
  println!("Testing CSV...");
  csv("Name,Street,City,Age
Peter Pan,Am Hang 5,12345 Einsam,42
Maria Schmitz,Kölner Straße 45,50123 Köln,43
Paul Meier,Münchener Weg 1,87654 München,65")
}
