//! # Horde
//!
//! Suite of binary analysis tools for reverse engineering made in Rust with Python bindings.
//!
//! Horde is a comprehensive suite of binary analysis tools designed for reverse engineering, developed in Rust with Python bindings.
//! It simplifies tasks like analyzing, modifying, and generating binary programs through various advanced techniques.
//! Key features include binary loading (ELF, PE, Mach-O), lifting to intermediate language (IL), disassembly, emulation, JIT compilation,
//! program instrumentation, taint analysis, control-flow analysis, and symbolic/concolic execution. Inspired by projects like [angr](https://github.com/angr/angr)
//! and [miasm](https://github.com/cea-sec/miasm/), Horde aims to streamline complex reverse engineering workflows for a wide range of architectures.
//!
//! ## Quick Links:
//!
//! * [Features](crate::_docs::features) (to-do)
//!
//! ## Tutorials
//!
//! * [Tutorial 1 - Basic Usage](crate::_docs::examples) (to-do)
//!
#![doc = include_str!("../ROADMAP.md")]

pub mod _docs;

pub mod loader;
