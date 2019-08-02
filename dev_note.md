## Get errors when install diesel_cli
``` shell
cargo install diesel_cli  // error, because has not install the client library for a database backend like postgres, sqlite on my operation system
cargo install diesel_cli --no-default-features --features mysql  
```