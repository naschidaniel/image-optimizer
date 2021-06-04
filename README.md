# image-optimizer

A small tool to optimise images for web applications. The tool is written in Rust. 

The following command line arguments are required: {inputfolder} {outputfolder} {sufix} {width} {quality}.


## Development

The following commands optimise all `*.jpg` and `*.png` files in the `./media` folder. In this example, the file `./testdata/media/fly_sm.JPG` is created with the width of `500 px` and the quality parameter of `90`.

```
cargo test
# or
cargo run ./media ./testdata sm 500 90
```
## Testdata

### Original File

![Original](./media/fly.JPG)

### Optimized File

![Converted File](./testdata/test_ok_fly_sm.JPG)

## Build for Production
```
cargo build --release
```

## License

[GPL-3.0](./LICENSE)

Copyright (c) 2019-present, Daniel Naschberger