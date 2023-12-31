# C Package Index

> Just a simple cli tool to manage packages in c
> (kinda like pip)

## Todo
- add the frontend (cli) (like pacman)
- Make code rustier
- Get it on a server
- Maybe upload
- ~~Use TOML to store metadata~~
- ~~Implement version tracking (ooo scary)~~
- ~~Make the Module.install method better (kinda confused by it though less relevant for current progression)~~


## How does this work?
### you should have a directory structure created by the cprojmgr

### in the makefile:
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


## How a module would be stored
```
/module-name
    /0.1.0 # old version number
        module-name.c # old files
        module-name.h
        module-name.so
        metadata.toml
    # Current Version
    module-name.c # C source file
    module-name.h # Header source file
    module-name.so # Shared Object file
    metadata.toml
```
    