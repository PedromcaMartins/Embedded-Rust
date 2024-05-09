#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_blue = 0;
        let mut num_red = 0;

        for shirt_color in &self.shirts {
            match shirt_color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}



#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}



fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_perf1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_perf1);
    println!("The user with preference {:?} gets {:?}",
        user_perf1, 
        giveaway1,
    );

    let user_perf2 = None;
    let giveaway2 = store.giveaway(user_perf2);
    println!("The user with preference {:?} gets {:?}",
        user_perf2, 
        giveaway2,
    );



    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);



    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    println!("{:#?}", list);
    list.sort_by_key(|r| r.width);
    println!("Sorted: {:#?}", list);
}