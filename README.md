Fly Me To The Moon Genetic Programming Assignments
==================================================

[![Build Status](https://travis-ci.org/darwins-challenge/moonlander-ast.svg?branch=master)](https://travis-ci.org/darwins-challenge/moonlander-ast), but ignore that :P.

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
    src/sim/sensor_data.rs      You won't necessarily need to touch this file,
                                but it's good for reference of values you can
                                use in your fitness function.

You'll need to know the sensors that are available to the moon lander. To see
what fields are available on the `sensor_data` struct, either look at the source
file (`src/sim/sensor_data.rs`) or the generated API documentation.

Current Grammar
---------------

The starting grammar of the moonlander looks like this:

    Sensor := X | Y | Vx | Vy | O | W | Fuel

    Command := Skip | Left | Right | Thrust

    Expression := Constant(Number)
                | Sensor(Sensor)
                | Plus(Expression, Expression)
                | Minus(Expression, Expression)
                | Multiply(Expression, Expression)
                | Divide(Expression, Expression)

    Condition := True | False

               | Not(Condition)
               | Or(Condition)
               | And(Condition)

               | Less(Expression, Expression)
               | LessEqual(Expression, Expression)
               | Equal(Expression, Expression)
               | GreaterEqual(Expression, Expression)
               | Greater(Expression, Expression)

For the first assignment, producing an AST of type `Condition` (i.e.,
the top-level node is a `Condition`) is sufficient.

For the second assignment, you'll need to extend the grammar with a new
node type.

Scenarios
---------

There are the scenario's you can try for the assignments.

> NOTE: You're free to change the scenario files, but observe that some variables
> have to be floating point. Don't forget to add in the decimal point, or the
> program will panic on reading the input file.


### First assignment, straight landing

This assignment can be solved by taking the existing code, and just changing the
fitness function to correctly incentivize/guide the genetic search. Do this
by combining the various features available on the `SensorData` object, with
weights. You may have to play around a bit with the numbers; look at the
relative scores that some example traces of your program got in the visualizer.

**1_fixed_vertical_landing.toml**

The lander starts at a fixed height, without any rotation, and needs to
land successfully. The only decision the program will need to make to
land correctly is for each frame to decide on this question: _whether or not to
use the thruster_.

To solve this scenario, it suffices to evaluate a `Condition` program. If the
`Condition` evaluates to `true`, the lander will use its thrusters (the command
will be `Thrust`). If it doesn't, it won't (the command will be `Skip`).

Run the example program `evolve_condition`, which will try to evolve a program
of type `Condition`.

**2_random_vertical_landing.toml**

The previous scenario evolved a program that started at a fixed position.
However, such a winning program might be overfitting to the problem. That is,
it may have learned perfectly how to land from a given height, but fail
when it is started at any other height.

Run the program again, but use this scenario. Does your model still evolve a
successful solution? Does the evolved program look any different?

### Second assignment, rotated landing

**3_fixed_rotated_landing.toml**

In the previous 2 scenarios, the lander always started upright. In this
scenario, it will start at angle.

Using the `Condition`, we could evaluate to one of two commands: `Thrust` or
`Skip`. Once the lander also needs to correct its attitude, those two commands
are no longer sufficient You can verify this yourself, just try it: can the
example orgram `evolve_condition` evolve a winning solution to scenario 3?

To solve this problem, you'll need to extend the grammar with a new type of
node, so that the program that gets generated can make more than 2 choices: it
should also be able to answer with `Left` or `Right`.

Can you invent and implement such an AST node? Think of what the form of the
new node should be, then model it.

See the section "_Implementing an AST Node_" for more information on how to
prepare the AST node structure for our genetic programming framework.

**4_random_rotated_landing.toml**

In the previous scenario, the starting rotation was fixed. What if the
starting rotation is randomized? Can we evolve a solution that generalizes?

Implementing an AST Node
------------------------

- AST nodes should have the following traits: `Debug`, `RustcEncodable`,
  `RustcDecodable`, `Clone`, `PartialEq`, `AstNode`, `RandNode`. The former 5
  can be derived, for the latter 2 we have a macro called `impl_astnode!()`.
- `impl_astnode!()` takes an integer that should be unique among all node types
  (pick 4 or a higher number), and a description of all variant cases of the
  enum of your AST node type. Each AST node arm is labeled with `leaf` to
  indicate it's a "leaf" node of the AST tree, or `int` to indicate it's an
  "internal" node (meaning it has other AST nodes as children). This labeling is
  used to limit the depth of the tree: while randomly generating trees, we'll
  reduce the chances of picking an internal node as we go deeper in the tree, so
  we don't recurse infinitely.
- For your particular new AST Node, which needs to produce a `Command`, you'll
  also want to implement `EvaluateToComamnd`. This is the interesting trait,
  which will describe how the node should be evaluated by the simulation engine.
- Your node type probably wants to contain nodes of other types: `Condition`,
  which implements `BooleanValue` with a method `is_true()`, or `Expression`
  which implements `NumericValue`, with method `num_value`. See the `doc`
  directory for the API documentation.
- You'll also need to add a new `example` program to run it. The simplest
  way to do that is to copy/paste `evolve_condition`, and search/replacing
  the `Condition` type with your new type.


The `EvaluateToCommand` trait looks like this:

    pub trait EvaluateToCommand {
        fn evaluate(&self, sensor_data: &SensorData) -> Command;
    }

Prerequisites
-------------

This repository is designed to be checked out _next_ to the `moonlander-gp`
crate, since it has a hard path reference to that crate in `Cargo.toml`.

It goes best together with `moonlander-visualization`, to visualize the output
of training runs.
