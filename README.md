# qserve

A super fast, lightweight file-serving program written in rust

```bash
cargo install qserve
```

**note**: *Qserve is in its early stage. If you encounter any errors, please create an issue.*

# installation

### cargo
```bash
cargo install qserve 
```

### from source

```bash
git clone https://github.com/shamxl/qserve
cd qserve
cargo build --release
```

# Basic usage

```bash
qserve --path <path>
```

# FAQs 

**Q**: *What is chunk size ?*

**A**: *Setting chunk size allows you to control the amount of data read in each iteration, balancing between memory efficiency and I/O performance. If you increase the chunk size, you will read larger portions of the file in each iteration, potentially reducing the number of I/O operations but using more memory.*
