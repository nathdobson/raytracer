#!/bin/bash
seq 0 99 | xargs -P0 -I{} ./request.sh {}