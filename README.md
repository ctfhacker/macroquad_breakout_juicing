# Breakout juicing

Learning various juicing techniques following the [Juice It or Lose It](https://www.youtube.com/watch?v=Fy0aCDmgnxg) talk.

# Tweening (Easing)

* Tweening - Filling "in between" the start and end animation states

Example:

Move fast in the beginning, then slows as the object approaches the target

```
x += (target - x) * .1
````

# Techniques

* On reset, blocks fall from the top of the screen
  - Linear
  - Slow near target (ease_out_expo)
  - Just past and come back (ease_out_back)
  - Bounce (ease_out_bounce)
  - Add random delay to all blocks

