# yarclient
## Introduction
    Yar Client for Rust-Lang

[![Build Status](https://travis-ci.org/hanskorg/snowflake-rust.svg?branch=master)](https://travis-ci.org/hanskorg/snowflake-rust)

# Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
yar_client = "0.1.0"
```

```rust
        let mut client = Builder::default()
         .set_url("http://10limi.com/rpc.php").unwrap()
         .set_opt(YAR_OPT_PACKAGER, "JSON").unwrap()
         .set_opt(YAR_OPT_CONNECT_TIMEOUT, 10).unwrap()
         .set_opt(YAR_OPT_TIMEOUT, 30).unwrap()
         .set_token("token")
         .set_provider("org.hansk.net.yarclient")
         .build().unwrap();
        let a = client.call("test", vec!["1".to_string(), "2".to_string()]);
```