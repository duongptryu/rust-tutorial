use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
   //Vector
   let mut v = vec![1,2,3,4,5];

   let mut v2 = Vec::new();
   v2.push(1);
   v2.push(2);
   v2.push(3);
   v2.push(4);
   v2.push(5);

   println!("{:?}", v2);

   let four = &v[3];

   println!("four = {}", four);

   match v.get(4) {
       Some(four) => println!("This is four element = {}", four),
       _ => println!("This is not a four element"),
   }

   for i in &mut v {
      *i += 10;
   }

   for i in &v {
      println!("{}", i)
   }

   enum SheetCell {
      Int(i32),
      Float(f64),
      Text(String)
   }

   let row = vec![SheetCell::Int(5), SheetCell::Float(10.12), SheetCell::Text(String::from("color"))];

   match &row[0] {
      &SheetCell::Float(i) => {println!("{}", i)},
      _ => println!("This is not a float")
   }


   //String
   //UTF8

   let s1 = String::from("");
   let s2 = String::new();
   let s3 = "Xin Chào";
   let s4 = s3.to_string();
   println!("{}", s3);

   let s5 = s2 + " in chasdasd";
   println!("{}", s5);

   for i in s3.chars() {
      println!("{}", i);
   }

   for i in s3.graphemes(true) {
      println!("{}", i);
   }

   //hashmap
   let mu = String::from("MU");
   let mc = String::from("MC");

   let mut scores = HashMap::new();
   scores.insert(&mu, 10);
   scores.insert(&mc, 9);

   let score = scores.get(&mc);
   println!("score {:?}", score);

    for (key, value) in scores {
      println!("{}{}", key, value);
    }
}