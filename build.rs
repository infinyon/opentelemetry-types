use std::io::Error;

fn main() -> Result<(), Error> {
    prost_build::compile_protos(
            &[
                "src/opentelemetry-proto/opentelemetry/proto/common/v1/common.proto",
                "src/opentelemetry-proto/opentelemetry/proto/resource/v1/resource.proto",
                "src/opentelemetry-proto/opentelemetry/proto/trace/v1/trace.proto",
                "src/opentelemetry-proto/opentelemetry/proto/trace/v1/trace_config.proto",
                "src/opentelemetry-proto/opentelemetry/proto/collector/trace/v1/trace_service.proto",
                "src/opentelemetry-proto/opentelemetry/proto/metrics/v1/metrics.proto",
                "src/opentelemetry-proto/opentelemetry/proto/collector/metrics/v1/metrics_service.proto",
                "src/opentelemetry-proto/opentelemetry/proto/logs/v1/logs.proto",
                "src/opentelemetry-proto/opentelemetry/proto/collector/logs/v1/logs_service.proto",
            ],
            &["src/opentelemetry-proto"],
        )?;

    Ok(())
}