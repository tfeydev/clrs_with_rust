# 📘 CLRS Algorithm Analysis in Rust & LaTeX
*Bridging Theory and Implementation through Rust and LaTeX*

This project combines **Rust** and **LaTeX** to produce a scientific and reproducible documentation of algorithms from *Introduction to Algorithms (CLRS)*.  
Each algorithm is **implemented in Rust** and automatically transformed into a **LaTeX-based report** including pseudocode, implementation, and analysis.

---

## 🚀 Motivation

The purpose of this project is to unite *theory and practice* by documenting algorithms in a way that is both elegant and formally correct.  
Rust ensures **type safety and performance**, while LaTeX provides **academic precision and readability**.

> **Concept:** Every algorithm is code and documentation at the same time.

---

## ⚙️ Project Structure

```
clrs/
├── algorithms/           # Rust implementations (sorting, graph, dp, etc.)
│   └── src/
│       ├── sorting/
│       │   └── insertion_sort.rs
│       └── graph/ ...
├── latex/                # LaTeX generator and templates
│   ├── preamble.tex
│   ├── macros.tex
│   ├── config/report.yml
│   └── src/generate.rs
├── output/               # Generated PDF reports
└── Cargo.toml
```

---

## 🧩 Usage

```bash
# Generate LaTeX documentation
cargo run -- doc

# Run a single algorithm
cargo run -- insertion_sort
```

After execution, the generated report will appear in:

```
output/CLRS_Analysis_Report.pdf
```

---

## 📄 Example Output

> *“Insertion Sort” shown as pseudocode and Rust implementation — automatically generated from YAML and code.*

[📄 View Example Report (PDF)](output/main.pdf)

---

## 🧠 Planned Extensions

- Automatic chapter generation from `report.yml`
- Integrated runtime and complexity analysis
- Exercise rendering and appendix support

---

## 🛠️ Technology Stack

| Component | Purpose |
|------------|----------|
| **Rust** | Algorithm implementation |
| **Serde + YAML** | Structured configuration |
| **LaTeX** | Academic documentation |
| **PDFLaTeX** | Automated PDF compilation |

---

## 📚 Reference

> *Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, Clifford Stein*  
> **Introduction to Algorithms**, 4th Edition, MIT Press, 2022.

---

© Thorsten Fey — Precision, structure, and theory meet modern systems engineering.