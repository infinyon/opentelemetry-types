mod macros;

/// Service stub and clients
pub mod collector {
    #[cfg(feature = "logs")]
    pub mod logs {
        pub mod v1 {
            crate::include_proto!("opentelemetry.proto.collector.logs.v1");
        }
    }

    #[cfg(feature = "metrics")]
    pub mod metrics {
        pub mod v1 {
            crate::include_proto!("opentelemetry.proto.collector.metrics.v1");
        }
    }

    #[cfg(feature = "traces")]
    pub mod trace {
        pub mod v1 {
            crate::include_proto!("opentelemetry.proto.collector.trace.v1");
        }
    }
}

/// Common types used across all signals
pub mod common {
    pub mod v1 {
        crate::include_proto!("opentelemetry.proto.common.v1");
    }
}

#[cfg(feature = "logs")]
/// Generated types used in logging.
pub mod logs {
    pub mod v1 {
        crate::include_proto!("opentelemetry.proto.logs.v1");
    }
}

#[cfg(feature = "metrics")]
/// Generated types used in metrics.
pub mod metrics {
    pub mod v1 {
        crate::include_proto!("opentelemetry.proto.metrics.v1");
    }
}

/// Generated types used in resources.
pub mod resource {
    pub mod v1 {
        crate::include_proto!("opentelemetry.proto.resource.v1");
    }
}

#[cfg(feature = "traces")]
/// Generated types used in traces.
pub mod trace {
    pub mod v1 {
        crate::include_proto!("opentelemetry.proto.trace.v1");
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use crate::metrics::v1::{Gauge, NumberDataPoint, number_data_point::Value};

    pub fn serialize_gauge(value: &Gauge) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(value.encoded_len());
        // Unwrap is safe, since we have reserved sufficient capacity in the vector.
        value.encode(&mut buf).unwrap();
        buf
    }

    pub fn deserialize_gauge(buf: &[u8]) -> Result<Gauge, prost::DecodeError> {
        Gauge::decode(buf)
    }

    #[test]
    fn serialize_and_deserialize() {
        let data_point = NumberDataPoint {
            flags: 0,
            attributes: Vec::new(),
            start_time_unix_nano: 10,
            time_unix_nano: 100,
            exemplars: Vec::new(),
            value: Some(Value::AsInt(3131)),
        };

        let gauge = Gauge {
            data_points: vec![data_point],
        };

        let bytes = serialize_gauge(&gauge);

        let new_gauge = deserialize_gauge(&bytes).unwrap();

        assert_eq!(new_gauge, gauge);
    }
}
