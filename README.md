# imt

Simple genetic algorithm to explore the [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem).
This demonstration evolves a population to generate a target string.

### Implementation details

- [Roulette wheel selection](https://en.wikipedia.org/wiki/Fitness_proportionate_selection) is used to optimize the mating pool.
- [One-point crossover](https://en.wikipedia.org/wiki/Crossover_%28genetic_algorithm%29#One-point_crossover) is used for crossover.

## Running

    cargo run --release

## Example output

    $ cargo run --release
    0.11, (01QNRPI2b)
    0.22, (HaEfQY1jP)
    0.33, (Ta969p19O)
    ...
    0.78, (TargDt1B3)
    0.78, (TKrget1F3)
    1.00, (Target123)
    Iterations: 1179
