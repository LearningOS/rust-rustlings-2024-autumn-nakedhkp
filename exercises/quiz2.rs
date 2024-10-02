pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 定义 transformer 函数
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];  // 初始化输出向量
        for (string, command) in input.iter() {
            // 根据不同的命令执行不同的操作
            let result = match command {
                Command::Uppercase => string.to_uppercase(),  // 转换为大写
                Command::Trim => string.trim().to_string(),   // 去除前后空格
                Command::Append(times) => {
                    let mut result = string.clone();
                    for _ in 0..*times {
                        result.push_str("bar");  // 附加 "bar"
                    }
                    result
                }
            };
            output.push(result);  // 将结果加入输出向量
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 导入 my_module 中的 transformer 函数
    use super::my_module::transformer;
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
