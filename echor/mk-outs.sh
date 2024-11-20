#!/bin/env bash

# Define a variable for the output directory
OUTDIR="tests/expected"

# Test if the output dir doesn't exist and create it if needed
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# Tests
echo "Hello there" >$OUTDIR/hello1.txt
echo "Hello" "there" >$OUTDIR/hello2.txt
echo -n "Hello  there" >$OUTDIR/hello1.n.txt
echo -n "Hello" "there" >$OUTDIR/hello2.n.txt
