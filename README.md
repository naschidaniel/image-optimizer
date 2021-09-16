![[release]](https://github.com/naschidaniel/image-optimizer/actions/workflows/release.yml/badge.svg) 
![[rsaudit]](https://github.com/naschidaniel/image-optimizer/actions/workflows/rsaudit.yml/badge.svg?name=rsaudit) 
![[rstest]](https://github.com/naschidaniel/image-optimizer/actions/workflows/rstest.yml/badge.svg?name=rstest)

# image-optimizer

A small tool to optimise images JPEG and PNG images for web applications. The images are scaled in keeping with the aspect ratio. In addition a copy in WebP format and a thumbnail with a aspect ration 1:1 can be created.
The tool is written in Rust. 

## Get the latest release of the image-optimizer

The required binary for the platform can be downloaded from [Releases: Main](https://github.com/naschidaniel/image-optimizer/releases/tag/main).

```
# Linux 
curl -L https://github.com/naschidaniel/image-optimizer/releases/download/main/image-optimizer-linux --output image-optimizer && chmod +x image-optimizer

# Windows
curl -L https://github.com/naschidaniel/image-optimizer/releases/download/main/image-optimizer.exe --output image-optimizer.exe

# MacOs
curl -L https://github.com/naschidaniel/image-optimizer/releases/download/main/image-optimizer-macos --output image-optimizer && chmod +x image-optimizer
```

The following command line arguments are required: {inputfolder} {outputfolder} {sufix} {width} {quality} {webpimage} {thumbnail}.

```
# Optimize images, create copy in WebP format and create a thumbnails of the images
./image-optimizer ./media ./testdata sm 500 90 true true

# Optimize images and create thumbnails of the images
./image-optimizer ./media ./testdata sm 500 90 false true

# Optimize images only
./image-optimizer ./media ./testdata sm 500 90 false false
```

The command `./image-optimizer ./media ./testdata sm 500 90 true true` will optimise all `*.jpg` and `*.png` files in the `./media` folder. In the folder `./testdata`, the optimized images will be created with a width of `500 px` and the quality of `90` in JPEG, PNG and [WebP](https://developers.google.com/speed/webp) Format. A square image section with a width of `500px`, height `500` and the postfix `thumbnail` will be generated. This image can be used as a preview for an image gallery.
## Testdata

### Original File

- Width: 4000 px
- Hight: 3000 px
- Size: 2,3 MB

![Original](./media/paradise/fly.JPG)

### Optimized Files

#### Converted JPG File
- Width: 500 px
- Hight: 375 px
- Size: 75,6 kb

![Converted JPG File](./testdata/test_ok_fly_sm.JPG)

#### Converted WebP File
- Width: 500 px
- Hight: 375 px
- Size: 54,8 kb

![Converted WebP File](./testdata/test_ok_fly_sm.webp)

#### Converted Thumbnail in WebP Format
- Width: 500 px
- Hight: 500 px
- Size: 63,5 kb

![Converted WebP File](./testdata/test_ok_fly_sm_thumbnail.webp)
## Development

```
cargo test
# or
cargo run ./media ./testdata sm 500 90 true true
```

## Build

```
cargo build --release --all-features
```


## How To use the image-optimizer in Production

An example of automatic image optimisiation for continuous integration and production with image-optimizer can be found in [https://github.com/naschidaniel/fly-tirol](https://github.com/naschidaniel/fly-tirol).

## Changelog

* 2021-09-16 A square image section with the postfix thumbnail can be generated.
* 2021-09-09 A copy of the resized file is saved in [WebP](https://developers.google.com/speed/webp) format.
* 2021-06-07 Init Repository

## License

[GPL-3.0](./LICENSE)

Copyright (c) 2021-present, Daniel Naschberger