# Virtual Printer Farm

An [OctoPrint](https://octoprint.org/) virtual printer CLI management system to help with development with OctoPrint based printers.

## Why Should I Care?

Currently, development with live printers proves to be difficult on a scalable level.
This solution should help provide a better testing environment - making the development 
process much easier.

## What Is This?

This is a CLI that consists of simple endpoints that help you manage your virtual printer farm.

## How Do I Use It?

Currently, you can't.

There are some heavy needed functionality to be implemented in this system before it can be used.

## How Will I Be Able to Use it?

You could either run this locally or create your own server.

## CLI Commands

### create

Creates a printer and generates a UUID and a port location.

You can append an optional port location like so: `--port=3001` into the request.

__Missing Functionality__

Needs to be able to create an OctoPrint instance, some of this is already written - but needs
configurating.

### get --all (TODO)

Returns a json array of printer objects `[{ "id": String, "port": i32 }]`

### get --port

Returns a printer json object `{ "id": String, "port": i32 }`

## Development

Rust with some OctoPrint knowledge can go a long way here.

OctoPrint is a big repo, so it is left out of the current repo for staying minimalistic. Might add it to the Dockerfile for easy setup.

__Steps__

- Install the latest Rust version using [Rustup](https://rustup.rs/)
- Read up on [OctoPrint documentation](https://docs.octoprint.org/en/master/)

Once you have those things installed:

- Run `cargo build`

__You'll also need to set up OctoPrint__

This is a bigger task in itself.

You'll need to follow through this official documentation [here](https://docs.octoprint.org/en/master/development/environment.html) to get OctoPrint onto your system.

__To run the application__

- Run `cargo run -- <commands>`
- To see command list run `cargo run -- --help`
