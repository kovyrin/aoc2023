# Brainstorming

## Case 1

```
 5,1,2,1,5 => 15
```

Here are the results we know are correct:
```
mask  : ??###.??????????????
result: #####.#.##.#.#####..
result: #####.#.##.#..#####.
result: #####.#.##..#.#####.
result: #####.#..##.#.#####.
result: #####..#.##.#.#####.
result: #####.#.##.#...#####
result: #####.#.##..#..#####
result: #####.#..##.#..#####
result: #####..#.##.#..#####
result: #####.#.##...#.#####
result: #####.#..##..#.#####
result: #####..#.##..#.#####
result: #####.#...##.#.#####
result: #####..#..##.#.#####
result: #####...#.##.#.#####
```
### Brainstorming steps

* Find the biggest one: 5
* It can only be at the beginning: `#####.??????????????`
* This means ?????????????? (14 spaces) contains 1,2,1,5
* There would be 3 spaces between the numbers and potentially one space on each end.
* To place those somewhere, we need at least 1+2+1+5 + 3 (spaces between) = 12 spaces (#.##.#.#####)
* So we have 14 - 12 = 2 extra spaces left to play with.
* We need to find the number of way to place to k=2 spaces into N=5 slots.
* The way to do this seems to be: N + comb(N, k) = 5 + comb(5, 2) = 5 + 10 = 15
* So we have 15 ways to place the spaces (which matches the correct answer)

Alternative solution:
* We try to fit the first item (5)
* It fits perfectly into pos=0: `#####.c`
* In the next block of `??????????????` (14 spaces), we can potentially fit `1,2,1,5`
* From this point forward, the logic is the same.


## Case 2

```
#.#?????..??????#? 1,1,1,1,1,3 => 37
```

Correct list of results:
```
result: #.#.#.#...#...###.
result: #.#.#..#..#...###.
result: #.#..#.#..#...###.
result: #.#.#.#....#..###.
result: #.#.#..#...#..###.
result: #.#..#.#...#..###.
result: #.#.#.#.....#.###.
result: #.#.#..#....#.###.
result: #.#..#.#....#.###.
result: #.#.#.....#.#.###.
result: #.#..#....#.#.###.
result: #.#...#...#.#.###.
result: #.#....#..#.#.###.
result: #.#.#.#...#....###
result: #.#.#..#..#....###
result: #.#..#.#..#....###
result: #.#.#.#....#...###
result: #.#.#..#...#...###
result: #.#..#.#...#...###
result: #.#.#.#.....#..###
result: #.#.#..#....#..###
result: #.#..#.#....#..###
result: #.#.#.....#.#..###
result: #.#..#....#.#..###
result: #.#...#...#.#..###
result: #.#....#..#.#..###
result: #.#.#.#......#.###
result: #.#.#..#.....#.###
result: #.#..#.#.....#.###
result: #.#.#.....#..#.###
result: #.#..#....#..#.###
result: #.#...#...#..#.###
result: #.#....#..#..#.###
result: #.#.#......#.#.###
result: #.#..#.....#.#.###
result: #.#...#....#.#.###
result: #.#....#...#.#.###
```

### Brainstorming steps

#.#?????..??????#?

The most compact representation of the list:
```
#,#,#,#,#,###
```

We know that the first two 1s are present in their places and there is a space after them:
```
#.#.????..??????#?
```
Then we see a group of 4 ?s. Going down the list, we still have 1,1,1,3 to place.

Since we need N+N-1 spaces to place N numbers, we know that we can only fit 1,1 into the 4 spaces.
That would take 3 spaces, leaving us with k=1 space to play with.
The number of places to place it into N = 1 + 1 + (2-1) = 3.
So, to calculate our options, we do N + comb(N, k) = 3 + comb(3, 1) = 3 + 3 = 6

The next group is `??????#?` and we still have 1,3 to place.
If they were all ?s, we would have 8 spaces to play with.
Then we would need 1+3+1 = 5 spaces and that leaves us k=3 spaces to play with
Number of places to put them would be N = 1+3+(2-1) = 5
Result: N + comb(N, k) = 5 + comb(5, 3) = 5 + 10 = 15.

But there is a constraint placed on us by the already present #.





## Case 3

```
?###???????? 3,2,1
``````

Unfolded version:
```
?###??????????###??????????###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1,3,2,1,3,2,1
```

Find the first place where we can place the 3.
.###.?????????###??????????###??????????###??????????###????????

Now we have a block of `?????????` (9 spaces) where we can put `##.#.###.`.
We get
.###.##.#.###.###??????????###??????????###??????????###????????

But the next number to place is 2 and we don't have a way to place it.


What it we try to place only 2,1 in there? We will need a dot at the end because the ???s are followed by #.
.###.##.#.???.###??????????###??????????###??????????###????????
That works and gives us N combinations.

Then we place the next 3:
.###.##.#.___.###.?????????###??????????###??????????###????????

Same pattern repeats until the end.
