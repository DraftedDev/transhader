# Tran(`shader`)

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/drafteddev/transhader)
[![Crates.io](https://img.shields.io/crates/v/transhader.svg)](https://crates.io/crates/transhader)
[![Downloads](https://img.shields.io/crates/d/transhader.svg)](https://crates.io/crates/transhader)

**Transpile shaders from one language to another using procedural macros.**

## Why?

Some Multi-Backend Graphics APIs (like bgfx) require shaders for every backend (GLSL for OpenGL; HLSL for D3D; MSL for Metal; SPIR-V for Vulkan; WGSL for WebGPU). You can either try to transpile your shaders using `naga` during Runtime (which would affect performance) or transpile each shader using a `build.rs` script.

This tool simply transpiles shaders during compile-time without any hassle.

## How?

Transhader uses [naga](https://github.com/gfx-rs/wgpu/tree/trunk/naga) and Rust's procedural macro system to generate optimal shader code. Configure shader transpiler options using the Rusty-Object-Notation.

## Drawbacks

- Limited shader language support.
- Longer compile-times, since shaders are transpiled using procedural macros.
- Less customization of transpiler options (though I am working on it).

## Features

- **`from-glsl` - Transpile OpenGL Shading Language (GLSL) Shaders.**
- **`from-spv` - Transpile Standard Portable Intermediate Representation for Vulkan (SPIR-V) Shaders.**
- **`from-wgsl` - Transpile Web GPU Shader Language (WGSL) Shaders.**


- **`to-glsl` - Transpile shaders to OpenGL Shading Language (GLSL) Shaders.**
- **`to-msl` - Transpile shaders to Metal Shading Language (MSL) Shaders.**
- **`to-spv` - Transpile shaders to Standard Portable Intermediate Representation for Vulkan (SPIR-V) Shaders.**
- **`to-wgsl` - Transpile shaders to Web GPU Shader Language (WGSL) Shaders.**
- **`to-hlsl` - Transpile shaders to High Level Shader Language (HLSL) Shaders.**

## License

Transhader is dual licensed under the MIT and Apache 2.0 licenses.
