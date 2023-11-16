<h1 align="center">QRrs</h1>
<p align="center">CLI QR code generator and reader written in rust</p>

<div align="center">

![CI](https://github.com/Lenivaya/qrrs/workflows/CI/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![codecov](https://codecov.io/gh/Lenivaya/qrrs/branch/master/graph/badge.svg?token=UBGW1EV2GV)](https://codecov.io/gh/Lenivaya/qrrs)
<a href="https://crates.io/crates/qrrs"><img src="https://img.shields.io/crates/v/qrrs.svg?colorB=319e8c" alt="Version info"></a><br>

![image](https://github.com/Lenivaya/qrrs/assets/49302467/d83217a5-0b11-4171-8d5c-1c4bb09b4339)

</div>

# Info

<!-- <img src="(https://github.com/Lenivaya/qrrs/assets/49302467/d83217a5-0b11-4171-8d5c-1c4bb09b4339.png" alt="emacs screenshot" align="right" width="400px"> -->

QRrs is a simple, cross-platform, command-line utility written in rust for working with qr codes (what actually follows from the name).

## Usage

### Quickly generate qr code

```console
qrrs "Your input here"
```

### Generate code, than read it

```console
qrrs "Something" /tmp/qr.png
```

```console
$ qrrs --read /tmp/qr.png
Something
```

### Print generated code to term

```console
qrrs -t "Something" /tmp/qr.png
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
qrrs --read --terminal /tmp/qr.png
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
qrrs --read --terminal /tmp/qr.png /tmp/qr1.png
```

Almost the same result will be without terminal flag, but now instead of QrCode printed in terminal we will see text from it.

```console
qrrs --read /tmp/qr.png /tmp/qr1.png
```

### Create code using pipeline

Use "**-**" to signalize passing data via stdin.

```console
$ echo "something" | qrrs - /tmp/something.png
$ qrrs -r /tmp/something.png
something
```

### Invert colors

![image](https://github.com/Lenivaya/qrrs/assets/49302467/186e2501-8ef1-4728-9567-5b2013911ec0)

### Create code specifying it's margin

![image](https://github.com/Lenivaya/qrrs/assets/49302467/b98a7df7-171a-4707-b733-e095fa7da814)
![image](https://github.com/Lenivaya/qrrs/assets/49302467/6be8fee9-a4a5-4855-b515-8649b68d8028)

## Install

## Nix

```console
nix run github:Lenivaya/qrrs "your input"
```

## NetBSD

```console
pkgin install qrrs
```

### From crates.io

```console
cargo install qrrs
```

### From github

```console
cargo install --git https://github.com/Lenivaya/qrrs.git
```

### [Precompiled binaries](https://github.com/Lenivaya/qrrs/releases)
