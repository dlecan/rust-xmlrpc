
extern crate xmlrpc;

fn main() {
    let master = std::env::var("ROS_MASTER_URI").unwrap();
    let client = xmlrpc::Client::new(&master);
    let mut request = xmlrpc::Request::new("getSystemState");
    request = request.argument(&"/").finalize();
    let response = client.remote_call(&request).unwrap();
    let value: (i32, String, Vec<Vec<(String, Vec<String>)>>) = response.result(0).unwrap();
    println!("{:?}", value);
}
