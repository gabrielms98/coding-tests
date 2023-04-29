#[warn(dead_code)]
fn matching_parens() {
    let input = "(]";
    let mut arr = vec![];

    let mut is_false = false;

    input.chars().for_each(|c| match c {
        '(' | '[' | '{' => arr.push(c),
        ')' => {
            if arr.pop() != Some('(') {
                is_false = true;
                return;
            }
        }
        ']' => {
            if arr.pop() != Some('[') {
                is_false = true;
                return;
            }
        }
        '}' => {
            if arr.pop() != Some('{') {
                is_false = true;
                return;
            }
        }
        _ => {}
    });

    if is_false || !arr.is_empty() {
        println!("false");
    } else {
        println!("true");
    }
}

fn main() {
    let s = "   -123 tst";
    println!(
        "{}",
        s.trim().split(' ').collect::<Vec<&str>>()[0]
            .trim()
            .parse::<i32>()
            .unwrap()
    );
}
