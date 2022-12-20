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