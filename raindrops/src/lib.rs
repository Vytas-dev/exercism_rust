pub fn raindrops(n: u32) -> String {
    let is_factor = |factor| n % factor == 0;
    let mut result = String::new();

    if is_factor(3) { result += "Pling"; }
    if is_factor(5) { result += "Plang"; }
    if is_factor(7) { result += "Plong"; }

    if result.is_empty() { result = n.to_string(); }

    result
}
