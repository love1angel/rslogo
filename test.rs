fn main() {
    let characters = vec!['a', 'b', 'c', 'd', 'e'];
    let mut joined_string = String::new();

    for (index, &character) in characters.iter().enumerate() {
        if index != 0 {
            joined_string.push(' '); // 添加空格分隔符
        }
        joined_string.push(character); // 将字符添加到字符串中
    }

    println!("{}", joined_string);
}

