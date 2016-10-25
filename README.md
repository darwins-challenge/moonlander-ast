Fly Me To The Moon GP Implementation
====================================

This crate contains the modeled Genetic Programming implementation of the
Moonlander Problem.

This crate comes with 4 challenge scenarios, specified in 4 TOML files, which
can be "solved" by adapting the model in the crate so that Genetic Programming
succesfully evaluates a solution to the scenario.

Organization
------------

The generic library code is in the crate itself (in the `src` directory),
and you _will_ need to touch this to solve the scenario's.

The code can be exercised by small wrapper programs in the `examples` directory,
and you may need to add new programs to exercise your newly written code. These
example programs will write JSON traces of what they're doing to `stdout`. That
output can be visualized using the `moonlander-visualization` crate.

To run an example, run a command like this:

    cargo run --release --example EXAMPLE SCENARIOFILE > TRACEFILE

For example:

    cargo run --release --example evolve_condition 1_fixed_vertical_landing.toml > output.json

This compiles the example and runs it on the given scenario, writing the output
to a file. The example program will run indefinitely. Feel free to cancel it
with `Ctrl-C` when you observe that it has succeeded in evolving the right
solution.

Important Files
---------------

These are the files you will mostly need to be touching (though feel free
to modify everything and anything):

    *.toml                      Scenarios, also containing evolution parameters.
    src/fitness.rs              Fitness function.
    src/grammar/*               All types of AST nodes.
    src/sim/sensor_ data.rs     You won't necessarily need to touch this file,
                                but it's good for reference of values you can
                                use in your fitness function.

Scenarios
---------

    1_fixed_vertical_landing.toml

The lander starts at a fixed height, without any rotation, and needs to
succesfully land. To solve this scenario, it suffices to evaluate a `Condition`
program.

If the `Condition` evaluates to `true`, the lander will use its thrusters (the
command will be `Thrust`). If it doesn't, it won't (the command will be `Skip`).

Use the program `evolve_condition`, which will try to evolve a program of type
`Condition`.

    2_random_vertical_landing.toml

The previous scenario evolved a program that started at a fixed position.
However, such a winning program might be overfitting to the problem. In this
scenario, the lander starts at a random height.

Does your model still evolve a successful solution?

    3_fixed_rotated_landing.toml

In the previous 2 scenarios, the lander always started upright. In this
scenario, it will start at angle.

Using the `Condition`, we could evaluate one of two commands: `Thrust` or
`Skip`. Once the lander also needs to correct its attitude, those two
commands are no longer sufficient (check: can `evolve_condition` evolve
a winning solution to this scenario?)

Instead, we'll need a new type of AST node, to increase the range of
programs that we can express.

Can you invent and implement such an AST node? (Don't forget to make a new
example to evolve it, and don't forget to update the fitness function)

    4_random_rotated_landing.toml

In the previous scenario, the starting rotation was fixed. What if the
starting rotation is randomized? Can we evolve a solution that generalizes?

Prerequisites
-------------

This repository is designed to be checked out _next_ to the `moonlander-gp`
crate, since it has a hard path reference to that crate in `Cargo.toml`.

It goes best together with `moonlander-visualization`, to visualize the output
of training runs.
