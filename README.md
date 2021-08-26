<h1 align="center">QRrs</h1>

<p align="center">CLI QR code generator and reader written in rust</p>

![CI](https://github.com/Lenivaya/qrrs/workflows/CI/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
<a href="https://crates.io/crates/qrrs"><img src="https://img.shields.io/crates/v/qrrs.svg?colorB=319e8c" alt="Version info"></a><br>

# Info

<img src="https://user-images.githubusercontent.com/49302467/114319615-d9c5b580-9b1a-11eb-9562-27c220a40881.png" alt="emacs screenshot" align="right" width="400px">

QRrs is a simple, cross-platform, command-line utility written in rust for working with qr codes (what actually follows from the name).

## Usage

### Quickly generate qr code

```console
$ qrrs "Your input here"
```

### Generate code, than read it

```console
$ qrrs "Something" /tmp/qr.png
```

```console
$ qrrs --read /tmp/qr.png
Something
```

### Print generated code to term

```console
$ qrrs -t "Something" /tmp/qr.png
```

```rich-text-format

█████████████████████████████
█████████████████████████████
████ ▄▄▄▄▄ █▄ █▄▄█ ▄▄▄▄▄ ████
████ █   █ █▀▄████ █   █ ████
████ █▄▄▄█ █ ▄█▀▄█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█ ▀ ▀ █▄▄▄▄▄▄▄████
████▄███ █▄▄ ▄▀ ▀▄▄▄  █▀▄████
████▄ ▀█▀▄▄▀▄▀▀▄█▀▄█ █▄ ▀████
████▄█▄██▄▄▄▀▀▀█ ▄▀█ ▀█▄ ████
████ ▄▄▄▄▄ █▄▀▄▀ ▄▄▀ ██ █████
████ █   █ █▄█▀ ▀▄▄█ ▀▀ ▀████
████ █▄▄▄█ ██▀ ▄█▀ ▀ ████████
████▄▄▄▄▄▄▄█▄▄▄█▄▄▄▄█▄██▄████
█████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀

```

### Show code as text in terminal

```console
$ qrrs --read --terminal /tmp/qr.png
```

```rich-text-format

█████████████████████████████
█████████████████████████████
████ ▄▄▄▄▄ █▄ █▄▄█ ▄▄▄▄▄ ████
████ █   █ █▀▄████ █   █ ████
████ █▄▄▄█ █ ▄█▀▄█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█ ▀ ▀ █▄▄▄▄▄▄▄████
████▄███ █▄▄ ▄▀ ▀▄▄▄  █▀▄████
████▄ ▀█▀▄▄▀▄▀▀▄█▀▄█ █▄ ▀████
████▄█▄██▄▄▄▀▀▀█ ▄▀█ ▀█▄ ████
████ ▄▄▄▄▄ █▄▀▄▀ ▄▄▀ ██ █████
████ █   █ █▄█▀ ▀▄▄█ ▀▀ ▀████
████ █▄▄▄█ ██▀ ▄█▀ ▀ ████████
████▄▄▄▄▄▄▄█▄▄▄█▄▄▄▄█▄██▄████
█████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀

```

### Save it into another file

```console
$ qrrs --read --terminal /tmp/qr.png /tmp/qr1.png
```

Almost the same result will be without terminal flag, but now instead of QrCode printed in terminal we will see text from it.

```console
$ qrrs --read /tmp/qr.png /tmp/qr1.png
```

## Install

### From crates.io

```console
$ cargo install qrrs
```

### From github

```console
$ cargo install --git https://github.com/Lenivaya/qrrs.git
```

### [Precompiled binaries](https://github.com/Lenivaya/qrrs/releases)
