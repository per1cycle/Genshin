
struct User {
    alive: bool, 
    age: u32,
    realname: String,
    email: String
}

fn build_user(alive: bool, age: u32) -> User {
    return User {
        alive: alive,
        age: age,
        realname: "KG",
        email: "DD@d.d"
    }
}
fn main {

}