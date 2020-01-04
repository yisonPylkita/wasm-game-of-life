# Game of Life - WASM + Rust edition

## Rules
- Game is ran in ticks. Each tick represents full world update
- World starts with seed as first tick
- Every cell interacts with eight neighbours
- Any cell with less than two neighbours dies - underpopulation
- Any cell with two or three neighbours lives to next generation
- Any cell with more than three neighbours dies - overpopulation
- Any dead cell with three neighbours becomes a live cell - birth
- Every cell death or birth takes place at the same time and efect of change appears in next tick

## Consequences of rules
- Every tick is a pure function that accepts state and returns next state
- Infinite world is imposible to simulate so I'll be using 1024 x 1024 world
- Since I'll be using world with fixed size I won't need dynamic memory allocation. This alone allows for good optimization size everything can be kept on stack
- Memory needed to represent one tick will be 1024 bytes x 1024 bytes -> 1 MB. Assuming next tick will be kept in separate memory space I expect 2 MB of data space needed + code size
- Since every tick transition is a pure function writing e2e tests is quite easy

