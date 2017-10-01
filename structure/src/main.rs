struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        name: String::from("somename123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        name: String::from("anothername567"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
