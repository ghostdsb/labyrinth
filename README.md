# Labyrinth

Mazes For Programmers in Rust

## Binary Tree Algorithm

![Binary Tree Labyrinth](./labyrinths/binary_tree.png)

## Sidewinder Algorithm

![Sidewinder Labyrinth](./labyrinths/sidewinder.png)

## Djikstra Solutions

![Binary Tree Labyrinth](./labyrinths/binary_dijkstra_sol.png)
![Sidewinder Labyrinth](./labyrinths/sidewinder_djikstra_sol.png)

## Colored Solutions

Darkness of color depicts distance from center square(the farther the darker).
This lets us see, quite clearly, the structure of the maze. We’re shining Dijkstra-flavored X-rays at it and seeing what’s inside. It turns out that this works great for letting us visually compare all kinds of different maze algorithms.
For example, the first labyrinth was generated using binary tree algorithm and second using sidewinder algorithm.
![Binary Tree Labyrinth](./labyrinths/col-binary.png)
![Sidewinder Labyrinth](./labyrinths/col-sidewinder.png)

## Aldous Broder Algorithm

In Aldous Broder Algorithm we randomly hop from cell to neighbour cell and create a path through them(in not already present).
A time taking algo, but lacks bias.

![Aldous Broder Labyrinth path](./labyrinths/aldous_broder_path.png)
![Aldous Broder Labyrinth bg](./labyrinths/aldous_broder_background.png)

> [**L:** Longest path in Labyrinth;   **R:** flow-fill along the longest path]

## Wilson Algorithm

The algorithm starts by choosing a point on the grid—any point—and marking it visited. Then it chooses any unvisited cell in the grid and does one of these loop-erased random walks until it encounters a visited cell. At that point it adds the path it followed to the maze, marking as visited each of the cells along that path, and then it goes again. The process repeats until all the cells in the grid have been visited.

Unbaised like Aldous Broder but focuses on visited cells rather than unvisited cells.

![Wilson Labyrinth 1](./labyrinths/wilson1.png)
![Wilson Labyrinth 2](./labyrinths/wilson2.png)

> Color fans out from center of labyrinth towards deeper, farther paths.