struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1: User = User {
        active: true,
        username: String::from("admin"),
        email: String::from("admin@google.com"),
        sign_in_count: 1,
    };

    let user2: User = User {
        email: String::from("user2@google.com"),
        ..user1
    };

    user1.email = String::from("updated_admin@google.com");

    // println!("user1 fields: {}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count); - Err. user2 borrowed user2.username.

    println!("user2 fields: {}, {}, {}, {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    let user1 = build_user(String::from("user1@google.com"), String::from("user1"));

    println!("user1 fields: {}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);


    let origin = Point(0, 0, 0);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);


    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
