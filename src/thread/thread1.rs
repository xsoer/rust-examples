use std::thread;
use std::time::Duration;

fn th1() -> bool {
    for i in 0..3 {
        thread::spawn(move || {
            println!("Hello from a thread!{}", i);
        });
        thread::sleep(Duration::from_secs(5));
    }
    return true;
}

fn th2(i: u8) -> u8 {
    i * i
}

fn th3(i: u8) -> bool {
    println!("{:?}", i);
    true
}

fn th4() -> bool{
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn th1_print() {
        assert_eq!(th1(), true);
    }

    #[test]
    fn th2_print() {
        assert_eq!(th2(3), 9);
    }

    #[test]
    fn ath3_print() {
        assert_eq!(th3(8), true);
    }

    #[test]
    fn th4_print() {
        assert_eq!(th4(), true);
    }
}
