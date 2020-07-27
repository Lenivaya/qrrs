<h1 align="center">QRrs</h1>

<p align="center">CLI QR code generator and reader written in rust</p>

[![Build Status](https://travis-ci.com/Lenivaya/qrrs.svg?branch=master)](https://travis-ci.com/Lenivaya/qrrs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

## Info

<img src="https://user-images.githubusercontent.com/49302467/88573963-3abf6300-d04a-11ea-9eb3-ae440a9dd76b.png" width="40%" height="70%" align="right">

QRrs is a command-line utility written in rust for working with qr codes (what actually follows from the name).

## Usage

Example of usage:

    $ qrrs "Something" /tmp/qr.png
    $ qrrs --read /tmp/qr.png
    Something
    $ qrrs --read --terminal /tmp/qr.png

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

## Install

    $ cargo install qrrs
