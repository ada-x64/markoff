# MARKOFF

![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)]

## Description

MARKOFF is a game about capturing territory using
[probabilistic cellular automata](https://en.wikipedia.org/wiki/Stochastic_cellular_automaton)
(a kind of [Markov chain](https://en.wikipedia.org/wiki/Markov_chain)). Players
take turns placing "seeds" on the board, which, when activated, create a chain
reaction.

## Gameplay loop

### Game Rules

Players can adjust grid size, amount of seed money, simulation time, and teams
(color, players (by name), and team name).

Also should have a playground where players can reset the grid, initialize with
random distributions, create new seed types, etc. This will be crucial for
development.

### Selection phase

Each player has a set number of points to choose seeds. Each seed is a PCA with
a wake condition. Preview is a live simulation of the cell behavior which
automatically resets.[^1][^2] Cell types have different colors. Empty = black,
active = white, edges are striked out, teams have colors, seeds have borders or
something. Preview shows captured as green, enemy as red. Activation condition
shown in 3x3 grid.

### Placement Phase (Main loop)

After selection, players take turns placing their seeds, once per turn. Once
placed and confirmed, the grid is simulated for a number of steps. Seeds may be
activated during this phase - including enemy seeds. Each activated seed resets
the simulation timer. Once the simulation is over, alive cells are converted
into captured cells, and the next player has a turn.

### Scoring phase

Once all seeds have been placed, whoever has the most territory wins.

## Bevy

This project is made with Bevy. The ECS architecture makes it easy to simulate
large grids in parallel. Would be able to do larger grids with GPGPU integration
(compute shaders writing to a texture, where each pixel is a cell).

[^1]: This requires saving an initial state and loading it back from disk.

[^2]: Would be cool to show the formula as formatted in LaTeX.
