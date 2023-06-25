#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* Make it work without changing the function signatures of `init`*/
fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    Some(Box::leak(c))
}


pub fn main() {
    unsafe {
        config = init();

        println!("{:?}",config)
    }
}
