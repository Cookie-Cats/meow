# meow

> Sweet cats always meow~ back.
>
> ​                                                    ---沃兹基 • 硕德

用 Rust 实现的高性能 HTTP 服务器，用于返回访问者 IP。

## 用法

```bash
./meow --help         
Sweet cats always meow~ back.

Usage: meow [OPTIONS]

Options:
  -l, --listen <0.0.0.0:8080>  
  -h, --help                   Print help
  -V, --version                Print version
 
# 在 127.0.0.1 的 8080 端口开放 HTTP 服务器
./meow -l 127.0.0.1:8080

# 如果不加-l参数，或输入格式错误，则默认监听 0.0.0.0:8080
./meow

# 返回值
curl --raw http://localhost:8080
127.0.0.1\n
```

**警告⚠️：如无法获取发送者 IP，则会返回 0.0.0.0**
