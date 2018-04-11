# yarclient
## Introduction
    `yar_client` is a crate what can transport info to Yar Server what PHP protocol tool, About Yar see: <https://github.com/laruence/yar>

[![Build Status](https://travis-ci.org/hanskorg/yar-client-rust.svg?branch=master)](https://travis-ci.org/hanskorg/yar-client-rust)

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
yar_client = "0.1.0"
```
## Examples
```rust
 use yar_client::*;
 let mut client = Builder::default()
 .set_url("http://10limi.com/rpc.php").unwrap()
 .set_opt(YAR_OPT_PACKAGER, "JSON").unwrap()
 .set_opt(YAR_OPT_CONNECT_TIMEOUT, 10).unwrap()
 .set_opt(YAR_OPT_TIMEOUT, 30).unwrap()
 .set_token("token")
 .set_provider("org.hansk.net.yarclient")
 .build().unwrap();
 let ret = client.call("test", vec!["1".to_string(), "2".to_string()]).unwrap();
 ```
## Builder
### yar_client::Builder::set_url(&str) -> Builder;
  Set up Yar server address
  ```$rust
   let client = Builder::default().set_url("http://10limi.com/rpc.php")?
```
### yar_client::Builder::set_opt(&str) -> Builder;
  use `yar_client::Build` build a new client,need call `*yar_client::Builder::set_url()*`,
  and can call `*set_opt()*` set up some options, this is the list of options:
 - [x] YAR_OPT_PACKAGER :JSON ��MsgPack��PHP , the msg body encoding method.
    **current version only can use JSON**
 - [ ] YAR_OPT_PERSISTENT , curl lib support keep-alive
 - [x] YAR_OPT_TIMEOUT unit second , transport timeout
 - [x] YAR_OPT_CONNECT_TIMEOUT unit second ,TCP connect timeout
 ```rust
    let client = Builder::default().set_opt(..., ...)?
```
### yar_client::Builder::set_token(&str) -> Builder;
    More About token see php yar manual, default value is `yar_client_rust`
```rust
    let client = Builder::default().set_token("...")
```
### yar_client::Builder::set_provider(&str) -> Builder;
    More About token see php yar manual, default value is `yar_client_rust`
```rust
let client = Builder::default().set_provider("...")?
```


