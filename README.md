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

```text
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
