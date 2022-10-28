![image](https://user-images.githubusercontent.com/63039748/164709140-8bb96d45-972e-4ac5-8e0e-ae566e673761.png)

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0--beta-green"> <img src="https://img.shields.io/github/license/dimensionhq/fleet?color=pink"> <img src="https://img.shields.io/tokei/lines/github/dimensionhq/fleet?color=white&label=lines%20of%20code"> <img src="https://img.shields.io/github/languages/top/dimensionhq/fleet?color=%230xfffff">
</p>

<br>


[Fleet](https://fleet.rs) is a blazingly fast build tool for Rust. Compiling with Fleet is up to 5x faster than with `cargo`.

**Note**: Since Fleet is in its beta phase, it might not be completely stable yet. Feel free to open any issues or bug reports at [issues](https://github.com/dimensionhq/fleet/issues/).

<br>

# :zap: Installation

On MacOS & Linux:
```bash
curl -L get.fleet.rs | sh
```
<br>

On Windows:
```powershell
iwr -useb windows.fleet.rs | iex
```

## Building from source
Prerequisites: **Rust**
```powershell
cargo install --git https://github.com/dimensionhq/fleet fleet-rs
```


## How does Fleet work?

Fleet works by optimizing your builds using existing tooling available in the Rust ecosystem, including seamlessly integrating sccache, lld, zld, ramdisks (for those using WSL or HDD's) and more.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/dimensionhq/fleet/tags). 

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE.md](LICENSE) file for details.
