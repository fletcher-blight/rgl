# rgl

Safe Rust GL System Level Abstraction

The goal of this projection is to provide a safe rust-experience to the standard OpenGL API.
You will find automatic error handling on all functions (opt-out). This will provide function
specific errors to provide a better understanding of what went wrong, and exactly where.

The embedded documentation is adapted straight from
[Khronos documentation](https://registry.khronos.org/OpenGL-Refpages/gl4/).
With minimal modifications as possible.

This library will not provide any abstractions to help use OpenGL it terms of clean/Drop
management, or helper utilities. Just the full catalog of the OpenGL API and nothing more.
