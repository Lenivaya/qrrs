<h1 align="center">QRrs</h1>

<p align="center">CLI QR code generator and reader written in rust</p>

[![Build Status](https://travis-ci.com/Lenivaya/qrrs.svg?branch=master)](https://travis-ci.com/Lenivaya/qrrs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

# Info

<img src="https://user-images.githubusercontent.com/49302467/88573963-3abf6300-d04a-11ea-9eb3-ae440a9dd76b.png" width="40%" height="70%" align="right" margin="5%">

QRrs is a command-line utility written in rust for working with qr codes (what actually follows from the name).

# Usage

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
██████████████  ████  ████  ██████████████
██          ██    ██        ██          ██
██  ██████  ██    ██        ██  ██████  ██
██  ██████  ██  ██          ██  ██████  ██
██  ██████  ██  ████    ██  ██  ██████  ██
██          ██  ██    ██    ██          ██
██████████████  ██  ██  ██  ██████████████
                ██████████
██      ██  ████████  ██  ██████████    ██
        ██      ██  ██████      ████  ██
████      ████  ██    ██    ██  ██  ████
████  ██    ██  ████    ██    ██    ████
██  ██    ██████        ████    ██    ████
                ██████  ██  ██  ████    ██
██████████████  ██  ██  ██████  ██    ██
██          ██    ██  ████    ████    ██
██  ██████  ██  ██    ██  ████  ██    ██
██  ██████  ██      ██████      ██████████
██  ██████  ██      ████    ██  ██
██          ██    ████    ████████
██████████████  ██████  ████████  ██    ██
```

```console
$ qrrs --read --terminal /tmp/qr.png >> code.txt
```

## Install

```console
$ cargo install qrrs
```
