fn main() {
    let _client = xray_lite::DaemonClient::from_lambda_env();
    println!("{}", xray_lite::TraceId::new());
    println!("{}", xray_lite::SegmentId::new());
}
