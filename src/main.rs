use std::io::Write;

// Reference: https://gist.github.com/roachhd/dce54bec8ba55fb17d3a
fn main() {
    // Init-ing memory.
    let mut memory_blocks: Vec<u8> = vec![0; 30000];
    let mut mem_pos = 0;

    // Inputting brainfuck.
    let mut input = String::new();
    if let Err(err) = std::io::stdin().read_line(&mut input) {
        panic!("Cannot input brainfuck, error: {}", err);
    }

    // Interpreting brainfuck.
    let input_chars = input.as_bytes();
    let mut in_pos = 0;

    let mut loop_stack = Vec::new();

    while in_pos < input.len() {
        let c = input_chars[in_pos] as char;

        match c {
            '>' => {
                mem_pos += 1;
                if mem_pos >= memory_blocks.len() {
                    memory_blocks.push(0);
                }
            }
            '<' => {
                if mem_pos != 0 {
                    mem_pos -= 1;
                }
            }
            '+' => {
                let value: &mut u8 = &mut memory_blocks[mem_pos];
                *value = value.wrapping_add(1);
            }
            '-' => {
                let value: &mut u8 = &mut memory_blocks[mem_pos];
                *value = value.wrapping_sub(1);
            }
            '[' => {
                loop_stack.push(in_pos);
            }
            ']' => {
                if let Some(pos) = loop_stack.last() {
                    if memory_blocks[mem_pos] != 0 {
                        in_pos = *pos;
                    } else {
                        loop_stack.pop();
                    }
                } else {
                    panic!("Unclosed loop!");
                }
            }
            '.' => {
                print!("{}", memory_blocks[mem_pos] as char);
            }
            _ => {}
        }

        in_pos += 1;
    }

    // Finish.
    std::io::stdout().flush().unwrap();
}
