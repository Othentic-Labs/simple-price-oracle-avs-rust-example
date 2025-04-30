use crate::services::tls::prove;
use crate::services::tls::present;
use tlsn_examples::ExampleType;

pub async fn run() {
    let uri = "/formats/json";
    let extra_headers = vec![];
    let example_type = ExampleType::Json;

    println!("Notarizing task");
    let result = crate::services::tls::prove::notarize(uri, extra_headers, &example_type).await;
    println!("Task notarized. Result: {:?}", result);

    println!("Creating presentation");
    let _ = crate::services::tls::present::create_presentation(&example_type).await;
    println!("Presentation created");
}
