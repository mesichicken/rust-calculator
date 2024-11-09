use std::io::stdin;

fn main() {
    let mut memory = Memory {
        slots: vec![],
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
            memory.add_and_print(tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            memory.add_and_print(tokens[0], -prev_result);
            continue;
        }

        let left: f64 = memory.eval_token(tokens[0]);
        let right: f64 = memory.eval_token(tokens[2]);
        let result = eval_expression(left, tokens[1], right);

        print_output(result);
        prev_result = result;
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
    println!(" => {}", value)
}

struct Memory {
    // メモリの名前と値の組を配列で保存する
    slots: Vec<(String, f64)>,
}

impl Memory {
    fn add_and_print(&mut self, token: &str, prev_result: f64) {
        let slot_name = &token[3..token.len() - 1];
        for slot in self.slots.iter_mut() {
            if slot.0 == slot_name {
                slot.1 += prev_result;
                print_output(slot.1);
                return;
            }
        }
        // メモリが見つからなかったら最後の要素に追加
        self.slots.push((slot_name.to_string(), prev_result));
        print_output(prev_result);
    }

    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name = &token[3..];
            for slot in &self.slots {
                if slot.0 == slot_name {
                    return slot.1;
                }
            }
            0.0
        } else {
            token.parse().unwrap()
        }
    }
}
