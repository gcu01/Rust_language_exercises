mod common_concepts;
use common_concepts::test_returns::{test_rtrn1, test_rtrn2, test_break};

fn main() {
    println!("Hello, world!");

println!("{}", test_rtrn1(4));
println!("{}", test_rtrn2(4));
println!("{}", test_break());

let v = [1,2, 3, 4, 5];

for elem in v {
    println!("{}", elem);
}
}


