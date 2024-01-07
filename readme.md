# C Package Index

> Just a simple cli tool to manage packages in c
> (kinda like pacman & pip)

Kinda finished with this <br>
idk if imma make it better than it is before i get it on a server <br>

## Example:

**this will not work if you don't have the `/opt/clibs` folder like I have**

- do `cargo build`
- cd into the `example` folder
- the exe is `../target/debug/cli`
- try to make and run (will not work)
- do `exe -S linkedlist`
- `make` and run

## Todo
- test this properly
- Make code rustier
- Get it on a server
- Maybe upload
- ~~add the frontend (cli) (like pacman)~~
- ~~split code to workspaces~~
- ~~Use TOML to store metadata~~
- ~~Implement version tracking (ooo scary)~~
- ~~Make the Module.install method better (kinda confused by it though less relevant for current progression)~~

## Simple example can be found in the example folder

## The different flags and modes (heavily influenced by pacman)

- ### Sync mode `-S` or `--sync`
	arguments:
	- `package` -  A conditional argument conflicting with `toml`. The packages to install <br>

	flags:
	- `info` - grabs the toml file of the module
	- `toml` - installs all the modules mentioned as dependencies in the `cpi.toml` file
- ### Remove mode `-R` or `--remove`
	arguments:
	- `package` - The packages to remove
- ### Query mode `-Q` or `--query`
	flags:
	- `info` - Takes one or more packages and prints the toml file of each. This has the same output as `-Si` for now...

## How does this work?
### you should have a directory structure created by the cprojmgr

### How to set this up

this shows how to create a simple `linkedlist` module

- create a folder in `/opt` called `clibs` `mkdir /opt/clibs`
- in there you can create a folder for the module `mkdir /opt/clibs/linkedlist`
- now this requires 3 files but i like to have 4. the required are `xx.h`, `xx.so`, `metadata.toml`, and the rest are `xx.c` and `xx.o`. <br> I will leave the implementation to you though
- The `metadata.toml` includes some metadata about the package. Example:
```toml
title = "linkedlist"
version = "0.1.0"
author = "you probably"

dependencies = []

```
- That's it. you can just use the `cpi -S linkedlist` to install it.


### Example for a makefile that compiles and links this code:
```makefile
CC = gcc
LANG = c
SRC = ./src
BUILD = ./build
MAIN_EXEC = example
CFLAGS = -Wall -Iheaders -I${BUILD}/libs/headers -g
LIBS = $(wildcard ${BUILD}/libs/objs/*.so)
SUBDIRS = $(shell find $(SRC) -type d)
FILES = $(wildcard $(addsuffix /*.${LANG},$(SUBDIRS)))
OBJS = $(patsubst ${SRC}/%.${LANG}, ${BUILD}/objs/%.o, ${FILES})
ALL_OBJS = ${LIBS} ${OBJS}

all: ${MAIN_EXEC}

${MAIN_EXEC}: ${ALL_OBJS}
	${CC} ${ALL_OBJS} -o $@

${BUILD}/objs/%.o: ${SRC}/%.c
	${CC} ${CFLAGS} -c $< -o $@
```


## Motivation
- im bored af
- why not?


## why in rust??
*the memes* <br>
(to lazy to implement HashSet) <br>