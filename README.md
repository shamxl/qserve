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

### setting chunk size

```
qserve --chunks <number of chunks>
```

# FAQs 

**Q**: *What is chunk size ?*

**A**: *Setting chunk size allows you to control the amount of data read in each iteration, balancing between memory efficiency and I/O performance. If you increase the chunk size, you will read larger portions of the file in each iteration, potentially reducing the number of I/O operations but using more memory.*
