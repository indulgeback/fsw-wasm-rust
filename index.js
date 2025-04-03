// 导入生成的 JavaScript 包装器
import * as wasm from "./pkg/fsw_wasm_rust.js"
import {
  politicalWords,
  weaponsWords,
  pornWords,
  adsWords,
  websitesWords,
} from "./sensitiveWords.js"

// 全局变量，使其他脚本可以访问
window.wasmModule = null
let dfaInstance = null

// 初始化函数，异步加载 WASM
async function initWasm() {
  try {
    // 加载和初始化 WASM 模块
    await wasm.default()

    // 将 WASM 功能暴露给全局变量
    window.wasmModule = wasm

    console.log("WASM 模块加载成功!")

    // 初始化 UI 和事件监听器
    setupUI()
    setupEventListeners()

    // 在成功加载后恢复按钮状态
    elements.checkButton.innerHTML = '<i class="fas fa-search"></i>检测敏感词'
    elements.checkButton.disabled = false
  } catch (error) {
    console.error("加载 WASM 模块失败:", error)
    document.getElementById(
      "result"
    ).textContent = `加载敏感词检测引擎失败: ${error.message}`
    document.getElementById("result").classList.add("danger")
    document.getElementById("result").classList.remove("hidden")
  }
}

// 缓存DOM元素引用
const elements = {
  // 主要功能区
  textToCheck: document.getElementById("textToCheck"),
  sensitiveWords: document.getElementById("sensitiveWords"),
  result: document.getElementById("result"),
  checkButton: document.getElementById("checkButton"),
  clearButton: document.getElementById("clearButton"),
  highlightedContent: document.getElementById("highlightedContent"),
  highlightedResult: document.getElementById("highlightedResult"),

  // 预设词库
  loadPolitical: document.getElementById("loadPolitical"),
  loadWeapons: document.getElementById("loadWeapons"),
  loadPorn: document.getElementById("loadPorn"),
  loadAds: document.getElementById("loadAds"),
  loadWebsites: document.getElementById("loadWebsites"),

  // 测试区域
  createDfaResult: document.getElementById("createDfaResult"),
  singleWord: document.getElementById("singleWord"),
  addWordResult: document.getElementById("addWordResult"),
  testAddWord: document.getElementById("testAddWord"),
  multipleWords: document.getElementById("multipleWords"),
  addWordsResult: document.getElementById("addWordsResult"),
  testAddWords: document.getElementById("testAddWords"),
  testWords: document.getElementById("testWords"),
  testText: document.getElementById("testText"),
  runDfaResult: document.getElementById("runDfaResult"),
  testRunDfa: document.getElementById("testRunDfa"),
  testCreateDfa: document.getElementById("testCreateDfa"),
}

function setupUI() {
  // 恢复之前保存的敏感词库（如果有）
  const savedWords = localStorage.getItem("sensitiveWords")
  if (savedWords) {
    elements.sensitiveWords.value = savedWords
  }

  // 添加自动保存功能
  elements.sensitiveWords.addEventListener("input", function () {
    localStorage.setItem("sensitiveWords", elements.sensitiveWords.value)
  })

  // 暗黑模式切换
  const themeToggle = document.getElementById("themeToggle")
  const icon = themeToggle.querySelector("i")

  // 检查之前保存的主题设置
  const savedTheme = localStorage.getItem("theme")
  if (savedTheme === "dark") {
    document.documentElement.setAttribute("data-theme", "dark")
    icon.classList.remove("fa-moon")
    icon.classList.add("fa-sun")
  }

  // 添加切换事件
  themeToggle.addEventListener("click", () => {
    const currentTheme = document.documentElement.getAttribute("data-theme")
    if (currentTheme === "dark") {
      document.documentElement.removeAttribute("data-theme")
      localStorage.setItem("theme", "light")
      icon.classList.remove("fa-sun")
      icon.classList.add("fa-moon")
    } else {
      document.documentElement.setAttribute("data-theme", "dark")
      localStorage.setItem("theme", "dark")
      icon.classList.remove("fa-moon")
      icon.classList.add("fa-sun")
    }
  })
}

function setupEventListeners() {
  // 检查按钮
  elements.checkButton.addEventListener("click", function () {
    const text = elements.textToCheck.value
    if (!text) {
      elements.result.textContent = "请输入要检测的文本"
      elements.result.classList.remove("hidden", "success", "danger")
      elements.result.classList.add("warning")
      return
    }

    const wordsText = elements.sensitiveWords.value
    if (!wordsText) {
      elements.result.textContent = "请添加敏感词"
      elements.result.classList.remove("hidden", "success", "danger")
      elements.result.classList.add("warning")
      return
    }

    // 清空现有DFA并添加敏感词
    const words = wordsText
      .split("\n")
      .map((w) => w.trim())
      .filter((w) => w.length > 0)

    if (words.length === 0) {
      elements.result.textContent = "请添加敏感词"
      elements.result.classList.remove("hidden", "success", "danger")
      elements.result.classList.add("warning")
      return
    }

    // 清空现有DFA并添加敏感词
    dfaInstance = wasmModule.create_dfa()
    wasmModule.add_sensitive_words(dfaInstance, words)

    // 检测文本
    const isSensitive = dfaInstance.is_sensitive(text)

    // 显示结果
    elements.result.textContent = isSensitive
      ? "⚠️ 检测到敏感内容"
      : "✅ 未检测到敏感内容"

    elements.result.classList.remove("hidden")
    if (isSensitive) {
      elements.result.classList.add("danger")
      elements.result.classList.remove("success")

      // 高亮显示敏感词
      let highlightedText = text
      words.forEach((word) => {
        if (text.includes(word)) {
          const regex = new RegExp(word, "g")
          highlightedText = highlightedText.replace(
            regex,
            `<span class="highlight">${word}</span>`
          )
        }
      })

      elements.highlightedContent.innerHTML = highlightedText
      elements.highlightedResult.classList.remove("hidden")
    } else {
      elements.result.classList.add("success")
      elements.result.classList.remove("danger")
      elements.highlightedResult.classList.add("hidden")
    }
  })

  // 添加清空按钮功能
  elements.clearButton.addEventListener("click", () => {
    elements.textToCheck.value = ""
    elements.result.classList.add("hidden")
    elements.highlightedResult.classList.add("hidden")
  })

  // 分类标签切换
  document.querySelectorAll(".category-tab").forEach((tab) => {
    tab.addEventListener("click", () => {
      document
        .querySelectorAll(".category-tab")
        .forEach((t) => t.classList.remove("active"))
      tab.classList.add("active")

      // 根据分类筛选词库
      const category = tab.dataset.category
      console.log("选择分类:", category)

      // 可以根据选择的分类自动加载对应词库
      switch (category) {
        case "political":
          elements.sensitiveWords.value = politicalWords.join("\n")
          break
        case "porn":
          elements.sensitiveWords.value = pornWords.join("\n")
          break
        case "weapons":
          elements.sensitiveWords.value = weaponsWords.join("\n")
          break
        case "ads":
          elements.sensitiveWords.value = adsWords.join("\n")
          break
        case "websites":
          elements.sensitiveWords.value = websitesWords.join("\n")
          break
        case "all":
          elements.sensitiveWords.value = allSensitiveWords.join("\n")
          break
      }
    })
  })

  // 加载预设词库
  elements.loadPolitical.addEventListener("click", function () {
    elements.sensitiveWords.value = politicalWords.join("\n")
  })

  elements.loadWeapons.addEventListener("click", function () {
    elements.sensitiveWords.value = weaponsWords.join("\n")
  })

  elements.loadPorn.addEventListener("click", function () {
    elements.sensitiveWords.value = pornWords.join("\n")
  })

  elements.loadAds.addEventListener("click", function () {
    elements.sensitiveWords.value = adsWords.join("\n")
  })

  elements.loadWebsites.addEventListener("click", function () {
    elements.sensitiveWords.value = websitesWords.join("\n")
  })

  // 测试函数 - create_dfa
  elements.testCreateDfa.addEventListener("click", () => {
    try {
      const testDfa = wasmModule.create_dfa()

      elements.createDfaResult.textContent = "✅ DFA创建成功"
      elements.createDfaResult.className = "result success"
      elements.createDfaResult.classList.remove("hidden")
    } catch (error) {
      elements.createDfaResult.textContent = "❌ 错误: " + error.message
      elements.createDfaResult.className = "result danger"
      elements.createDfaResult.classList.remove("hidden")
    }
  })

  // 测试函数 - add_sensitive_word
  elements.testAddWord.addEventListener("click", () => {
    try {
      const testDfa = wasmModule.create_dfa()
      const word = elements.singleWord.value

      if (!word) {
        elements.addWordResult.textContent = "请输入敏感词"
        elements.addWordResult.className = "result warning"
        elements.addWordResult.classList.remove("hidden")
        return
      }

      wasmModule.add_sensitive_word(testDfa, word)

      // 测试添加的词是否生效
      const testText = word
      const isMatch = testDfa.is_sensitive(testText)

      if (isMatch) {
        elements.addWordResult.textContent = `✅ 敏感词 "${word}" 添加成功并正常检测`
        elements.addWordResult.className = "result success"
      } else {
        elements.addWordResult.textContent = `❌ 敏感词添加失败，无法检测`
        elements.addWordResult.className = "result danger"
      }
      elements.addWordResult.classList.remove("hidden")
    } catch (error) {
      elements.addWordResult.textContent = "❌ 错误: " + error.message
      elements.addWordResult.className = "result danger"
      elements.addWordResult.classList.remove("hidden")
    }
  })

  // 测试函数 - add_sensitive_words
  elements.testAddWords.addEventListener("click", () => {
    try {
      const testDfa = wasmModule.create_dfa()
      const wordsText = elements.multipleWords.value

      if (!wordsText.trim()) {
        elements.addWordsResult.textContent = "请输入敏感词"
        elements.addWordsResult.className = "result warning"
        elements.addWordsResult.classList.remove("hidden")
        return
      }

      const words = wordsText
        .split("\n")
        .map((w) => w.trim())
        .filter((w) => w.length > 0)

      const success = wasmModule.add_sensitive_words(testDfa, words)

      // 测试添加的词是否全部生效
      let allDetected = true
      let detectedCount = 0

      for (const word of words) {
        if (testDfa.is_sensitive(word)) {
          detectedCount++
        } else {
          allDetected = false
        }
      }

      if (success && allDetected) {
        elements.addWordsResult.textContent = `✅ 全部 ${words.length} 个敏感词添加成功并正常检测`
        elements.addWordsResult.className = "result success"
      } else {
        elements.addWordsResult.textContent = `⚠️ 添加了 ${detectedCount}/${words.length} 个敏感词，函数返回: ${success}`
        elements.addWordsResult.className = "result warning"
      }
      elements.addWordsResult.classList.remove("hidden")
    } catch (error) {
      elements.addWordsResult.textContent = "❌ 错误: " + error.message
      elements.addWordsResult.className = "result danger"
      elements.addWordsResult.classList.remove("hidden")
    }
  })

  // 测试函数 - run_dfa_with_words
  elements.testRunDfa.addEventListener("click", () => {
    try {
      const wordsText = elements.testWords.value
      const testText = elements.testText.value

      if (!wordsText.trim() || !testText.trim()) {
        elements.runDfaResult.textContent = "请输入敏感词和检测文本"
        elements.runDfaResult.className = "result warning"
        elements.runDfaResult.classList.remove("hidden")
        return
      }

      const words = wordsText
        .split("\n")
        .map((w) => w.trim())
        .filter((w) => w.length > 0)

      const result = wasmModule.run_dfa_with_words(testText, words)

      elements.runDfaResult.textContent = result
        ? `✅ 检测成功: 文本中包含敏感词`
        : `✅ 检测成功: 文本中不包含敏感词`
      elements.runDfaResult.className = result
        ? "result danger"
        : "result success"
      elements.runDfaResult.classList.remove("hidden")
    } catch (error) {
      elements.runDfaResult.textContent = "❌ 错误: " + error.message
      elements.runDfaResult.className = "result danger"
      elements.runDfaResult.classList.remove("hidden")
    }
  })
}

// 启动应用
initWasm()
