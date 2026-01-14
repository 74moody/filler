#!/bin/bash
set -e

# Build the docker image (engine + environment)
docker build -t filler .

# Run the container, build bot INSIDE it, then run the game
docker run --rm -it \
  -v "$(pwd)/solution":/filler/solution \
  filler bash -lc "
    cd /filler/solution &&
    cargo build --release &&
    cd /filler &&
    chmod +x linux_robots/wall_e &&
    ./linux_game_engine -f maps/map00 \
      -p1 /filler/solution/target/release/solution \
      -p2 /filler/linux_robots/wall_e
  "
