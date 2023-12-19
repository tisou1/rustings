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

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        // input.iter() 迭代器时，每个元素都是 &(String, Command) 类型的引用
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            // 使用指令来处理command
            // output.push()
            let value = match command {
                Command::Uppercase =>  string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => {
                    // for
                    let mut appended = String::new();
                    for _ in 0..*n {
                        appended.push_str("bar");
                    }
                    string.to_string() + &appended

                    // map + 闭包函数
                    // let appended = (0..*n).map(|_| "bar").collect::<String>();
                    // string.to_string() + &appended
                }
            };

            output.push(value)
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

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
