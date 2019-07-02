# fast-permute
A fast python tool for creating permutations of alphanumerics

## What it does
If you have an input file containing:
```
1
2
3
```

This will produce
```
1
2
3
12
13
21
23
31
32
123
132
213
231
312
321
```

## How to run it

I recommend using pypy for maximum speed.

`pypy3 permute.py <input file> <output base name>`

This will create multiple files with the length of the strings in the file prepended e.g. if you give an output base name of "foo", then "7foo" will contain all permutations of length 7.

You can combine all permutations into one file with:
`cat *<output base name> > <output base name>`

## Generating specific lengths

You can limit the lengths of output strings generated using permute-minmax.py and passing it the minimum and maximum lengths you'd like to generate. e.g.

`pypy3 permute-minmax.py <input file> <output base name> <min length> <max length>`

As with permute.py you can concatenate the multiple output files together if you'd like to combine them.

## Performance

When I first started creating this tool, I was generating about 100M of output in 5s on my MBP running on a nearly dead battery. Currently, the code clocks in at 1.4G written in 5s on the same laptop. This is a list of the things that worked and didn't work.

### What worked

#### pypy

pypy is much faster than CPython thanks to Just-In-Time compiler magic documented at http://speed.pypy.org/. For example, using the following code:

```
with open(argv[1],'r') as infile:
  start = infile.read().split("\n")

with open(argv[2], 'w') as outfile:
  for i in range(0,len(start)+1):
    for x in permutations(start, i):
      outfile.write(''.join(x))
```

I got the following output written in 5s:

```
INTERPRETER=python SCRIPT=permute.py
117M out.txt
INTERPRETER=python3 SCRIPT=permute.py
131M out.txt
INTERPRETER=pypy SCRIPT=permute.py
261M out.txt
INTERPRETER=pypy3 SCRIPT=permute.py
253M out.txt
```

That's a nearly 2x speedup from python3 to pypy3.

#### Hand buffering

After reading https://www.reddit.com/r/unix/comments/6gxduc/how_is_gnu_yes_so_fast/ I decided to implement a simple counter that appended to a temporary buffer which was periodically flushed to disk. I hand bisected values from 10k to 1 and 18 worked best for my data set. I suspect this is quite specific to my data set and computer, and you may have luck with other values.

This gave a 1.3X speedup on the pypy3 best speed from above.

#### Multiprocessing

Multithreading was a complete disaster, multiprocessing worked well.

This presented three problems, the first was “what work do I spawn in a different thread”, the second was “how do I write to one file”, and the third was “how do I pass resources like file handles between processes”. I struggled to split up the work of the permutations() call into different processes, so settled instead on splitting each run of the for loop into a different process. Practically, this means you’ll have as many threads as lines in your input file, and that the first few threads will exit quickly (e.g. the first thread will just re-output your input file). So by the end of a long run, you’ll just have one thread taxing one core.

I didn’t handle the “how do I write to one file” and instead wrote to multiple files, then cat then together (aka a hack).

This gives another 4.4X speedup on top of the pypy and handbuffering speedups. Unbuffered, this still gives a 2.8X speedup.

#### Quickperm, Pools & Stdout

After atom from hashcat threw down the gauntlet with his permute.c from hashcat-utils https://twitter.com/hashcat/status/1136294080835719168 I switched to the quickperm algorithm from https://gist.github.com/brianpursley/57bbaf0a8823e51012bc. That gave a 21% speedup on the default itertools.permutations. (I also moved to tracking number of generated permutations, rather than disk space used as the primary indicator)

However, quickperm creates permutations of a fixed length (i.e. you send it 4 things, you'll get permutations of 4 things). So I used itertools.combinations to create different lists to send to quickperm.

Previously, I had a problem with multiprocessing, that I didn't have an easy way to parallelise, and did it on the different lengths. However, in atom's case, he was permuting on a fixed length and I wanted to try optimise that use case as well. That gave me the idea of handing each list from combinations off to a different process. Which worked super well, and gave a 5.3x speedup!

The problem was, if I wanted to do multiple lengths like the original permute, I quickly exceeded the OS's process limit. So I switched to using Pools instead of Processes from the multiprocessing library. That gave me similar speed across multiple lengths as I was getting on single lengths (same number of processes in each set to the number of cores in my laptop). Compared to my old permute approach, it gave a 1.8x speedup though!

Finally, I tested writing to stdout rather than to multiple individual files for multiple lengths. I like the file approach, because I can generate the permutations once, then use the different lengths as I need them later. But, we want speed! Switching to stdout provided an additional 1.4x speedup!

All in, for a fixed length of 12 and the '0123456789ab' charset, I got a speedup 7.2x over my previous approach (I don't think that number's a coincidence for an i7)! And for multiple lengths, I got a 2x speedup over my previous approach!

### What didn't work

#### Not using strings

In Python, strings are immutable (i.e. can’t be changed). That means appending to a string creates a whole new string object. Lists [] aren’t immutable, and can be modified. The idea was that if I append()’ed to the buffer as a list, it would be less intensive. While this makes sense, the final list would need to be join()’ed which I suspect is the slow down on the larger lists. Thus having the buffer as a string made more sense.

Additionally, if you google for example usage of permutations() like a good programmer, you’ll see lots of people wrap the call in a list() so they get that instead of a list. I was doing that originally, and ended up with a program that would get killed by the OOM after the list took up multiple Gs. That’s why I chose to write things to a file as soon as I got them.

#### Using fancy Python

Python has two cool calls map() and lambda. map() effectively runs a function across a list in a single call instead of you hand looping. lambda lets you define functions on the fly. So instead of this code:

```
for x in permutations(start, i):
  outfile.write(''.join(x))
```

You could have this code:

```
map((lambda x: outfile.write(''.join(x)+'\n')), permutations(start, i))
```

This was unfortunately, slower.

Next up, I tried list comprehensions. This is another way to run a function across a list with an optional filter. I didn’t need the filter so tried using this:

```
[outfile.write(''.join(x)+'\n') for x in permutations(start, i)]
```

While pretty, it too was slower.

#### Threading

I mentioned this earlier, but threading turned into a huge failure, and I’m still not sure why, which bugs me. Python has some famous weirdness with multithreading due to the Global Interpreter Lock (GIL). I thought this wouldn’t be too much of a problem, given it’s not supposed to impact I/O. It turns out it does, or at least my code was bad

First I tried something really simple, to use the threaded map() call with my fancy Python from above. That practically meant I changed the map call from the above to:

```
...
from multiprocessing.dummy import Pool as ThreadPool
...
pool = ThreadPool(4)
...
  pool.map((lambda x: outfile.write(''.join(x)+'\n')), permutations(start, i))
...
```

This was a spectacular failure. Next I tried more vanilla threading as described in https://pymotw.com/2/threading/. It looked so similar to the multiprocess example, that it’s not worth repeating here. Needless to say, it was the *worst* performer of any example with a 10x *slowdown*.
