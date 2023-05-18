# Introduction
[English Version](README.md)

该项目是根据[《Zero To Production In Rust》](https://zero2prod.com)一书中的示例实现的，该书是学习[Rust](https://www.rust-lang.org/)和[Actix-web](https://actix.rs/)框架的优秀资源。通过实现一个真实的、可投入生产的电子邮件订阅应用程序，本项目旨在帮助读者深入学习相关技术。

最终版在final branch。

# Key features
* actix-web框架
* Postgres数据库
* 单元测试和集成测试
* 健壮的用户身份验证实现
* 使用Docker设置CI CD
* 将app部署到云服务器
* 全面介绍如何确保API的安全性
* 幂等性 - 如何使API具备重试安全性
* 错误处理（Rust的长项）
* 日志（测试用于发现“已知未知”问题，遥测用于发现“未知未知”问题）
* 探索了许多Rust的crate（库）

# Book review
* 这本书不是仅仅告诉你怎么做，而是带着你一步步`提出问题-解决问题-分析不足-重构`得出最终的最优解，让你知道为什么这么做。
* 让我认识到了测试代码的帮助与重要性。
* 详细地展示了写代码的流程是什么样的
* 对 Rust 的学习和后端的学习都很有帮助，看完[Rust book](https://doc.rust-lang.org/book/)之后就可以看这本书进行实战学习

# Basic Usage
## 通过 docker 启动 postgres 和 Redis 
```bash
./scripts/init_db.sh
./scripts/init_redis.sh
```

## How to run
```bash
cargo run
```

## How to test
```bash
cargo test 
```