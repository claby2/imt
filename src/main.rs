#![allow(clippy::cast_precision_loss)]

use rand::{
    distributions::{Alphanumeric, Distribution, WeightedIndex},
    rngs::ThreadRng,
    seq::SliceRandom,
    Rng,
};
use std::{
    fmt::{self, Display, Formatter},
    iter,
};

const TARGET: &str = "Target123";

trait Dna: Clone {
    fn new(rng: &mut ThreadRng) -> Self;
    fn fitness(&self) -> f64;
    fn crossover(&self, other: &Self) -> Self;
    fn mutate(&mut self, rng: &mut ThreadRng, mutation_rate: f64);
}

#[derive(Debug, Clone)]
struct DnaString<const LENGTH: usize>(String);

impl<const LENGTH: usize> Display for DnaString<LENGTH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const LENGTH: usize> Dna for DnaString<LENGTH> {
    fn new(rng: &mut ThreadRng) -> Self {
        DnaString(
            iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(LENGTH)
                .collect(),
        )
    }

    // Calculate fitness as ratio between correct and incorrect characters.
    fn fitness(&self) -> f64 {
        let mut correct: usize = 0;
        for (a, b) in self.0.chars().zip(TARGET.chars()) {
            if a == b {
                correct += 1;
            }
        }
        correct as f64 / LENGTH as f64
    }

    // One-point crossover implementation.
    fn crossover(&self, other: &Self) -> Self {
        let half = LENGTH / 2;
        Self(format!("{}{}", &self.0[..half], &other.0[half..LENGTH]))
    }

    fn mutate(&mut self, rng: &mut ThreadRng, mutation_rate: f64) {
        self.0 = self
            .0
            .chars()
            .map(|c| {
                if rng.gen_bool(mutation_rate) {
                    char::from(rng.sample(Alphanumeric))
                } else {
                    c
                }
            })
            .collect();
    }
}

#[derive(Debug)]
struct Population<T: Dna, const SIZE: usize> {
    population: Vec<T>,
    mating_pool: Vec<T>,
    mutation_rate: f64,
    rng: ThreadRng,
}

impl<T: Dna, const SIZE: usize> Population<T, SIZE> {
    fn new(mutation_rate: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            population: iter::repeat(())
                .map(|()| T::new(&mut rng))
                .take(SIZE)
                .collect(),
            mating_pool: Vec::with_capacity(SIZE),
            mutation_rate,
            rng,
        }
    }

    // Create mating pool based on roulette wheel selection.
    fn select(&mut self) {
        let fitnesses: Vec<f64> = self.population.iter().map(|d| d.fitness()).collect();
        let total_fitness: f64 = fitnesses.iter().sum();
        let weights: Vec<f64> = if total_fitness == 0.0 {
            // All elements in the population have a fitness of 0.0.
            // Make weights equal to uniform distribution.
            vec![1.0 / SIZE as f64; SIZE]
        } else {
            fitnesses.iter().map(|f| f / total_fitness).collect()
        };
        let dist = WeightedIndex::new(&weights).unwrap();
        // Create mating pool from weighted distribution.
        self.mating_pool = iter::repeat(())
            .map(|()| self.population[dist.sample(&mut self.rng)].clone())
            .take(SIZE)
            .collect();
    }

    // Generate new population from mating pool.
    fn generate(&mut self) {
        for i in 0..SIZE {
            // Choose two random elements from the mating pool.
            let pair: Vec<&T> = self.mating_pool.choose_multiple(&mut self.rng, 2).collect();
            let pair: (&T, &T) = (pair[0], pair[1]);
            let mut d = pair.0.crossover(pair.1);
            d.mutate(&mut self.rng, self.mutation_rate);
            self.population[i] = d;
        }
    }

    // Return the element with the highest fitness.
    fn best(&self) -> T {
        let best_fitness: f64 = 0.0;
        let mut best: Option<usize> = None;
        for i in 0..SIZE {
            if self.population[i].fitness() >= best_fitness {
                best = Some(i);
            }
        }
        self.population[best.unwrap()].clone()
    }
}

fn main() {
    let mut population = Population::<DnaString<{ TARGET.len() }>, 100>::new(0.01);
    let mut iterations: usize = 0;
    loop {
        iterations += 1;
        population.select();
        population.generate();
        let best = population.best();
        println!("{:.2}, ({})", best.fitness(), best);
        if (best.fitness() - 1.0).abs() < f64::EPSILON {
            println!("Iterations: {}", iterations);
            break;
        }
    }
}
