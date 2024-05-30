# Rust Ray Tracer

A ray tracer implemented in Rust, based on Peter Shirley's "Ray Tracing in One Weekend" C++ series. It features depth of field and a basic material system.

## Features

- **Basic Ray Tracing**: Renders scenes with spheres and planes.
- **Materials**: Supports diffuse, metal, and dielectric materials.
- **Depth of Field**: Simulates camera blur using a defocus disk.

### Installation and Usage

1. **Clone the repository**:

    ```sh
    git clone https://github.com/yourusername/rust-ray-tracer.git
    cd rust-ray-tracer
    ```

2. **Build the project**:

    ```sh
    cargo build --release
    ```

3. Render an image and save it to `output.ppm`:

    ```sh
    cargo run --release > output.ppm
    ```
Then open the output file with your favorite image editor. You can use [an online PPM viewer](https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html) if that doesn't work.

This below image took 5m15s to be rendered on a Ryzen 5600 @ 3.7Ghz 6C12T, using Rayon for multi-threading:

![Spheres](https://github.com/artogahr/ray-tracer/blob/main/output1.png)

The same image took 38m46s to render using a single thread, and no performance optimizations.

#### Next Steps

* Performance Profiling, identify chokepoints - Done
* CPU Multithreading - Done 
* GPU Multithreading - Probably won't do, not realistically feasible for this

Based on book version:
<br>[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)<br>
Version 4.0.0-alpha.2, 2024-04-07
