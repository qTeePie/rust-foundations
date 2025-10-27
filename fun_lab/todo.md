Make smth out of this:

```rust
/*
    ✨ Structs ✨
    Structs lets us implement some object-oriented like functionality to our Rust programs.

    Similar to Structs in C, these are structures that can hold multiple related values in key:value pairs.

    -----
    **Important note on _Struct Update Syntax_**
    > Note that the struct update syntax uses = like an assignment;
    this is because it moves the data, just as we saw in the “Variables and Data Interacting with Move” section.
    In this example, we can no longer use user after creating user2 because the String in the username field of user was moved into user2.
    If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user,
    then user would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait,
    so the behavior we discussed in the “Stack-Only Data: Copy” section would apply. We can also still use user.email in this example,
    because its value was not moved out of user.
    ----

    Tuple structs - hold multiple related fields like regular structs, but their fields dont have associated names.
    (Think RGB, Points, and similar 🎨)
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    fav_color: Option<Color>, // Will learn more about optionality in later chapters, just playin around
    origin: Option<Coordinates>,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
        fav_color: None,
        origin: None,
    }
}

// ✨ Implement methods for User
impl User {
    fn logon(&mut self) {
        self.active = true;
        println!("✅ {} is now logged ON", self.username);
    }

    fn logoff(&mut self) {
        self.active = false;
        println!("🚪 {} has logged OFF", self.username);
    }

    fn signin(&mut self) {
        self.sign_in_count += 1;
        self.active = true;
        println!(
            "🔐 {} signed in! Total sign-ins: {}",
            self.username, self.sign_in_count
        );
    }

    fn set_color(&mut self, color: Color) {
        // tell me what u like 😉
        self.fav_color = Some(color);
        println!("🎨 {} picked a new favorite color!", self.username);
    }

    fn set_origin(&mut self, origin: Coordinates) {
        // tell me what u from 😉
        self.origin = Some(origin);
        println!("📍 {} set their origin coordinates!", self.username);
    }

    fn profile(&self) {
        println!("\n===== 🐾 User Profile 🐾 =====");
        println!("👤 Username: {}", self.username);
        println!("📧 Email: {}", self.email);
        println!("🔢 Sign-ins: {}", self.sign_in_count);
        // Status
        println!(
            "{}",
            if self.active {
                "🟢 Online"
            } else {
                "🔴 Offline"
            }
        );

        // Pattern matching expression → if let PATTERN = VALUE
        if let Some(Color(r, g, b)) = &self.fav_color {
            println!("🎨 Favorite Color: rgb({}, {}, {})", r, g, b);
        } else {
            println!("🎨 Favorite Color: none");
        }

        if let Some(Coordinates(lat, lon)) = &self.origin {
            println!("📍 Origin: {:.4}° N, {:.4}° E", lat, lon);
        } else {
            println!("📍 Origin: unknown");
        }

        println!("==============================\n");
    }
}

// Tuple structs for fun
struct Color(i32, i32, i32);
struct Coordinates(f64, f64);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Coordinates(58.9528, 5.7333);

    let mut user = build_user(
        String::from("snow_leopard@princess.am"),
        String::from("snowleopard357"),
    );

    user.signin(); // 🔐 signs in
    user.logoff(); // 🚪 logs off
    user.logon(); // ✅ logs back on
    user.set_color(black); // 🎨 set color
    user.set_origin(origin); // 📌 coordinates

    // 😍 Cute profile summary
    user.profile();
}
```

and this:

```rust
// 🧠 ----------------------------------------------------------
// STRUCTS vs ENUMS — the ultimate sibling rivalry 😭🔥
// --------------------------------------------------------------
// Structs  = "AND" data ➜ combines several fields together.
// Enums    = "OR" data ➜ one of several possible variants.
// --------------------------------------------------------------

#[derive(Debug)]
struct User {
    username: String,
    age: u8,
}

// Our enum can *hold* different kinds of info.
#[derive(Debug)]
enum LoginStatus {
    LoggedIn(User),
    LoggedOut,
    Banned { reason: String },
}

fn main() {
    // 1️⃣ create a normal struct instance
    let izzy = User {
        username: String::from("NovaStark"),
        age: 98,
    };

    // 2️⃣ store that struct *inside* an enum variant
    let status1 = LoginStatus::LoggedIn(izzy);
    let status2 = LoginStatus::LoggedOut;
    let status3 = LoginStatus::Banned {
        reason: String::from("hacked the matrix 😈"),
    };

    // 3️⃣ handle each variant differently
    show_status(status1);
    show_status(status2);
    show_status(status3);
}

// --------------------------------------------------------------
// 🧩 Enums make pattern matching EASY ✨
// --------------------------------------------------------------
fn show_status(status: LoginStatus) {
    match status {
        LoginStatus::LoggedIn(user) => {
            println!("👤 {} (age {}) just logged in.", user.username, user.age);
        }
        LoginStatus::LoggedOut => {
            println!("👋 User is logged out.");
        }
        LoginStatus::Banned { reason } => {
            println!("🚫 Banned for reason: {}", reason);
        }
    }
}
```

add more silly stuff later so this will be less boring.
