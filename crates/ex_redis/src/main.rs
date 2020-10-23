use redis::Commands;
use redis::

fn main() {
    fetch_an_integer();
}


fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    
    let client = redis::Client::open("redis://redis-test-01.corp.funji.club/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    // let _ : () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let res = con.get("my_key");
    println!("{:#?}", res);
    res
}