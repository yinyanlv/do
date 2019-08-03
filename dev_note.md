### win10安装diesel_cli报错
``` shell
# 报错，因为操作系统上没安装用于postgres、sqlite后台的客户端库
cargo install diesel_cli  
# 成功
cargo install diesel_cli --no-default-features --features mysql  
```