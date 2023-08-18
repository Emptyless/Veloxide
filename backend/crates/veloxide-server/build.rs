use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract_path = env::var("CONTRACTS_PATH").unwrap_or_else(|_| "../../../contracts".into());

    let helloworld_proto = format!("{}/helloworld.proto", contract_path);
    let bank_account_service_proto = format!("{}/bank_account_service.proto", contract_path);

    tonic_build::compile_protos(helloworld_proto)?;
    tonic_build::compile_protos(bank_account_service_proto)?;

    println!("cargo:rerun-if-changed=migrations");

    Ok(())
}
