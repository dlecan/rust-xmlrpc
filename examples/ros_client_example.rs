extern crate xmlrpc;

fn main() {
    let client = xmlrpc::Client::new("http://xmlrpc-server.example");
    let mut request = xmlrpc::Request::new("getSystemState");
    request = request.argument(&"/").finalize();
    let response = client.remote_call(&request).unwrap();
    let value: Vec<(i32, String, Vec<Vec<(String, Vec<String>)>>)> = response.result().unwrap();
    println!("{:?}", value);
}
