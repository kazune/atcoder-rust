# atcoder-rust

atcoderを解きつつ、rustを練習します。

## プロジェクトの作り方（abc123の場合）

```sh
cargo new abc123
cd abc123
mkdir src/bin
```

## A問題を解くときの現実的な流れ

最初は次の流れにするとやりやすいです。

1. プロジェクトを作る
2. `src/bin/a.rs` を作る
3. 問題文のサンプルをテストに写す
4. cargo test --bin a を実行
5. 失敗する
6. solve を実装
7. テストが通るまで直す
8. 最後に提出

この流れなら、かなり安定します。
