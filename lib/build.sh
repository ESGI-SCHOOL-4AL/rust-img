#!/bin/bash

gcc -c -o ppma_io.o ppma_io.c
ar rcs libppma_io.a ppma_io.o