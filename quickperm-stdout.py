#!/usr/bin/env python

from sys import argv,stdout
from os import path
from multiprocessing import Pool
from signal import signal, SIGTERM
from functools import partial
from itertools import combinations

def run(subset):
  bufcount = 0
  buf = ""

  list_in = list(subset)
  list_len = len(subset)

  p = list(range(0, list_len + 1))
  x = 1
  buf += (''.join(list_in)+'\n')
  bufcount += 1
  while x < list_len:
    p[x] -= 1
    if x % 2 == 1:
      j = p[x]
    else:
      j = 0
    list_in[j], list_in[x] = list_in[x], list_in[j]
    buf += (''.join(list_in)+'\n')
    bufcount += 1
    x = 1
    while p[x] == 0:
      p[x] = x
      x += 1
    if bufcount == 18:
      print(buf,end="")
      buf = ""
      bufcount = 0
    if buf != "": print(buf,end="")
  return

if __name__ == '__main__':
  with open(argv[1],'r') as infile:
    start = infile.read().split("\n")
    del start[-1]
  
  for i in range(int(argv[2]),int(argv[3])+1):
    if i == 1:
      print('\n'.join(start))
    else:
      with Pool() as pool:
        pool.map(run, combinations(start, i))
