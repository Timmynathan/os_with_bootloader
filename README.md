# 🧠 About os_with_bootloader

**os_with_bootloader** is a minimalist operating system built entirely from scratch using Rust, with a custom bootloader to initialize the system from bare metal. This project is part of my exploration into low-level systems programming, where I aim to understand what happens under the hood of modern computers — from powering on the hardware to executing high-level logic.

🔧 Features
A handcrafted bootloader to transition from BIOS to 32-bit/64-bit protected mode

A kernel written in Rust with no standard library (#![no_std])

Custom macros, text output management, and cursor control

Low-level memory management and screen writing

Clean modular architecture (e.g., main.rs, writer.rs)

🚀 Purpose
This project serves as both a learning experience and a foundational framework for building more advanced OS features like memory paging, multitasking, and file systems in Rust.


