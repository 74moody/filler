# 🧠 Imperator Bot — Rust Edition

<p align="center">
  <img src="/extra/imperator-filler.png" alt="Imperator" width="350">
</p>



> A high-performance, deterministic **Filler** bot written in **Rust** for the **01-Edu curriculum**.

This Imperator bot plays the Filler game by analyzing the game state in real time and placing pieces strategically to maximize territory while restricting the opponent’s options.

---

## ✨ Highlights

- 🚀 **Fast & deterministic** — no randomness, no timeouts  
- 🦀 **Rust-powered** — memory-safe, panic-free, efficient  
- 🐳 **Docker-ready** — runs exactly as expected in the provided environment  
- 🤖 **Competitive** — consistently defeats standard bots  
- 📐 **Rule-correct** — strictly follows Filler placement rules  

---

## 📌 Overview

The Imperator bot is an autonomous program that:

1. Reads the game state from **stdin**
2. Parses the Anfield and the current piece
3. Computes the best valid placement
4. Outputs coordinates to **stdout**

The design focuses on:
- correctness
- stability
- predictable behavior
- audit friendliness

---

## 📁 Project Structure

```
solution/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── parser.rs
    ├── game.rs
    ├── solver.rs
    ├── scoring.rs
    ├── types.rs
    └── utils.rs
```

---

## 🛠️ Building (Correct & Safe)

⚠️ **Do NOT build on the host system**  
Host builds may cause **GLIBC incompatibility**.

Always build **inside Docker**.

```bash
docker build -t filler .

docker run -it --rm \
  -v "$(pwd)/solution":/filler/solution \
  filler
```

Inside the container:

```bash
cd /filler/solution
cargo build --release
```

The executable will be generated at:

```
/filler/solution/target/release/solution
```

---

## ▶️ Running the Game

```bash
./linux_game_engine -f maps/map00 \
  -p1 /filler/solution/target/release/solution \
  -p2 /filler/linux_robots/wall_e
```

---

## 🧠 Strategy

Deterministic heuristic placement prioritizing expansion toward the opponent while preventing future moves.

---

## 🏆 Expected Performance

The Imperators consistently wins **at least 4 out of 5 games** against:
- `linux_robots/wall_e`
- `linux_robots/h2_d2`
- `linux_robots/bender`

---

## 🔍 Code Quality Guarantees

- No panics / no unwrap
- No infinite loops
- No blocking I/O
- Deterministic output
- Clean compilation
- Docker-safe execution

---

## 📜 License

MIT License

Copyright (c) 2026 ![mohani](https://learn.reboot01.com/git/mohani)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

