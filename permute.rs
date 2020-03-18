/*
  A rust implementation of hashcat-util's permute.c
  https://github.com/hashcat/hashcat-utils/blob/master/src/permute.c

  By @singe with huge help from @scriptjunkie1

  Usage: ./permute <file>
*/


use std::io::{self, Write, BufReader, BufRead};
use std::fs::File;
use std::env;

// Write our buffer to stdout
fn out_flush(out: &mut Vec<u8>, handle: &mut std::io::StdoutLock) {
  //if out.len() == 0 { return(); }
  handle.write(&out).unwrap();
  out.clear();
}

// Push a newly generated permutation to our buffer
// if we hit the threshold flush it to stdout
fn out_push(out: &mut Vec<u8>, pw_buf: &[u8], handle: &mut std::io::StdoutLock) {
  out.extend_from_slice(&pw_buf);
  if out.len() >= 8192 - 300 {
    out_flush(out, handle);
  }
}

// Generate the next permutation using the quickperm algorithm
fn next_permutation(word: &mut Vec<u8>,p: &mut Vec<usize>, mut k: usize) -> usize {
  p[k] -= 1;
  let j = k % 2 * p[k];
  word.swap(k, j);
  k = 1;
  while p[k] == 0 {
    p[k] = k;
    k += 1;
  }
  return k;
}

fn main() {
  let so = io::stdout();
  let mut handle: std::io::StdoutLock = so.lock();

  // Open the filename passed in
  let args: Vec<String> = env::args().collect();
  let f = File::open(&args[1]).unwrap();
  let f = BufReader::new(f);

  let mut out = Vec::with_capacity(8192);

  for line in f.lines() {
    let mut line_str = line.unwrap();
    line_str.push_str("\n");
    let mut line_buf = line_str.into_bytes();
    let line_len = line_buf.len() - 1;

    // Initialise our quickperm vector 
    let mut k;  
    let mut p = vec![0; line_len+1];
    for k in 0..(line_len+1) {
      p[k] = k;
    }
    k = 1;

    // Generate permutations and put them in the buffer
    out_push(&mut out, &line_buf, &mut handle);
    while k < line_len {
      k = next_permutation(&mut line_buf, &mut p, k);
      out_push(&mut out, &line_buf, &mut handle);
    }

    // We're done do a flush of anything left in the buffer
    out_flush(&mut out, &mut handle);
  } // fetch the next line from the file
}
