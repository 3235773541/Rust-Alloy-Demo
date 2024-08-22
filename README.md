## Rust---Loco---Alloy入门学习
Rust、Loco和Alloy的入门学习项目，主要学习Loco框架的使用和web3-rust相关库的使用。

#### 参考文档
https://course.rs/

https://loco.rs/docs/getting-started/guide/

https://alloy.rs/

#### 关于commit：
前十个commit是loco的入门学习，主要由loco命令生成，后面的六commit是Alloy的入门学习，涉及合约调用，链上交易订阅，和交易数据解析。

#### 运行：
1. 安装rust，loco和相关依赖


2. docker 启动数据库依赖
```shell
docker run -d -p 5432:5432 \
  -e POSTGRES_USER=loco \
  -e POSTGRES_DB=myapp_development \
  -e POSTGRES_PASSWORD="loco" \
  postgres:15.3-alpine
```

```
docker run -p 6379:6379 -d redis redis-server
```

3. 配置.env文件, 参考.env.example
⚠️这里必须Alchemy的API_KEY，订阅交易时使用的是 `alchemy_pendingTransactions`



3. 启动loco
```
cargo loco start
```

```
$ curl localhost:5150/guide
hello
```
5. loco的其他命令
```shell
$ curl -X POST -H "Content-Type: application/json" -d '{
  "title": "Your Title",
  "content": "Your Content xxx"
}' localhost:5150/articles
{"created_at":"...","updated_at":"...","id":2,"title":"Your Title","content":"Your Content xxx"}
```

```shell
$ curl localhost:5150/articles
[{"created_at":"...","updated_at":"...","id":1,"title":"how to build apps in 3 steps","content":"use Loco: https://loco.rs"},{"created_at":"...","updated_at":"...","id":2,"title":"Your Title","content":"Your Content xxx"}

```
6. 测试订阅功能

使用`web3-rs`库的封装，订阅交易
```shell
cargo loco task tx_subscribe
```

7. 测试合约调用
使用`alloy-rs`库的封装，调用合约
```shell
cargo loco task Contract
```

8. 测试交易订阅以及数据解析
使用`tokio_tungstenite`库订阅交易，使用`alloy`解析数据
会打印 uniswap的 `UniversalRouter`的 `execute` 交易的信息
```shell
cargo loco task alloy_subscribe
```


#### 遇到的一些问题：
1. web3-rs, ehters-rs 库被标注已经deprecated，目前alloy-rs文档相对全面


#### TODO：
- [ ] 1. 对于input的解析，应该有简洁的直接通过abi解析任意调用函数的方法，而不用对单个函数进行分类
