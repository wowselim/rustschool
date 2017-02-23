fn main() {
  let now = 2017;
  let year_of_birth = 1995;
  let age = now - year_of_birth;

  println!("It is {0}. You were born in {1}. You will be {2} years old by the end of this year.",
    now, year_of_birth, age
  );
}
