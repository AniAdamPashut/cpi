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

### example makefile:
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