#[derive(Debug)]

struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area_complicated(width1, height1)
  );

  
  let rect1 = (30, 50);

  println!(
    "The area of the rectangle is {} square pixels.",
    area_tuple(rect1)
  );


  let rect1 = Rectangle {
    width: 30, 
    height: 50
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    area_struct(&rect1)
  );

  // println!("Rectangle struct is {}", rect1); - Err. Display function not implemented
  
  println!("Rectangle struct is {:#?}", rect1);


  let scale = 2;
  let rect1 = Rectangle {
    width: dbg!(15 * scale), 
    height: 50
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    dbg!(area_struct(&rect1))
  );

  dbg!(&rect1);
  dbg!(rect1);
  // dbg!(&rect1); - Err. dbg consumes rect1. user & instead
}

fn area_complicated(width: u32, height: u32) -> u32 {
  width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area_struct(dimensions: &Rectangle) -> u32 {
  dimensions.width * dimensions.height
}