/* A Marco Polo Game

If the nama Marco is given, the program will respond with Polo
Otherwise, the program will respons with "Whats your name?
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "Whats your name?".to_string()
    }
}
