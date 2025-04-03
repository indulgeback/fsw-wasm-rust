use fsw_wasm_rust::{add_sensitive_word, create_dfa, DFA};

#[test]
fn test_political_words() {
    let mut dfa = create_dfa();

    // 添加测试所需的政治敏感词
    let test_words = [
        "江泽民",
        "习近平",
        "胡锦涛",
        "温家宝",
        "法轮功",
        "中共",
        "薄熙来",
        "李洪志",
        "政府",
        "邓小平",
    ];

    for word in &test_words {
        dfa.add_word(word);
    }

    // 测试检测效果
    assert!(dfa.is_sensitive("江泽民同志视察"));
    assert!(dfa.is_sensitive("法轮功组织"));
    assert!(dfa.is_sensitive("习近平主席讲话"));
    assert!(dfa.is_sensitive("中共领导人"));
    assert!(dfa.is_sensitive("薄熙来案件"));
    assert!(!dfa.is_sensitive("正常的政治新闻"));
}

#[test]
fn test_skip_matching() {
    let mut dfa = DFA::new();
    dfa.add_word("李洪志");

    // 测试跳词匹配
    assert!(dfa.is_sensitive("李洪志")); // 正常匹配
    assert!(dfa.is_sensitive("李x洪x志")); // 字符间有干扰
    assert!(dfa.is_sensitive("xxxx李xx洪x志xxxx")); // 带前缀后缀的干扰
    assert!(dfa.is_sensitive("李.....洪.........志")); // 多个干扰字符
    assert!(!dfa.is_sensitive("李洪")); // 不完整词汇
    assert!(!dfa.is_sensitive("洪志")); // 不完整词汇
}

#[test]
fn test_create_dfa() {
    let dfa = create_dfa();
    assert_eq!(dfa.transitions.len(), 0);
    assert_eq!(dfa.accept_states.len(), 0);
    assert_eq!(dfa.original_words.len(), 0);
}

#[test]
fn test_add_sensitive_word() {
    let mut dfa = create_dfa();

    // 测试添加一个敏感词
    add_sensitive_word(&mut dfa, "测试");
    assert!(dfa.is_sensitive("这是一个测试"));
    assert_eq!(dfa.original_words.len(), 1);

    // 测试添加多个敏感词
    add_sensitive_word(&mut dfa, "敏感词");
    add_sensitive_word(&mut dfa, "关键字");
    assert!(dfa.is_sensitive("这是一个敏感词测试"));
    assert!(dfa.is_sensitive("这是一个关键字"));
    assert_eq!(dfa.original_words.len(), 3);
}

#[test]
fn test_edge_cases() {
    // 空DFA
    let dfa = create_dfa();
    assert!(!dfa.is_sensitive("任何文本都不应匹配"));

    // 空文本
    let mut dfa = create_dfa();
    add_sensitive_word(&mut dfa, "测试");
    assert!(!dfa.is_sensitive(""));

    // 空敏感词
    let mut dfa = create_dfa();
    add_sensitive_word(&mut dfa, "");
    assert!(!dfa.is_sensitive("任何文本都不应匹配"));

    // 非常短的敏感词
    let mut dfa = create_dfa();
    add_sensitive_word(&mut dfa, "a");
    assert!(dfa.is_sensitive("abc"));

    // 中文和特殊字符
    let mut dfa = create_dfa();
    add_sensitive_word(&mut dfa, "特殊！@#￥");
    assert!(dfa.is_sensitive("包含特殊！@#￥字符"));
}

#[test]
fn test_large_word_list() {
    let mut dfa = create_dfa();

    // 添加100个不同的敏感词
    for i in 0..100 {
        add_sensitive_word(&mut dfa, &format!("敏感词{}", i));
    }

    // 确认能正确匹配
    assert!(dfa.is_sensitive("这里包含敏感词42"));
    assert!(dfa.is_sensitive("这里包含敏感词99"));
    assert!(!dfa.is_sensitive("这里只有普通文本"));
    assert!(!dfa.is_sensitive("敏感词")); // 只有词前缀不应匹配

    // 检查性能
    let long_text = "这是一个很长的文本，".repeat(1000);
    let result = dfa.is_sensitive(&long_text);
    assert!(!result);
}

// 本地环境特定测试
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_add_sensitive_words_native() {
    let mut dfa = create_dfa();

    // 直接使用字符串列表
    let words = vec!["词汇1", "词汇2", "词汇3"];

    for word in &words {
        dfa.add_word(word);
    }

    assert!(dfa.is_sensitive("这里有词汇1"));
    assert!(dfa.is_sensitive("这里有词汇2"));
    assert!(dfa.is_sensitive("这里有词汇3"));
    assert_eq!(dfa.original_words.len(), 3);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_run_dfa_with_words_native() {
    let mut dfa = create_dfa();
    let words = vec!["敏感", "测试"];

    for word in &words {
        dfa.add_word(word);
    }

    assert!(dfa.is_sensitive("这是一个敏感的测试"));
    assert!(!dfa.is_sensitive("这是一个普通文本"));
}
