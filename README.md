# Devy
![License](https://img.shields.io/badge/license-The%20Unlicense-blue)
![Version](https://img.shields.io/badge/Version:%200.1.0-BETA-red)

A grouping of CLI devtools

## Available Tools
- Base64 encode/decode
- Password/Unique string creation
- API runner that parses a Yaml file to build requests

## Main Commands
### Commands:
|Command|Description|
|-------|-----------|
|base64 |    Base64 encoding and decoding.|
|password |  Generate a password, default is 32 characters with all character types|
|api |       Make API request(s) from a yaml file|
|help |      Print this message or the help of the given subcommand(s)|

### Options:
|Option|Description|
|------|-----------|
|-h, --help|     Print help|
|-V, --version|  Print version|

## Base64 Command
Base64 encoding and decoding.

`Usage: devy base64 [OPTIONS] <STRING>`

Arguments:
-  `<STRING>`  The string to encode or decode

|Option|Description|
|------|-----------|
|  -d, --decode|  base64 decode the given value|
|  -e, --encode|  base64 encode the given value|
|  -h, --help|    Print help|

## Password Command
Generate a password, default is 32 characters with all character types

`Usage: devy password [OPTIONS]`

|Option|Description|
|------|-----------|
|  -l, --lower|            Include lowercase letters|
|  -u, --upper|            Include uppercase letters|
|  -n, --numbers|          Include numbers|
|  -s, --special|          Include special characters|
|      --length `<LENGTH>`|  The length of the password [default: 32]|
|  -h, --help|             Print help|

## Api Command
Make API request(s) from a yaml file

`Usage: devy api --file <FILE>`

|Option|Description|
|------|-----------|
|      --file `<FILE>`|  Path to Yaml file to use for sending requests|
|  -h, --help|         Print help|


### Yaml file example
```yml
- title: "the request title to print"
  data:
    method: GET
    url: https://example.com
      "Accept": "application/json"
    }
- title: "another request title"
  data:
    method: POST
    url: https://example.com
    headers: {
      "Accept": "application/json",
      "Content-Type": "application/json"
    }
    post_body: '{
      "id": 1234,
      "name": "Test Post"
    }'
```

