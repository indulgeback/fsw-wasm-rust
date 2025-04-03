// use encoding_rs_io::DecodeReaderBytes;
// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// DFA（确定有限自动机）结构体，用于敏感词检测
#[wasm_bindgen]
pub struct DFA {
    #[wasm_bindgen(skip)]
    pub transitions: HashMap<(usize, char), usize>, // 状态转移表
    #[wasm_bindgen(skip)]
    pub accept_states: Vec<usize>, // 接受状态集合
    #[wasm_bindgen(skip)]
    pub original_words: Vec<String>, // 存储原始添加的敏感词，用于跳词匹配
}

#[wasm_bindgen]
impl DFA {
    /// 创建一个新的DFA实例
    ///
    /// # 返回值
    ///
    /// 返回一个空的DFA实例
    pub fn new() -> Self {
        DFA {
            transitions: HashMap::new(),
            accept_states: vec![],
            original_words: vec![],
        }
    }

    /// 向DFA中添加一个敏感词
    ///
    /// # 参数
    ///
    /// * `word` - 要添加的敏感词
    pub fn add_word(&mut self, word: &str) {
        // 添加到原始词列表
        self.original_words.push(word.to_string());

        // 构建DFA转移表
        let mut current_state = 0;
        for c in word.chars() {
            let next_state = self.transitions.len() + 1; // 从1开始，避免与初始状态0混淆
            if let Some(&existing_state) = self.transitions.get(&(current_state, c)) {
                current_state = existing_state;
            } else {
                self.transitions.insert((current_state, c), next_state);
                current_state = next_state;
            }
        }
        self.accept_states.push(current_state);
    }

    /// 检测文本是否包含敏感词
    ///
    /// # 参数
    ///
    /// * `text` - 要检测的文本
    ///
    /// # 返回值
    ///
    /// 如果文本包含敏感词，返回true；否则返回false
    pub fn is_sensitive(&self, text: &str) -> bool {
        // 先尝试连续匹配 (更快)
        if self.is_continuous_match(text) {
            return true;
        }

        // 如果连续匹配失败，尝试跳词匹配
        self.is_skip_match(text)
    }

    /// 检测文本是否包含连续的敏感词（不跳词）
    ///
    /// # 参数
    ///
    /// * `text` - 要检测的文本
    ///
    /// # 返回值
    ///
    /// 如果文本包含连续的敏感词，返回true；否则返回false
    fn is_continuous_match(&self, text: &str) -> bool {
        let processed_text = text;
        let mut current_state = 0;
        let mut i = 0;

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

    /// 检测文本是否包含跳词的敏感词
    ///
    /// # 参数
    ///
    /// * `text` - 要检测的文本
    ///
    /// # 返回值
    ///
    /// 如果文本包含跳词的敏感词，返回true；否则返回false
    fn is_skip_match(&self, text: &str) -> bool {
        // 使用原始敏感词列表而不是重建
        let text_chars: Vec<char> = text.chars().collect();

        for word in &self.original_words {
            let word_chars: Vec<char> = word.chars().collect();
            if self.check_skip_match(&text_chars, &word_chars) {
                return true;
            }
        }

        false
    }

    /// 检查文本中是否以跳词方式包含指定敏感词
    ///
    /// # 参数
    ///
    /// * `text` - 要检测的文本字符数组
    /// * `word` - 敏感词字符数组
    ///
    /// # 返回值
    ///
    /// 如果文本以跳词方式包含敏感词，返回true；否则返回false
    fn check_skip_match(&self, text: &[char], word: &[char]) -> bool {
        if word.is_empty() {
            return false;
        }

        // 先找到敏感词的第一个字符
        let first_char = word[0];
        let mut possible_starts = Vec::new();

        // 找出文本中所有可能的起始位置
        for (i, &c) in text.iter().enumerate() {
            if c == first_char {
                possible_starts.push(i);
            }
        }

        // 从每个可能的起始位置开始检查
        for &start in &possible_starts {
            let mut text_pos = start;
            let mut word_pos = 0;

            while text_pos < text.len() && word_pos < word.len() {
                if text[text_pos] == word[word_pos] {
                    word_pos += 1;
                    if word_pos == word.len() {
                        return true; // 成功匹配整个敏感词
                    }
                }
                text_pos += 1;
            }
        }

        false // 未能从任何位置匹配完整敏感词
    }
}

/// 创建一个新的DFA实例
///
/// # 返回值
///
/// 返回一个新的DFA实例
#[wasm_bindgen]
pub fn create_dfa() -> DFA {
    DFA::new()
}

/// 使用指定的敏感词列表对文本进行敏感词检测
///
/// # 参数
///
/// * `text` - 要检测的文本
/// * `words` - 敏感词列表
///
/// # 返回值
///
/// 如果文本包含敏感词，返回true；否则返回false
#[wasm_bindgen]
pub fn run_dfa_with_words(text: &str, words: Box<[JsValue]>) -> bool {
    let mut dfa = DFA::new();

    for word_js in words.iter() {
        if let Some(word) = word_js.as_string() {
            dfa.add_word(&word);
        }
    }

    dfa.is_sensitive(text)
}

/// 向DFA实例中添加一个敏感词
///
/// # 参数
///
/// * `dfa_ptr` - DFA实例引用
/// * `word` - 要添加的敏感词
#[wasm_bindgen]
pub fn add_sensitive_word(dfa_ptr: &mut DFA, word: &str) {
    dfa_ptr.add_word(word);
}

/// 批量添加敏感词到DFA实例
///
/// # 参数
///
/// * `dfa_ptr` - 要添加词汇的DFA实例引用
/// * `words` - 敏感词列表
///
/// # 返回值
///
/// 如果所有词都成功添加，返回true；否则返回false
#[wasm_bindgen]
pub fn add_sensitive_words(dfa_ptr: &mut DFA, words: Box<[JsValue]>) -> bool {
    let mut success = true;

    for word_js in words.iter() {
        if let Some(word) = word_js.as_string() {
            dfa_ptr.add_word(&word);
        } else {
            success = false;
        }
    }

    success
}
