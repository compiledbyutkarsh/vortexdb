# 🚀 VortexDB

A high-performance, async, distributed key-value database built from scratch in Rust.

> ⚡ Built for learning systems programming: databases, concurrency, networking, and storage engines.

---

## ✨ Features

- ⚡ Async TCP server (Tokio-based)
- 🧠 In-memory sharded MemTable for high concurrency
- 📝 Write-Ahead Logging (WAL) for crash durability
- 💾 SSTable-style segment storage (append-only design)
- 🔁 Background compaction engine
- 🌐 Basic replication framework (async fanout)
- 📊 Pipelined benchmark client
- 🔄 Automatic recovery on startup

---

## 🏗️ Architecture

Client → TCP Server → Database Engine  
│  
├── WAL (durability layer)  
├── MemTable (in-memory fast path)  
├── Segment Files (persistent storage layer)  
└── Replication Layer (async distributed sync)

---

## ⚙️ How to Run

### Clone repo
```
git clone https://github.com/compiledbyutkarsh/vortexdb.git  
cd vortexdb
```

### Build
```
cargo build --release
```

### Run server
```
./target/release/vortexdb
```

### Run replica (optional)
```
./target/release/replica  
```
### Run benchmark
```
./target/release/benchmark
``` 

---

## 📊 Performance

- ~360,000 requests/sec (pipelined mode)
- Low-latency async processing
- Batched network requests for throughput

---

## 📝 Example Usage

Connect using any raw TCP client like `nc` or `telnet`:

```bash
nc 127.0.0.1 8080
```

### Interaction Format

```
SET name utkarsh
OK

GET name
utkarsh
```
---

## 🧠 Concepts Used

- Rust systems programming
- Async runtime (Tokio)
- Custom TCP protocol
- Memory + disk hybrid storage
- Concurrency + sharding
- Basic distributed systems
- Performance benchmarking

---

## 📌 Inspiration

- RocksDB
- LevelDB
- FoundationDB (conceptual ideas)

---

## 🔮 Future Improvements

- Raft consensus
- Full LSM-tree compaction
- Compression + Bloom filters
- Query planner layer# 🚀 VortexDB

A high-performance, async, distributed key-value database built from scratch in Rust.

> ⚡ Built for learning systems programming: databases, concurrency, networking, and storage engines.


