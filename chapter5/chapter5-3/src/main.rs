use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        // 1行読み取って空白なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // メモリへの書き込み
        if tokens[0] == "mem+" {
            add_and_print_memory(&mut memory, prev_result);
            continue;
        } else if tokens[0] == "mem-" {
            add_and_print_memory(&mut memory, -prev_result);
            continue;
        }

        // 式の計算
        let left: f64 = eval_token(tokens[0], memory);
        let right: f64 = eval_token(tokens[2], memory);
        let result = eval_expression(left, tokens[1], right);

        // 結果の表示
        print_output(result);

        prev_result = result;
    }
}

fn print_output(value: f64) {
    println!("  => {}", value);
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
            // 入力が正しいならここには来ない
            unreachable!()
        }
    }
}
