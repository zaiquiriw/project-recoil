use testlib::test;

pub fn main() {
    let test_result = test();

    match test_result {
        Ok(_) => println!("Success!"),
        Err(error) => panic!("Problem inserting points: {error:?}"),
    }
}
