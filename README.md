# lem_in

## todo

...

## mandatory

- [ ] the goal of this project is to find the quickest way to get n ants across the farm
- [ ] quickest way means the solution with the least number of lines, respecting the
output format requested below
- [ ] obviously, there are some basic constraints. to be the first to arrive, ants will need
to take the shortest path (and that isn’t necessarily the simplest). they will also
need to avoid traffic jams as well as walking all over their fellow ants
- [ ] at the beginning of the game, all the ants are in the room ##start. the goal is to bring them to the room ##end with as few turns as possible. each room can only contain one ant at a time. (except at ##start and ##end which can contain as many ants as necessary)
- [ ] we consider that all the ants are in the room ##start at the beginning of the game
- [ ] at each turn you will only display the ants that moved
- [ ] at each turn you can move each ant only once and through a tube (the room at the receiving end must be empty)
- [ ] you must display your results on the standard output in the following format:
  ```
  number_of_ants
  the_rooms
  the_links
  La1-r1 La2-r2 La3-r3 ...
  ```

## bonus

- [ ] ant farm visualizer
  - either in 2 dimensions, seen from the top. or even better from the perspective of an ant in the corridors of the farm in 3d
  - to use it, we could write: `./lem-in < ant_farm_map.txt | ./visu-hex`
  - please note that because the commands and comments also appear on the standard output, it is possible to pass specific commands to the visualizer (such as various colors or levels)
  - you should have noticed that the room’s coordinates will only be useful here.
- ... (at least 5 bonus for 125)

## push check

- [ ] reread subject
- [ ] check below 2 seconds for 4000 rooms
- [ ] `make test`
- [ ] no forbidden function (`nm -u`)
- [ ] check all leaks with valgrind
- [ ] check all leaks with fsanitize
- [ ] clone vogsphere
