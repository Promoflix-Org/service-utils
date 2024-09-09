use okapi::openapi3::OpenApi;
use openapi_rs::gen::OpenApiGenerator;
use std::fs::File;
use std::io::Write;

use anyhow::Result;

pub type GenSpec = Box<dyn FnOnce(&str, &mut OpenApiGenerator)>;

#[derive(Debug, Clone)]
pub struct Spec<F: FnOnce(&str, &mut OpenApiGenerator)> {
    pub route: String,
    pub gen: F,
}

pub fn generate_openapi_spec<F: FnOnce(&str, &mut OpenApiGenerator)>(
    spec_fns: Vec<Spec<F>>,
    generator: &mut OpenApiGenerator,
) -> Result<()> {
    for spec_fn in spec_fns {
        //println!("Generating spec for {}", spec_fn.route);
        (spec_fn.gen)(&spec_fn.route, generator);
    }

    Ok(())
}

pub fn write_openapi_spec(openapi_spec: OpenApi) -> Result<()> {
    let mut spec_file = File::create("./swagger-ui/openapi.json")?;

    let json = serde_json::to_string(&openapi_spec)?;

    spec_file.write_all(json.as_bytes())?;
    Ok(())
}
