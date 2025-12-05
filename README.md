# Datastructures

[![Rust](https://img.shields.io/badge/Rust-1.91-orange?logo=rust)](https://www.rust-lang.org/)    
[![License](https://img.shields.io/badge/license-SSPL-blue.svg)](LICENSE)

---

**Datastructures** is a lightweight, efficient, and robust collection of  
**high-performance data structures** written in **Rust**.

Designed for both **systems programming** and **application-level development**,  
Datastructures provides the essential building blocks required to create  
fast, predictable, and memory-aware software — with or without the standard library.

It serves as the **structural foundation** for larger projects that demand  
tight memory control, low-level performance, and deterministic behavior.

---

# 🧩 Purpose of Datastructures

Modern applications — especially those involving networking, distributed systems,  
game engines, embedded devices, or zero-allocation environments — often require  
**specialized data structures** beyond what `std` offers.

Datastructures fills this gap by offering:

- deterministic, fixed-capacity containers  
- allocation-free data structures  
- LRU lists, free lists, static buffers  
- heaps, queues, and intrusive lists  
- compatibility with both `std` and `no_std` environments  

These structures form the **infrastructure layer** used by higher-level components  
such as routing tables, schedulers, protocol engines, or ECS systems.

---

# ✨ Key Features

- 🧱 **Essential Low-Level Data Structures**  
  Linked lists, double lists, static buffers, free lists, heaps, and more.

- ⚙️ **Zero-Allocation & Fixed-Capacity Variants**  
  Ideal for embedded, networking, and real-time systems.

- 🧩 **Optional No-Std Support**  
  Datastructures can operate without the Rust standard library by enabling the  
  `no-std` feature.  
  This makes it usable in embedded environments, kernels, WASM, and  
  other constrained systems.

- 🚀 **Performance-Focused**  
  Optimized for O(1) operations where possible  
  (remove, insert, rotation, LRU updates, etc.).

- 🔧 **Modular Workspace Architecture**  
  Extensible and easy to integrate into larger systems.

- 🧪 **Testing-Oriented Design**  
  Predictable, deterministic behavior.

---

# 🧭 Project Status

🚧 **Active Development**

Datastructures is being developed as the structural foundation for  
larger distributed and high-performance systems.

Current focus areas include:

- Static and dynamic linked lists  
- LRU structures for caches and routing tables  
- Binary heaps and priority queues  
- Freelist allocators and slab storage  
- Buffer abstractions and chunked memory blocks  
- No-std compatibility & optimizations  

Contributions and feedback are highly encouraged.

---

# 📦 Installation

During development:

``` toml
[dependencies]
datastructures = { path = "https://github.com/enzoblain/Datastructures" }
```

Once published:

``` toml
[dependencies]
datastructures = "0.1"
```

Enable the optional `no-std` mode:

``` toml
[dependencies]
datastructures = { version = "0.1", features = ["no-std"] }
```

---

# 🤝 Contributing

Contributions are welcome — especially regarding:

- performance improvements  
- additional data structures  
- unsafe optimizations  
- no-std compatibility  
- documentation & examples  

Typical workflow:

``` sh
cargo fmt
cargo clippy
cargo test --workspace
```

See `CONTRIBUTING.md` for details.

---

# 📄 License Philosophy

Datastructures is licensed under the **Server Side Public License (SSPL) v1**.

This license ensures the library remains **open** while preventing  
proprietary forks or commercial services from exploiting the project  
without contributing back.

It protects Datastructures in contexts where structural consistency  
and ecosystem integrity are critical.

---

# 📬 Contact

**Discord:** enzoblain  
**Email:** enzoblain@proton.me  

Open to discussions, improvements, and architecture/design questions.