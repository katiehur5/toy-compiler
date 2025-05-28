# A Rust Toy Compiler

## Overview
This project implements the code generation and optimization phases of a toy compiler for a minimalist Toy language. The focus is on translating IR (intermediate representation) to low-level machine code while applying basic optimizations to improve performance and eliminate redundancy.
The generator emits x86-64 assembly and integrates seamlessly with prior frontend stages, including parsing and semantic analysis. 

## Features
Peephole and local optimization techniques include:
- constant folding
- constant propagation
- dead code elimination

## Reflection
Writing the codegen backend demystified compiler construction for me. I developed an appreciation for how high-level code becomes hardware instructions, and how even small optimizations can significantly impact runtime behavior.

This project demanded a thorough understanding of the AST (abstract syntax tree) structure. Good news: I had that. Bad news: Katie, meet Rust.
