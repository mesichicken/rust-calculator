use std::io::stdin;

fn main() {
    let mut memory = Memory {
        slots: vec![0.0; 10],
    };
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with('+') {
            add_and_print_memory(&mut memory, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            add_and_print_memory(&mut memory, tokens[0], -prev_result);
            continue;
        }

        let left: f64 = eval_token(tokens[0], &memory);
        let right: f64 = eval_token(tokens[2], &memory);
        let result = eval_expression(left, tokens[1], right);

        print_output(result);
        prev_result = result;
    }
}

fn add_and_print_memory(memory: &mut Memory, token: &str, prev_result: f64) {
    let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
    memory.slots[slot_index] += prev_result;
    print_output(memory.slots[slot_index]);
}

fn eval_token(token: &str, memory: &Memory) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..].parse().unwrap();
        memory.slots[slot_index]
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            unreachable!()
        }
    }
}

fn print_output(value: f64) {
    println!("The value is {}", value)
}

struct Memory {
    slots: Vec<f64>,
}
