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