# 初始化

- cargo generate tyr-rust-bootcamp/template
- update master -> main
- update cliff.toml postprocessors.replace = https://github.com/kindywu/03-simple-redis
- pre-commit install
- update `Cargo.toml` name = "macro"
- git remote add origin https://github.com/kindywu/04-macro.git
  git branch -M main
  git push -u origin main

# 查看宏运行

- cargo install cargo-expand
- cargo expand --example enum_macro
