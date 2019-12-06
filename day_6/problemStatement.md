# Day 6

## Problem Statement

You have the following map:
```
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
```
Visually, the above map of orbits looks like this:

```
        G - H       J - K - L
       /           /
COM - B - C - D - E - F
               \
                I

```

In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.

Here, we can count the total number of orbits as follows:

    D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
    L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
    COM orbits nothing.

The total number of direct and indirect orbits in this example is 42.

What is the total number of direct and indirect orbits in your map data?


## Initial thoughts

This is trivial once you've built a DAG. Then it's just the distance from each node to COM.

There might be libraries for this stuff in Rust, but lets code it from scratch.
