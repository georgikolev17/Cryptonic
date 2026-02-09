# Cryptonic 🛡️🧠

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-AGPL--3.0-blue?style=for-the-badge)
![Build Status](https://img.shields.io/github/actions/workflow/status/georgikolev17/Cryptonic/rust.yml?style=for-the-badge)

**Cryptonic** is a privacy-preserving machine learning framework built from scratch in Rust. It leverages **Fully Homomorphic Encryption (FHE)** to enable the execution of neural networks on encrypted data, allowing models to make predictions without ever exposing the underlying input.

> *Designed for performance, type safety, and cryptographic rigor.*

---

## 🚀 Key Features

### 🔐 Fully Homomorphic Encryption (FHE)
Unlike traditional ML frameworks, Cryptonic is built to operate on `tfhe::Ciphertext` types natively.
* **Zero-Knowledge Inference:** The server performs inference on encrypted data and yields an encrypted result. The server never sees the input or the output.
* **Operator Overloading:** Implements custom `Add`, `Mul`, and `Sub` traits for encrypted types, allowing the neural network logic to remain identical for both cleartext (`i32`) and ciphertext contexts.

### ⚙️ Custom Tensor Engine
Cryptonic does not rely on heavy external tensor crates like `ndarray`. It features a **bespoke linear algebra engine** written to handle low-level memory management.
* **Memory Layouts:** Supports both `RowMajor` and `ColumnMajor` memory layouts with manual stride calculation.
* **Broadcasting & Slicing:** Implements NumPy-style broadcasting rules and view-based slicing for efficient memory usage.
* **Generic Architecture:** The `Matrix<T>` struct is generic over any type implementing specific arithmetic traits, enabling polymorphic execution.

### 🧩 Modular Neural Network API
* **Dynamic Graph Construction:** Define networks dynamically by adding layers (`DenseLayer`) and linking them via unique IDs.
* **Type Agnosticism:** A single `Nnet<T>` definition works for both training (cleartext) and secure inference (encrypted), reducing code duplication.

---

## 🧠 Architecture Deep Dive

### The Tensor Library (`src/tensor_library`)
At the core of Cryptonic is a custom-built tensor library that manages raw data vectors.
* **Manual Stride Calculation:** To support different memory layouts, strides are calculated manually based on the shape and layout flag.
* **Safety:** Rigorous bounds checking (`check_bounds`) and error handling (`MatrixError`) ensure memory safety without the overhead of a garbage collector.

```rust
// Example: Creating a Matrix with specific memory layout
let mat: Matrix<i32> = Matrix::new(vec![3, 4], Layout::RowMajor);

// The engine automatically handles stride calculation:
// Layout::RowMajor    -> strides: [4, 1]
// Layout::ColumnMajor -> strides: [1, 3]
```

### The Cryptography Layer (`src/cryptography`)
We wrap `tfhe` types in a custom `CipherTextType` struct to enforce strictly typed arithmetic operations within the network.

```rust
pub struct CipherTextType {
    pub CipherTxt: Option<Ciphertext>,
    pub ServerKey: Option<ServerKey>,
    pub Modulus: Option<u64>
}

// Enables "Encrypted + Encrypted" math transparently
impl Add for CipherTextType { ... }
```

---

## 🛠️ Installation & Usage

### Prerequisites
* Rust Toolchain (Stable)
* Cargo

### 1. Clone the Repository
```bash
git clone [https://github.com/georgikolev17/Cryptonic.git](https://github.com/georgikolev17/Cryptonic.git)
cd Cryptonic
```

### 2. Run Tests
Validate the tensor engine and FHE logic:
```bash
cargo test --verbose
```

### 3. Run the Example
Execute a basic neural network forward pass:
```bash
cargo run --release
```

---

## 📊 Example: Defining a Network

```rust
use Cryptonic::neural_network::dense_layer::DenseLayer;
use Cryptonic::neural_network::nnet::Nnet;

fn main() {
    let mut network: Nnet<i32> = Nnet::new();

    // 1. Define Layers
    let dense_layer1 = DenseLayer::new(Some(vec![2]), Some(vec![2]));
    let dense_layer2 = DenseLayer::new(Some(vec![2]), Some(vec![2]));

    // 2. Add to Network
    let id1 = network.add_layer(LayerType::DenseLayer(dense_layer1), vec![0; 2]);
    let id2 = network.add_layer(LayerType::DenseLayer(dense_layer2), vec![1; 2]);

    // 3. Link Layers (Define the Graph)
    network.add_link(None, Some(id1), Vec::new()); // Input -> Layer 1
    network.add_link(Some(id1), Some(id2), weights); // Layer 1 -> Layer 2
}
```

---

## 📜 License
This project is licensed under the **AGPL-3.0** License.
