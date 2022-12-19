# 差分進化 Differential evolution

差分進化を実装した。

すべての関数 "y = f(x)" の 最小値に近い"x" or 最大値に近い"x" を求められるはず。

## 利用方法 Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ys_differential_evolution = "0.5"
```

## 実行例 Example

    $ git clone https://github.com/YoshizawaShogo/differential_evolution.git
    $ cd differential_evolution
    $ git checkout v0.5.0
    $ cargo run --example knapsack --release

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.