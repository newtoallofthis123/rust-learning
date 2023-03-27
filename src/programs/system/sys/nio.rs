// NoobScience Input and Output

pub fn input() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.to_string();
    let input = input.trim();
    return input.to_string();
}

pub fn conv_int(str_input: String) -> u8 {
    let str_input: u8 = str_input.trim().parse().expect("Unable to parse number");
    return str_input;
}

pub fn conv_float(conv_float_input: String) -> f32 {
    let conv_float_input: f32 = conv_float_input.trim().parse().expect("Unable to parse number");
    return conv_float_input;
}