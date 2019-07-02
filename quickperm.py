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
  pth, fil = path.split(argv[2])
  list_in = list(subset)
  list_len = len(subset)
  with open(pth+'/'+str(list_len)+fil,'a') as outfile:
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
        outfile.write(buf)
        buf = ""
        bufcount = 0
    if buf != "": outfile.write(buf)
  outfile.close()
  return

if __name__ == '__main__':
  with open(argv[1],'r') as infile:
    start = infile.read().split("\n")
    del start[-1]
  
  for i in range(int(argv[3]),int(argv[4])+1):
    if i == 1:
      pth, fil = path.split(argv[2])
      with open(pth+'/'+str(i)+fil,'a') as outfile:
        outfile.write('\n'.join(start))
    else:
      with Pool() as pool:
        pool.map(run, combinations(start, i))
