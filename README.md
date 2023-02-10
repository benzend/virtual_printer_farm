# Virtual Printer Farm

An OctoPrint based virtual printer farm to help with development with OctoPrint based printers

## Why Should I Care?

Currently, development with live printers proves to be difficult on a scalable level.
This solution should help provide a better testing environment - making the development 
process much easier.

## What Is This?

This is an API that consists of simple endpoints that help you manage your virtual printer farm.

## How Do I Use It?

There are currently some heavy needed functionality to be implemented in this system.

One main thing that I do not have familiarity with is being able to create a printer using
python subprocess to target OctoPrint and create a virtual printer while also having it run.

## How Will I Be Able to Use it?

You could either run this locally or create your own server.

## API Endpoints

### POST /printers

Creates a printer and generates a UUID and a port location.

You can append an optional port location like so: `{ "port": 3000 }` into the body of the request.

__Missing Functionality__

Needs to be able to create an OctoPrint instance, some of this is already written - but needs
configurating.

### GET /printers

Returns an array of printer objects `[{ "id": str(uuid), "port": int }]`

### GET /printers/:id

Returns a printer object `{ "id": str(uuid), "port": int }`

### Development

Python and FastAPI with some OctoPrint knowledge can go a long way here.

Pipenv is also recommended as it is what's used here.

OctoPrint is a big repo, so it is left out of the current repo for minimalism.

__Steps__

- Install Python 3.10 (I currently use [pyenv](https://github.com/pyenv/pyenv) to manage my python versions)
- Install [Pipenv](https://pipenv.pypa.io/en/latest/)
- Read up on [OctoPrint documentation](https://docs.octoprint.org/en/master/)

Once you have those things installed:

- Run `pipenv shell`

This should put you in the environment with all the other needed dependencies like FastAPI and yaaml.

__You'll also need to set up OctoPrint__

This is a bigger task in itself.

You'll need to follow through this official documentation [here](https://docs.octoprint.org/en/master/development/environment.html) to get OctoPrint onto your system.

__To run the application__

- Run `uvicorn main:app --reload`

`uvicorn main:app` targets the `main.py` file and runs it.
`--reload` watches for changes in the file
