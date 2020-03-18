use std::io::{self, Write, BufReader};
use std::io::BufRead;
use std::fs::File;
use std::env;

struct OutT {
  fp: std::io::Stdout,
  buf: String,
  len: usize,
}

fn out_flush(out: &mut OutT) {
  if out.len == 0 { return(); }
  let mut handle: std::io::StdoutLock = out.fp.lock();
  handle.write(out.buf.as_bytes()).unwrap();
  out.len = 0;
}

fn out_push(out: &mut OutT, pw_buf: Vec<u8>, pw_len: usize) {
  let strbuf = String::from_utf8(pw_buf).unwrap();
  out.buf.push_str(&strbuf);
  out.buf.push_str("\n");
  out.len += pw_len + 1;

  if out.len >= 8192 - 300 {
    out_flush(out);
  }
}

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
  let args: Vec<String> = env::args().collect();
  let f = File::open(&args[1]).unwrap();
  let f = BufReader::new(f);

  for line in f.lines() {
    let mut line_buf = line.unwrap().into_bytes();
    let line_len = line_buf.len();

    let mut out = OutT {
      fp: io::stdout(),
      buf: String::from(""),
      len: 0,
    };
    
    let mut k;  
    let mut p = vec![0; line_len+1];
    for k in 0..(line_len+1) {
      p[k] = k;
    }
    k = 1;

    out_push(&mut out, line_buf.clone(), line_len);
    while k < line_len {
      k = next_permutation(&mut line_buf, &mut p, k);
      out_push(&mut out, line_buf.clone(), line_len);
    }
    //out_push(&mut out, &line_buf, line_len);
    out_flush(&mut out);
  } 
}
