fn main() {
    let x    = 10;
    let r    = &x;    // r    -> x
    let rr   = &r;    // rr   -> r   -> x
    let rrr  = &rr;   // rrr  -> rr  -> r  -> x
    let rrrr = &rrr;  // rrrr -> rrr -> rr -> r -> x

    if is_ten_deref_with_no_pain(rrrr) {
        println!(" Painless! ");
    }

    if is_ten_deref_with_pain(rrrr) {
        println!(" Bullshit! ");
    }
  }

  fn is_ten_deref_with_no_pain(x: &i32) -> bool {
    *x == 10
  }

  fn is_ten_deref_with_pain(x: &&&&i32) -> bool {
      ****x == 10
  }
