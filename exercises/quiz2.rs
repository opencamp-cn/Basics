// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod my_module {
    use super::Command;

    // 完整的函数签名应该是一个包含(String, Command)元组的向量作为输入。
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for (string, command) in input.iter() {
            // 创建一个新字符串来存储结果
            let mut result = String::from(string);
            match command {
                Command::Uppercase => output.push(result.to_uppercase()),
                Command::Trim => output.push(result.trim().to_string()),
                Command::Append(count) => {
                    for _ in 0..*count {
                        result.push_str("bar");
                    }
                    output.push(result.clone());
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::my_module::transformer; // 注意这里需要导入具体的函数

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}