use futures::executor::block_on;

fn main() {
    println!("Hello, world!");
    let future = do_something();
    block_on(future);
}

async fn do_something() {
    println!("go go go !");
}
