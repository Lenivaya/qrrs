<p align="center">
    <img src="https://user-images.githubusercontent.com/49302467/111488823-47600b00-8742-11eb-8fe4-125657091599.png" width="35%" height="65%" align="center" margin="3em">
</p>

<h1 align="center">QRrs</h1>

<p align="center">CLI QR code generator and reader written in rust</p>

![CI](https://github.com/Lenivaya/qrrs/workflows/CI/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
<a href="https://crates.io/crates/qrrs"><img src="https://img.shields.io/crates/v/qrrs.svg?colorB=319e8c" alt="Version info"></a><br>

# Info

QRrs is a simple, cross-platform, command-line utility written in rust for working with qr codes (what actually follows from the name).

## Usage

### Generate code, than read it

```console
$ qrrs "Something" /tmp/qr.png
```

```console
$ qrrs --read /tmp/qr.png
Something
```

### Show code as text in terminal, or save it in a file

```console
$ qrrs --read --terminal /tmp/qr.png
```

```

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

```console
$ qrrs --read --terminal /tmp/qr.png >> code.txt
```

## More advanced commands

Using read and terminal flags together with output specified, as a result shows the QrCode in terminal and copies it to new file.

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
