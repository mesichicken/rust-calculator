use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        if tokens[0] == "mem+" {
            add_and_print_memory(&mut memory, prev_result);
            continue;
        } else if tokens[0] == "mem-" {
            add_and_print_memory(&mut memory, -prev_result);
            continue;
        }

        let left: f64 = eval_token(tokens[0], memory);

        let right: f64 = eval_token(tokens[2], memory);

        let result = eval_expression(left, tokens[1], right);

        print_output(result);
        prev_result = result;
    }
}

fn add_and_print_memory(memory: &mut f64, prev_result: f64) {
    *memory += prev_result;
    print_output(*memory);
}

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
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
