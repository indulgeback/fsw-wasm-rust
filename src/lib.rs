use encoding_rs_io::DecodeReaderBytes;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct DFA {
    transitions: HashMap<(usize, char), usize>,
    accept_states: Vec<usize>,
}

impl DFA {
    pub fn new() -> Self {
        DFA {
            transitions: HashMap::new(),
            accept_states: vec![],
        }
    }

    pub fn add_word(&mut self, word: &str) {
        let mut current_state = 0;
        for c in word.chars() {
            let next_state = self.transitions.len();
            if let Some(&existing_state) = self.transitions.get(&(current_state, c)) {
                current_state = existing_state;
            } else {
                self.transitions.insert((current_state, c), next_state);
                current_state = next_state;
            }
        }
        self.accept_states.push(current_state);
    }

    pub fn load_words_from_files(&mut self, file_paths: &[&str]) -> io::Result<()> {
        for &file_path in file_paths {
            let path = Path::new(file_path);
            println!("尝试打开文件: {}", path.display());

            // 尝试以 UTF-8 编码打开文件
            match File::open(&path) {
                Ok(file) => {
                    // 假设文件可能是 GBK 编码，使用 encoding_rs_io 进行解码
                    let reader = DecodeReaderBytes::new(file);
                    let buffered = io::BufReader::new(reader);

                    for line_result in buffered.lines() {
                        match line_result {
                            Ok(line) => {
                                if !line.trim().is_empty() {
                                    self.add_word(&line);
                                }
                            }
                            Err(e) => {
                                eprintln!("读取文件 {} 时出错: {}", file_path, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("打开文件 {} 时出错: {}", file_path, e);
                }
            }
        }
        Ok(())
    }

    pub fn is_sensitive(&self, text: &str) -> bool {
        let mut current_state = 0;
        let mut i = 0;

        // 预处理文本，去除多余的符号
        let processed_text: String = text.chars().filter(|c| c.is_alphanumeric()).collect();

        while i < processed_text.len() {
            if let Some(c) = processed_text.chars().nth(i) {
                if let Some(&next_state) = self.transitions.get(&(current_state, c)) {
                    current_state = next_state;
                    if self.accept_states.contains(&current_state) {
                        return true;
                    }
                } else {
                    current_state = 0;
                }
            }
            i += 1;
        }
        false
    }
}

fn run_dfa(text: &str) -> bool {
    // 打印当前工作目录
    match env::current_dir() {
        Ok(path) => println!("当前工作目录: {}", path.display()),
        Err(e) => eprintln!("获取当前工作目录时出错: {}", e),
    }

    let mut dfa = DFA::new();

    // 定义要加载的文件路径
    let files = [
        "sensitive_words/涉枪涉爆违法信息关键词.txt",
        "sensitive_words/网址.txt",
        "sensitive_words/色情类.txt",
        "sensitive_words/广告.txt",
        "sensitive_words/政治类.txt",
    ];

    // 加载敏感词库
    if let Err(e) = dfa.load_words_from_files(&files) {
        eprintln!("加载敏感词库时出错: {}", e);
        return false;
    }

    dfa.is_sensitive(text)
}

mod tests {
    use super::*;

    #[test]
    fn test_run_dfa() {
        // 调用 run_dfa 函数进行测试
        let result = run_dfa("%%%%%%%%%硝铵炸药&*&*配方%%%%%%%%%");

        // 使用 assert! 或 assert_eq! 来验证结果
        assert!(result, "Expected run_dfa to return true for sensitive text");
    }
}
