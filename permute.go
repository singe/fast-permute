package main

import (
  "fmt"
  "os"
)

func out_flush(out *[]byte, handle *os.File) {
  fmt.Fprintf(handle, "%s", *out)
  *out = nil
}

func out_push(out *[]byte, pw_buf *[]byte, handle *os.File) {
  *out = append(*out, *pw_buf...)
  if len(*out) >= 8192 - 300 {
    out_flush(out, handle)
  }
}

func next_permutation(word *[]byte, p *[]int, k int) int {
  (*p)[k] -= 1
  j := k % 2 * (*p)[k]
  (*word)[j], (*word)[k] = (*word)[k], (*word)[j]
  for k = 1; (*p)[k] == 0; k++ {
    (*p)[k] = k
  }
  return k
}

func main() {
  handle := os.Stdout
  var out []byte = make([]byte, 0, 8192)

  //var line_buf = []byte("abcd\n")
  var line_buf = []byte("0123456789ab\n")
  var line_len = len(line_buf) - 1

  var k int
  var p []int = make([]int, line_len+1)
  for k := range p {
    p[k] = k
  }
  k = 1

  out_push(&out, &line_buf, handle)
  for k < line_len {
    k = next_permutation(&line_buf, &p, k)
    out_push(&out, &line_buf, handle)
  }

  out_flush(&out, handle)
}
