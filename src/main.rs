fn main() {
    study_how_to_use_struct();
}

fn study_how_to_use_struct() {
    let mut user1 = build_user(String::from("rhksdlr134@naver.com"), String::from("rhksldr134"));

    user1.email = String::from("kwanigi2005@gmail.com");

    let user2 = update_user_email_username(user1, String::from("foo@email.com"), String::from("foo"));

    println!("User2's email is : {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn update_user_email_username(user: User, email: String, username: String) -> User {
    User {
        email,
        username,
        ..user
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Struct without fields
struct AlwaysEqual;