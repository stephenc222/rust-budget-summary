pub fn strip_comma(mut num_str: String) -> String {
    let comma_pos = num_str.find(',').unwrap_or(0);
    if comma_pos != 0 {
        num_str.remove(comma_pos);
    }
    return num_str;
}
