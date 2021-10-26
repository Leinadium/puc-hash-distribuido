use std::collections::HashMap;
use std::env;
use std::num::ParseIntError;

fn main() {

  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let mut scores = HashMap::new();

  let vermelho = "vermelho".to_string();

  scores.insert(String::from("azul"), 3);
  
  scores.insert("vermelho".to_string(), 5);
  scores.insert(vermelho, 2);
  
  let v = scores.get(&String::from("vermelho"));
  match v {
    Some(val) => println!("valor: {}", val),
    None => println!("nada"),
  }
  println!("experimentando imprimir hash tables:");  
  println!("{:?}", scores);

  
  let mut row: Vec<SpreadsheetCell> = Vec::new();
  
  row.push(SpreadsheetCell::Float(2.0));
  row.push(SpreadsheetCell::Int(4));
  row.push(SpreadsheetCell::Text(String::from("alo alo")));

  let mut index = 0;
  while index < 3 {
  //for x in &row {
    let elem = row.get(index);
    match elem {
      Some(x)=>
        match x {
          SpreadsheetCell::Text(y) => println!("texto: {}", y),
          SpreadsheetCell::Float(y) => println!("float: {}", y),
          SpreadsheetCell::Int(y) => println!("int: {}", y),
        }//,
      None =>println!("nada!"),
    }
    index +=1;
  }

  let mut v = vec![];
  for i in &args {
    let a: Result<i32, ParseIntError> = i.parse();
    let num = match a {
      Ok(number) => number,
      Err(e)=> 0,
    };
    println!("passou aqui {}", num);
    if num != 0 {
      v.push(num);
    }
  }

  for x in &mut v {
    *x += 50;
  }
  for i in &mut v {
    println!("{}", i);
  }

  let x = 5;
  let r = &x;
  println! ("r = {}", r);
}

