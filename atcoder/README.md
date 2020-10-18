# AtCoder

AtCoderの問題の回答置き場

## 実行方法

```
cargo run < input.txt
```

## サンプル

### 入力

```
// 1行に1つの文字列
let s = {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
};

// 1行に1つの数値
let n: usize = {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().parse().unwrap()
};

// 1行に複数の数値（個別）
let (n, q) = {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut ws = line.trim_end().split_whitespace();
    let n: usize = ws.next().unwrap().parse().unwrap();
    let q: usize = ws.next().unwrap().parse().unwrap();
    (n, q)
};

// 1行に複数の数値（Vec）
let a: Vec<usize> = {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().split_whitespace().map(|v| v.parse().unwrap()).collect()
};
```