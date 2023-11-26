# qserve

A super fast, lightweight file-serving program written in rust

```
cargo install qserve
```

**note**: *Qserve is in its early stage. If you encounter any errors, please create an issue.*

# installation

### cargo
```
cargo install qserve 
```

### from source

```
git clone https://github.com/shamxl/qserve
cd qserve
cargo build --release
```

# Usage

**note**: qserve currently don't support setting default path; future updates will include that

```
qserve
```

### setting ip & port
```
qserve --ip <ip address> --port <port>
```
