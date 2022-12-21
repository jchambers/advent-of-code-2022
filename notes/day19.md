- Blueprint 1:
  - Each ore robot costs 4 ore.
  - Each clay robot costs 2 ore.
  - Each obsidian robot costs 3 ore and 14 clay.
  - Each geode robot costs 2 ore and 7 obsidian.

# Deadlines

Thinking about the time value of robots, an ore robot finished at the start of minute 1 will produce 24 ore over the course of the simulation. An ore robot built at the last minute will produce only one ore.

Can we work backwards from there? Can we prune branches by setting deadlines (i.e. is there a time by which we MUST have one obsidian?)? If we want at least one geode, we must start building a geode robot no later than minute 23, and so we must have at least 7 obsidian (and 2 ore) by then.

To have 7 obsidian no later than minute 23, we could have one obsidian robot done at minute 16. We could do it with two obsidian robots at minutes 19 and 20. With three robots, we'd still need four minutes; if we finished at minutes 21, 22, and 23, that would only give us 6 units by the end, so we'd need to start a minute earlier at minute 20. With four obsidian robots, we'd be producing more than we need every round.

So the latest we could even _consider_ starting in on obsidian robots is minute 19 (to finish at minute 20). Working backwards from there, we can figure out the latest time we could possibly build a clay robot. Does that really help, though?

# Excess spending

Another angle: is there a point where building more of a resource is provably not helpful? In the blueprint above, there's never a point in having more than three ore robots since we can't spend more than three ore per turn (I mean, we _could_ spend four on another ore robot, but that's self-defeating).

Similarly, there's never a point in exceeding a 14:3 ratio of clay robots to ore robots since then we'd be producing more clay than we can use.

BUT EVIDENTLY THIS IS WRONG. This causes tests to fail for the second blueprint:

- Blueprint 2:
  - Each ore robot costs 2 ore.
  - Each clay robot costs 3 ore.
  - Each obsidian robot costs 3 ore and 8 clay.
  - Each geode robot costs 3 ore and 12 obsidian.

The ore limit still seems good, but the clay and obsidian limits are both causing problems. In hindsight, this is likely happening because we can "bank" ore, so those limits might just be out. We can still set a limit at the maximum clay/obsidian consumption, but that seems less helpful.

# Goals?

What if, instead of thinking about next moves, we thought about next goals? That might help us cut out some states—and particularly waiting states—we don't need to consider. Does that turn this into a more manageable graph exploration problem?

A related(?) idea: if we're in state A and can theoretically get to state B in a short amount of time or to state C in a longer amount of time, is there a good way to assess the relative value of those moves?

# Table-filling

Can we think of this as a "table-filling" problem? As a first pass, what if we considered an ore-only world? We could construct a two-dimensional table that has the amount of ore collected on one axis, the number of ore-collecting robots on the other, and the minimum time to reach that state as the value of each cell. We'd start with our one robot and zero ore:

| Robots/ore | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|          1 | 0 |   |   |   |   |   |   |   |   |   |
|          2 |   |   |   |   |   |   |   |   |   |   |
|          3 |   |   |   |   |   |   |   |   |   |   |

…then could start filling in the top row:

| Robots/ore | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|          1 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|          2 |   |   |   |   |   |   |   |   |   |   |
|          3 |   |   |   |   |   |   |   |   |   |   |

…then the row with two robots. To get to two robots and zero ore, we'd be making a transition from 1 robot and 4 ore and would take one unit of time to get there, so:

| Robots/ore | 0 | 1 | 2 | 3 | 4 |  5 |  6 |  7 |  8 |  9 |
|          1 | 0 | 1 | 2 | 3 | 4 |  5 |  6 |  7 |  8 |  9 |
|          2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
|          3 |   |   |   |   |   |    |    |    |    |    |

…but, wait, is that right? Once we have two robots, we're producing ore twice as fast. To quote from the problem statement:

```
== Minute 4 ==
1 ore-collecting robot collects 1 ore; you now have 4 ore.

== Minute 5 ==
Spend 4 ore to start building an ore-collecting robot.
1 ore-collecting robot collects 1 ore; you now have 1 ore.
The new ore-collecting robot is ready; you now have 2 of them.

== Minute 6 ==
2 ore-collecting robots collect 2 ore; you now have 3 ore.
```

So, for one, it seems like there's no path where we actually have 2 robots and 0 ore, and there's only one way to get to 2 robots, 1 ore (and 2 robots, 2 ore). There are multiple ways to get to 2 robots, 3 ore, though:

- Wait until 1 robot, 6 ore before building the second ore robot
- Build the second ore robot at 1 robot, 4 ore and then wait one tick (which gives us a delta of 2 ore)

Trying that again:

| Robots/ore | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|          1 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|          2 | - | 5 | 6 | 6 | 7 | 7 | 8 | 8 | 9 | 9 |
|          3 |   |   |   |   |   |   |   |   |   |   |

So, as an observation, we're filling in cells in row 2 by looking at:

1. Each cell in row 1 with ore > 4 (so x in row 2 + 3)
2. The cell two steps back in row 2

…and then the minimum time to arrive at a state in row 2 is `min(a, b) + 1`.

Attempting the three-robot row, we can transition to three robots from any cell in the row with two robots that has 4 or more ore:

| Robots/ore | 0 | 1 | 2 | 3 | 4 | 5 | 6 |  7 |  8 |  9 |
|-------------------------------------------------------|
|          1 | 0 | 1 | 2 | 3 | 4 | 5 | 6 |  7 |  8 |  9 |
|          2 | - | 5 | 6 | 6 | 7 | 7 | 8 |  8 |  9 |  9 |
|          3 | - | - | 8 | 8 | 9 | 9 | 9 | 10 | 10 | 10 |

As before, we can make the transition to 3 robots from any 2-robot state with more than 4 ore. With three robots, we're always generating a minimum of two ore (in that very first cell where the third robot is under construction), but more likely three, so that means some states (3 robots, 0 ore; 3 robots, 1 ore) are unreachable.

Generalizing a bit:

```
T_min[ore][robots] = min(
  // We already have enough robots, so just wait for resources
  T_min[ore - robots][robots],
  
  // Build a new robot; the previous-row column we're looking at is the cost of the robot plus `ore` (note to
  // confused future-self: `ore` is the amount of `ore` that is in inventory in the current state), minus the
  // amount of ore that will be produced by the robots already working.
  T_min[robot_cost + ore - robots][robots - 1]
) + 1
```

We can limit the number of rows in this particular table to 3 (we know we don't need more than 3 ore robots) and can limit the number of columns to however far we could get in the given time limit.

What happens if we add in the clay robot? We're now concerned with two kinds of robots and two kinds of resources, and so we're moving from a two- to an eight-dimensional table. The ore-only table is still valid in the slice of that expanded table where we have no clay robots and no collected clay.

We can start populating the "clay space" by making transitions from 0 clay robots to 1 clay robot, and we can make that transition from any state where we have 0 clay robots and more than 4 collected ore. We know that when we build our first clay robot, we'll have no clay in inventory.

```
T_min[clay][clay_robots][ore][ore_robots] = min(
  // Just wait
  T_min[clay - clay_robots][clay_robots][ore - ore_robots][ore_robots],
  
  // Build a new robot
  T_min[clay - clay_robots][clay_robots - 1][clay_robot_cost + ore - robots][ore_robots]
) + 1
```

## Interlude: upper bounds

We've set (maybe?) upper bounds on the number of robots we can have, but can we set an upper bound on the quantities of each type of resource we'll have? Let's assume we can build an ore robot every turn. Then each ore robot will produce `time_limit - time_built` units of ore, and that works out to (I think) `time_limit * (time_limit + 1) / 2`. So for a time limit of 24, that's 300. For a time limit of 32, it's 528. Large, but manageable.

…but we know we're only building a maximum of three ore robots (in this example), which limits things quite a bit. Going back to our table (handy!), we know we'd have 3 ore robots and 2 ore on turn 8. We'd produce 3 ore every minute for 16 minutes for an additional 48, which limits us to 56 ore produced by the end.

Let's assume that's an upper bound for all of the resource types (it's not, but this is just for an estimate). With that in mind, how big would our 8-dimensional table be?

3 ore robots * 56 ore * 14 clay robots * 56 clay * 7 obsidian robots * 56 obsidian * 10(?) geode robots * 56 geodes
 ==> 28,913,418,240 possible states

…which is still entirely impractical.

But ideally we don't need to explore the whole space. How can we prune it?

## Practically-equivalent states

At some point, even though the precise resource counts may be different, two states may be practically equivalent. If we have LIKE A BILLION ore in one state and LIKE TWO BILLION in another, both cases are equivalent in that we have way more ore than we can spend over the course of the simulation. Can we collapse those states to prune the search space?

At a given time, we can figure out the maximum amount of a resource we could practically spend for the rest of the simulation by finding the highest-cost thing we can spend those resources on and then assuming we can build one of those things every turn for the rest of the simulation. If our reserves plus expected production exceed that amount, then _both_ the resource count and the robot count for that resource are practically unlimited.

Proving that last part to myself, I think we want:

```
resources_by_end_of_simulation = current_reserves + (production_capacity * t)
maximum_resource_spend = highest_cost * t
```

Another way of thinking about it: given a certain amount of a resource in inventory, a production rate, and a maximum spending rate, we can figure out if we'll run out of resources before the end of the simulation. We're in "practically unlimited" territory if:

```
current_reserves >= (production_capacity - spending_rate) * t_remaining
```

Knowing that we're in this state has two big benefits:

1. We know we don't need to build any more robots for that resource type and
2. We can collapse a bunch of similar states into one, reducing branching
