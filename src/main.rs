mod domain;

//imports
use domain::domain_test;

#[tokio::main]
async fn main() {
    let google = domain_test();
    println!("{:?}", google);

}
