![[release]](https://github.com/naschidaniel/image-optimizer/actions/workflows/release.yml/badge.svg) 
![[rsaudit]](https://github.com/naschidaniel/image-optimizer/actions/workflows/rsaudit.yml/badge.svg?name=rsaudit) 
![[rstest]](https://github.com/naschidaniel/image-optimizer/actions/workflows/rstest.yml/badge.svg?name=rstest)

# image-optimizer

A small tool to optimise images for web applications. The tool is written in Rust. 

## Get the latest release of the image-optimizer

The required binary for the platform can be downloaded from [Releases: Main](https://github.com/naschidaniel/image-optimizer/releases/tag/main).

```
# Linux 
curl -L https://github.com/naschidaniel/image-optimizer/releases/download/main/image-optimizer-linux --output image-optimizer && chmod +x image-optimizer

# Windows
curl -L https://github.com/naschidaniel/image-optimizer/releases/download/main/image-optimizer.exe

# MacOs
curl -L https://github.com/naschidaniel/image-optimizer/releases/download/main/image-optimizer-macos --output image-optimizer && chmod +x image-optimizer
```

The following command line arguments are required: {inputfolder} {outputfolder} {sufix} {width} {quality}.

```
./image-optimizer ./media ./testdata sm 500 90
```

The command has optimised all `*.jpg` and `*.png` files in the `./media` folder. In this example, the file `./testdata/media/fly_sm.JPG` was created with the width of `500 px` and the quality of `90`.


## Development

```
cargo test
# or
cargo run ./media ./testdata sm 500 90
```
## Testdata

### Original File

- Width: 4000 px
- Hight: 3000 px
- Size: 2,3 MB

![Original](./media/fly.JPG)

### Optimized File

- Width: 500 px
- Hight: 375 px
- Size: 75,6 kb

![Converted File](./testdata/test_ok_fly_sm.JPG)


## License

[GPL-3.0](./LICENSE)

Copyright (c) 2021-present, Daniel Naschberger