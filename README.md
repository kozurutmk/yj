# yj

## Description

yj is a command-line tool to convert YAML to JSON.

## Usage

```bash
echo "hello: world" | yj
{"hello": "world"}
```
```bash
yj < input.yaml > output.json
```

## Build

```bash
cargo build --release
```
