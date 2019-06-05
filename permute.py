#!/usr/bin/env python
# Create all permutations of a list of items
# by @singe
# e.g. an input file with 1\n2\n3 will produce:
#
#1
#2
#3
#12
#13
#21
#23
#31
#32
#123
#132
#213
#231
#312
#321

from itertools import permutations
from sys import argv,stdout
from multiprocessing import Process
from signal import signal, SIGTERM
from functools import partial

class Worker(Process):
  def signal_term_handler(self, outfile, buf, signal, frame):
      outfile.write(buf)
      outfile.close()
      exit(0)
  
  def run(self):
    bufcount = 0
    buf = ""
    with open(str(i)+argv[2],'w') as outfile:
      signal(SIGTERM, partial(Worker.signal_term_handler,self,outfile, buf))
      for x in permutations(start, i):
        buf += (''.join(x)+'\n')
        bufcount += 1
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
  
  jobs = []
  for i in range(1,len(start)+1):
    p = Worker()
    jobs.append(p)
    p.start()
  for j in jobs:
    j.join()
