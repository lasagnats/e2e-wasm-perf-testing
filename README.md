# E2E WASM performance testing

This repository contains a collection of WASM client side apps with corresponding Backend solutions written in the same programming language.
There is also a Puppeteer performance testing script provided, which is intended for E2E performance testing of these solution pairs.

The server-side app is a simple API with a single `/data/{count} `endpoint that generates the required number of entries in the format:
```
[{
    ID: 1,
    Label: "A Random String"
},
{
    ID: 2,
    Label: "Another Random String"
},
]
```

This repository contains solutions written in:
- C#/Blazor

## Pre-requisites

Please, ensure that node & npm is installed on your machine.
The detailed description of how to set up each solution can be found in the respective README.md.
Run `npm ci` before executing any tests.

## Running performance tests

In order to execute performance tests, start the chosen client-side & server-side solution (instructions can be found in the README.md of each solution).
The run `npm run perf-test` from the root folder.