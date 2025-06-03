// @generated
impl serde::Serialize for AttestorVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ATTESTOR_VERSION_UNKNOWN",
            Self::AttestorVersion100 => "ATTESTOR_VERSION_1_0_0",
            Self::AttestorVersion110 => "ATTESTOR_VERSION_1_1_0",
            Self::AttestorVersion200 => "ATTESTOR_VERSION_2_0_0",
            Self::AttestorVersion201 => "ATTESTOR_VERSION_2_0_1",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AttestorVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ATTESTOR_VERSION_UNKNOWN",
            "ATTESTOR_VERSION_1_0_0",
            "ATTESTOR_VERSION_1_1_0",
            "ATTESTOR_VERSION_2_0_0",
            "ATTESTOR_VERSION_2_0_1",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttestorVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(AttestorVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(AttestorVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ATTESTOR_VERSION_UNKNOWN" => Ok(AttestorVersion::Unknown),
                    "ATTESTOR_VERSION_1_0_0" => Ok(AttestorVersion::AttestorVersion100),
                    "ATTESTOR_VERSION_1_1_0" => Ok(AttestorVersion::AttestorVersion110),
                    "ATTESTOR_VERSION_2_0_0" => Ok(AttestorVersion::AttestorVersion200),
                    "ATTESTOR_VERSION_2_0_1" => Ok(AttestorVersion::AttestorVersion201),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticatedUserData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.expires_at != 0 {
            len += 1;
        }
        if !self.host_whitelist.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.AuthenticatedUserData", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.created_at != 0 {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if self.expires_at != 0 {
            struct_ser.serialize_field("expiresAt", &self.expires_at)?;
        }
        if !self.host_whitelist.is_empty() {
            struct_ser.serialize_field("hostWhitelist", &self.host_whitelist)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticatedUserData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "createdAt",
            "expiresAt",
            "hostWhitelist",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreatedAt,
            ExpiresAt,
            HostWhitelist,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "createdAt" => Ok(GeneratedField::CreatedAt),
                            "expiresAt" => Ok(GeneratedField::ExpiresAt),
                            "hostWhitelist" => Ok(GeneratedField::HostWhitelist),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticatedUserData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.AuthenticatedUserData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AuthenticatedUserData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut created_at__ = None;
                let mut expires_at__ = None;
                let mut host_whitelist__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExpiresAt => {
                            if expires_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiresAt"));
                            }
                            expires_at__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HostWhitelist => {
                            if host_whitelist__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostWhitelist"));
                            }
                            host_whitelist__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AuthenticatedUserData {
                    id: id__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    expires_at: expires_at__.unwrap_or_default(),
                    host_whitelist: host_whitelist__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.AuthenticatedUserData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.AuthenticationRequest", len)?;
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            Signature,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.AuthenticationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AuthenticationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AuthenticationRequest {
                    data: data__,
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.AuthenticationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClaimContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.extracted_parameters.is_empty() {
            len += 1;
        }
        if !self.provider_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ClaimContext", len)?;
        if !self.extracted_parameters.is_empty() {
            struct_ser.serialize_field("extractedParameters", &self.extracted_parameters)?;
        }
        if !self.provider_hash.is_empty() {
            struct_ser.serialize_field("providerHash", &self.provider_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClaimContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extractedParameters",
            "providerHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExtractedParameters,
            ProviderHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "extractedParameters" => Ok(GeneratedField::ExtractedParameters),
                            "providerHash" => Ok(GeneratedField::ProviderHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClaimContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ClaimContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClaimContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extracted_parameters__ = None;
                let mut provider_hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExtractedParameters => {
                            if extracted_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extractedParameters"));
                            }
                            extracted_parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ProviderHash => {
                            if provider_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerHash"));
                            }
                            provider_hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClaimContext {
                    extracted_parameters: extracted_parameters__.unwrap_or_default(),
                    provider_hash: provider_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ClaimContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClaimRequestData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if self.timestamp_s != 0 {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ClaimRequestData", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if self.timestamp_s != 0 {
            struct_ser.serialize_field("timestampS", &self.timestamp_s)?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", &self.context)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClaimRequestData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
            "parameters",
            "owner",
            "timestampS",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Parameters,
            Owner,
            TimestampS,
            Context,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "owner" => Ok(GeneratedField::Owner),
                            "timestampS" => Ok(GeneratedField::TimestampS),
                            "context" => Ok(GeneratedField::Context),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClaimRequestData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ClaimRequestData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClaimRequestData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut parameters__ = None;
                let mut owner__ = None;
                let mut timestamp_s__ = None;
                let mut context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimestampS => {
                            if timestamp_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampS"));
                            }
                            timestamp_s__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClaimRequestData {
                    provider: provider__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    timestamp_s: timestamp_s__.unwrap_or_default(),
                    context: context__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ClaimRequestData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClaimTunnelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        if !self.transcript.is_empty() {
            len += 1;
        }
        if self.signatures.is_some() {
            len += 1;
        }
        if self.zk_engine != 0 {
            len += 1;
        }
        if !self.fixed_server_iv.is_empty() {
            len += 1;
        }
        if !self.fixed_client_iv.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ClaimTunnelRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if !self.transcript.is_empty() {
            struct_ser.serialize_field("transcript", &self.transcript)?;
        }
        if let Some(v) = self.signatures.as_ref() {
            struct_ser.serialize_field("signatures", v)?;
        }
        if self.zk_engine != 0 {
            let v = ZkProofEngine::from_i32(self.zk_engine)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.zk_engine)))?;
            struct_ser.serialize_field("zkEngine", &v)?;
        }
        if !self.fixed_server_iv.is_empty() {
            struct_ser.serialize_field("fixedServerIV", pbjson::private::base64::encode(&self.fixed_server_iv).as_str())?;
        }
        if !self.fixed_client_iv.is_empty() {
            struct_ser.serialize_field("fixedClientIV", pbjson::private::base64::encode(&self.fixed_client_iv).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClaimTunnelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "data",
            "transcript",
            "signatures",
            "zkEngine",
            "fixedServerIV",
            "fixedClientIV",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            Data,
            Transcript,
            Signatures,
            ZkEngine,
            FixedServerIv,
            FixedClientIv,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "request" => Ok(GeneratedField::Request),
                            "data" => Ok(GeneratedField::Data),
                            "transcript" => Ok(GeneratedField::Transcript),
                            "signatures" => Ok(GeneratedField::Signatures),
                            "zkEngine" => Ok(GeneratedField::ZkEngine),
                            "fixedServerIV" => Ok(GeneratedField::FixedServerIv),
                            "fixedClientIV" => Ok(GeneratedField::FixedClientIv),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClaimTunnelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ClaimTunnelRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClaimTunnelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                let mut data__ = None;
                let mut transcript__ = None;
                let mut signatures__ = None;
                let mut zk_engine__ = None;
                let mut fixed_server_iv__ = None;
                let mut fixed_client_iv__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
                        }
                        GeneratedField::Transcript => {
                            if transcript__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transcript"));
                            }
                            transcript__ = Some(map.next_value()?);
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = map.next_value()?;
                        }
                        GeneratedField::ZkEngine => {
                            if zk_engine__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zkEngine"));
                            }
                            zk_engine__ = Some(map.next_value::<ZkProofEngine>()? as i32);
                        }
                        GeneratedField::FixedServerIv => {
                            if fixed_server_iv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedServerIV"));
                            }
                            fixed_server_iv__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FixedClientIv => {
                            if fixed_client_iv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedClientIV"));
                            }
                            fixed_client_iv__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ClaimTunnelRequest {
                    request: request__,
                    data: data__,
                    transcript: transcript__.unwrap_or_default(),
                    signatures: signatures__,
                    zk_engine: zk_engine__.unwrap_or_default(),
                    fixed_server_iv: fixed_server_iv__.unwrap_or_default(),
                    fixed_client_iv: fixed_client_iv__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ClaimTunnelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for claim_tunnel_request::Signatures {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ClaimTunnelRequest.Signatures", len)?;
        if !self.request_signature.is_empty() {
            struct_ser.serialize_field("requestSignature", pbjson::private::base64::encode(&self.request_signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for claim_tunnel_request::Signatures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requestSignature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestSignature,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestSignature" => Ok(GeneratedField::RequestSignature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = claim_tunnel_request::Signatures;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ClaimTunnelRequest.Signatures")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<claim_tunnel_request::Signatures, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestSignature => {
                            if request_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestSignature"));
                            }
                            request_signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(claim_tunnel_request::Signatures {
                    request_signature: request_signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ClaimTunnelRequest.Signatures", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for claim_tunnel_request::TranscriptMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sender != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if self.reveal.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ClaimTunnelRequest.TranscriptMessage", len)?;
        if self.sender != 0 {
            let v = TranscriptMessageSenderType::from_i32(self.sender)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.sender)))?;
            struct_ser.serialize_field("sender", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", pbjson::private::base64::encode(&self.message).as_str())?;
        }
        if let Some(v) = self.reveal.as_ref() {
            struct_ser.serialize_field("reveal", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for claim_tunnel_request::TranscriptMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "message",
            "reveal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Message,
            Reveal,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "message" => Ok(GeneratedField::Message),
                            "reveal" => Ok(GeneratedField::Reveal),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = claim_tunnel_request::TranscriptMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ClaimTunnelRequest.TranscriptMessage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<claim_tunnel_request::TranscriptMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut message__ = None;
                let mut reveal__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value::<TranscriptMessageSenderType>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Reveal => {
                            if reveal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reveal"));
                            }
                            reveal__ = map.next_value()?;
                        }
                    }
                }
                Ok(claim_tunnel_request::TranscriptMessage {
                    sender: sender__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    reveal: reveal__,
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ClaimTunnelRequest.TranscriptMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClaimTunnelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        if self.signatures.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ClaimTunnelResponse", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if let Some(v) = self.signatures.as_ref() {
            struct_ser.serialize_field("signatures", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            match v {
                claim_tunnel_response::Result::Claim(v) => {
                    struct_ser.serialize_field("claim", v)?;
                }
                claim_tunnel_response::Result::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClaimTunnelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "signatures",
            "claim",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            Signatures,
            Claim,
            Error,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "request" => Ok(GeneratedField::Request),
                            "signatures" => Ok(GeneratedField::Signatures),
                            "claim" => Ok(GeneratedField::Claim),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClaimTunnelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ClaimTunnelResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClaimTunnelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                let mut signatures__ = None;
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map.next_value()?;
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = map.next_value()?;
                        }
                        GeneratedField::Claim => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claim"));
                            }
                            result__ = map.next_value::<::std::option::Option<_>>()?.map(claim_tunnel_response::Result::Claim)
;
                        }
                        GeneratedField::Error => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            result__ = map.next_value::<::std::option::Option<_>>()?.map(claim_tunnel_response::Result::Error)
;
                        }
                    }
                }
                Ok(ClaimTunnelResponse {
                    request: request__,
                    signatures: signatures__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ClaimTunnelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for claim_tunnel_response::Signatures {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attestor_address.is_empty() {
            len += 1;
        }
        if !self.claim_signature.is_empty() {
            len += 1;
        }
        if !self.result_signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ClaimTunnelResponse.Signatures", len)?;
        if !self.attestor_address.is_empty() {
            struct_ser.serialize_field("attestorAddress", &self.attestor_address)?;
        }
        if !self.claim_signature.is_empty() {
            struct_ser.serialize_field("claimSignature", pbjson::private::base64::encode(&self.claim_signature).as_str())?;
        }
        if !self.result_signature.is_empty() {
            struct_ser.serialize_field("resultSignature", pbjson::private::base64::encode(&self.result_signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for claim_tunnel_response::Signatures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attestorAddress",
            "claimSignature",
            "resultSignature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttestorAddress,
            ClaimSignature,
            ResultSignature,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attestorAddress" => Ok(GeneratedField::AttestorAddress),
                            "claimSignature" => Ok(GeneratedField::ClaimSignature),
                            "resultSignature" => Ok(GeneratedField::ResultSignature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = claim_tunnel_response::Signatures;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ClaimTunnelResponse.Signatures")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<claim_tunnel_response::Signatures, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attestor_address__ = None;
                let mut claim_signature__ = None;
                let mut result_signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AttestorAddress => {
                            if attestor_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestorAddress"));
                            }
                            attestor_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClaimSignature => {
                            if claim_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimSignature"));
                            }
                            claim_signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResultSignature => {
                            if result_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultSignature"));
                            }
                            result_signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(claim_tunnel_response::Signatures {
                    attestor_address: attestor_address__.unwrap_or_default(),
                    claim_signature: claim_signature__.unwrap_or_default(),
                    result_signature: result_signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ClaimTunnelResponse.Signatures", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompleteClaimOnAvsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        if self.task_index != 0 {
            len += 1;
        }
        if !self.completed_task_json.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.CompleteClaimOnAvsRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.task_index != 0 {
            struct_ser.serialize_field("taskIndex", &self.task_index)?;
        }
        if !self.completed_task_json.is_empty() {
            struct_ser.serialize_field("completedTaskJson", &self.completed_task_json)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompleteClaimOnAvsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chainId",
            "taskIndex",
            "completedTaskJson",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            TaskIndex,
            CompletedTaskJson,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" => Ok(GeneratedField::ChainId),
                            "taskIndex" => Ok(GeneratedField::TaskIndex),
                            "completedTaskJson" => Ok(GeneratedField::CompletedTaskJson),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompleteClaimOnAvsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.CompleteClaimOnAvsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CompleteClaimOnAvsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut task_index__ = None;
                let mut completed_task_json__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TaskIndex => {
                            if task_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskIndex"));
                            }
                            task_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CompletedTaskJson => {
                            if completed_task_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completedTaskJson"));
                            }
                            completed_task_json__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CompleteClaimOnAvsRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    task_index: task_index__.unwrap_or_default(),
                    completed_task_json: completed_task_json__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.CompleteClaimOnAvsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompleteClaimOnAvsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        if !self.task_completed_object_json.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.CompleteClaimOnAvsResponse", len)?;
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", &self.tx_hash)?;
        }
        if !self.task_completed_object_json.is_empty() {
            struct_ser.serialize_field("taskCompletedObjectJson", &self.task_completed_object_json)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompleteClaimOnAvsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txHash",
            "taskCompletedObjectJson",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxHash,
            TaskCompletedObjectJson,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txHash" => Ok(GeneratedField::TxHash),
                            "taskCompletedObjectJson" => Ok(GeneratedField::TaskCompletedObjectJson),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompleteClaimOnAvsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.CompleteClaimOnAvsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CompleteClaimOnAvsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tx_hash__ = None;
                let mut task_completed_object_json__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::TaskCompletedObjectJson => {
                            if task_completed_object_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskCompletedObjectJson"));
                            }
                            task_completed_object_json__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CompleteClaimOnAvsResponse {
                    tx_hash: tx_hash__.unwrap_or_default(),
                    task_completed_object_json: task_completed_object_json__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.CompleteClaimOnAvsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTaskOnMechainRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.CreateTaskOnMechainRequest", len)?;
        if self.timestamp != 0 {
            struct_ser.serialize_field("timestamp", &self.timestamp)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTaskOnMechainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTaskOnMechainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.CreateTaskOnMechainRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateTaskOnMechainRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreateTaskOnMechainRequest {
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.CreateTaskOnMechainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTaskOnMechainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.task_id != 0 {
            len += 1;
        }
        if self.required_attestors != 0 {
            len += 1;
        }
        if !self.hosts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.CreateTaskOnMechainResponse", len)?;
        if self.task_id != 0 {
            struct_ser.serialize_field("taskId", &self.task_id)?;
        }
        if self.required_attestors != 0 {
            struct_ser.serialize_field("requiredAttestors", &self.required_attestors)?;
        }
        if !self.hosts.is_empty() {
            struct_ser.serialize_field("hosts", &self.hosts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTaskOnMechainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "taskId",
            "requiredAttestors",
            "hosts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaskId,
            RequiredAttestors,
            Hosts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "taskId" => Ok(GeneratedField::TaskId),
                            "requiredAttestors" => Ok(GeneratedField::RequiredAttestors),
                            "hosts" => Ok(GeneratedField::Hosts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTaskOnMechainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.CreateTaskOnMechainResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateTaskOnMechainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task_id__ = None;
                let mut required_attestors__ = None;
                let mut hosts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TaskId => {
                            if task_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskId"));
                            }
                            task_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequiredAttestors => {
                            if required_attestors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredAttestors"));
                            }
                            required_attestors__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hosts => {
                            if hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hosts"));
                            }
                            hosts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CreateTaskOnMechainResponse {
                    task_id: task_id__.unwrap_or_default(),
                    required_attestors: required_attestors__.unwrap_or_default(),
                    hosts: hosts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.CreateTaskOnMechainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTunnelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.host.is_empty() {
            len += 1;
        }
        if self.port != 0 {
            len += 1;
        }
        if !self.geo_location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.CreateTunnelRequest", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if self.port != 0 {
            struct_ser.serialize_field("port", &self.port)?;
        }
        if !self.geo_location.is_empty() {
            struct_ser.serialize_field("geoLocation", &self.geo_location)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTunnelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "host",
            "port",
            "geoLocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Host,
            Port,
            GeoLocation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "host" => Ok(GeneratedField::Host),
                            "port" => Ok(GeneratedField::Port),
                            "geoLocation" => Ok(GeneratedField::GeoLocation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTunnelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.CreateTunnelRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateTunnelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut host__ = None;
                let mut port__ = None;
                let mut geo_location__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map.next_value()?);
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GeoLocation => {
                            if geo_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("geoLocation"));
                            }
                            geo_location__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CreateTunnelRequest {
                    id: id__.unwrap_or_default(),
                    host: host__.unwrap_or_default(),
                    port: port__.unwrap_or_default(),
                    geo_location: geo_location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.CreateTunnelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataSlice {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.from_index != 0 {
            len += 1;
        }
        if self.length != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.DataSlice", len)?;
        if self.from_index != 0 {
            struct_ser.serialize_field("fromIndex", &self.from_index)?;
        }
        if self.length != 0 {
            struct_ser.serialize_field("length", &self.length)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataSlice {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fromIndex",
            "length",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FromIndex,
            Length,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fromIndex" => Ok(GeneratedField::FromIndex),
                            "length" => Ok(GeneratedField::Length),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataSlice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.DataSlice")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DataSlice, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from_index__ = None;
                let mut length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FromIndex => {
                            if from_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromIndex"));
                            }
                            from_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Length => {
                            if length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            length__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DataSlice {
                    from_index: from_index__.unwrap_or_default(),
                    length: length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.DataSlice", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DisconnectTunnelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.DisconnectTunnelRequest", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DisconnectTunnelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DisconnectTunnelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.DisconnectTunnelRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DisconnectTunnelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DisconnectTunnelRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.DisconnectTunnelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Empty {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("reclaim_attestor.Empty", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Empty {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Empty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.Empty")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Empty, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Empty {
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.Empty", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ErrorNoError => "ERROR_NO_ERROR",
            Self::ErrorInternal => "ERROR_INTERNAL",
            Self::ErrorBadRequest => "ERROR_BAD_REQUEST",
            Self::ErrorNotFound => "ERROR_NOT_FOUND",
            Self::ErrorProxyError => "ERROR_PROXY_ERROR",
            Self::ErrorInvalidClaim => "ERROR_INVALID_CLAIM",
            Self::ErrorNetworkError => "ERROR_NETWORK_ERROR",
            Self::ErrorPaymentRefused => "ERROR_PAYMENT_REFUSED",
            Self::ErrorBgpAnnouncementOverlap => "ERROR_BGP_ANNOUNCEMENT_OVERLAP",
            Self::ErrorAuthenticationFailed => "ERROR_AUTHENTICATION_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_NO_ERROR",
            "ERROR_INTERNAL",
            "ERROR_BAD_REQUEST",
            "ERROR_NOT_FOUND",
            "ERROR_PROXY_ERROR",
            "ERROR_INVALID_CLAIM",
            "ERROR_NETWORK_ERROR",
            "ERROR_PAYMENT_REFUSED",
            "ERROR_BGP_ANNOUNCEMENT_OVERLAP",
            "ERROR_AUTHENTICATION_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ErrorCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ErrorCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ERROR_NO_ERROR" => Ok(ErrorCode::ErrorNoError),
                    "ERROR_INTERNAL" => Ok(ErrorCode::ErrorInternal),
                    "ERROR_BAD_REQUEST" => Ok(ErrorCode::ErrorBadRequest),
                    "ERROR_NOT_FOUND" => Ok(ErrorCode::ErrorNotFound),
                    "ERROR_PROXY_ERROR" => Ok(ErrorCode::ErrorProxyError),
                    "ERROR_INVALID_CLAIM" => Ok(ErrorCode::ErrorInvalidClaim),
                    "ERROR_NETWORK_ERROR" => Ok(ErrorCode::ErrorNetworkError),
                    "ERROR_PAYMENT_REFUSED" => Ok(ErrorCode::ErrorPaymentRefused),
                    "ERROR_BGP_ANNOUNCEMENT_OVERLAP" => Ok(ErrorCode::ErrorBgpAnnouncementOverlap),
                    "ERROR_AUTHENTICATION_FAILED" => Ok(ErrorCode::ErrorAuthenticationFailed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ErrorData", len)?;
        if self.code != 0 {
            let v = ErrorCode::from_i32(self.code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ErrorData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ErrorData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                let mut data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value::<ErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ErrorData {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ErrorData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.client_version != 0 {
            len += 1;
        }
        if self.signature_type != 0 {
            len += 1;
        }
        if self.auth.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.InitRequest", len)?;
        if self.client_version != 0 {
            let v = AttestorVersion::from_i32(self.client_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_version)))?;
            struct_ser.serialize_field("clientVersion", &v)?;
        }
        if self.signature_type != 0 {
            let v = ServiceSignatureType::from_i32(self.signature_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.signature_type)))?;
            struct_ser.serialize_field("signatureType", &v)?;
        }
        if let Some(v) = self.auth.as_ref() {
            struct_ser.serialize_field("auth", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "clientVersion",
            "signatureType",
            "auth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientVersion,
            SignatureType,
            Auth,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clientVersion" => Ok(GeneratedField::ClientVersion),
                            "signatureType" => Ok(GeneratedField::SignatureType),
                            "auth" => Ok(GeneratedField::Auth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.InitRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_version__ = None;
                let mut signature_type__ = None;
                let mut auth__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientVersion => {
                            if client_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientVersion"));
                            }
                            client_version__ = Some(map.next_value::<AttestorVersion>()? as i32);
                        }
                        GeneratedField::SignatureType => {
                            if signature_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureType"));
                            }
                            signature_type__ = Some(map.next_value::<ServiceSignatureType>()? as i32);
                        }
                        GeneratedField::Auth => {
                            if auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auth"));
                            }
                            auth__ = map.next_value()?;
                        }
                    }
                }
                Ok(InitRequest {
                    client_version: client_version__.unwrap_or_default(),
                    signature_type: signature_type__.unwrap_or_default(),
                    auth: auth__,
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.InitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.toprf_public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.InitResponse", len)?;
        if !self.toprf_public_key.is_empty() {
            struct_ser.serialize_field("toprfPublicKey", pbjson::private::base64::encode(&self.toprf_public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "toprfPublicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ToprfPublicKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "toprfPublicKey" => Ok(GeneratedField::ToprfPublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.InitResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut toprf_public_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ToprfPublicKey => {
                            if toprf_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toprfPublicKey"));
                            }
                            toprf_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(InitResponse {
                    toprf_public_key: toprf_public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.InitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageReveal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reveal.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.MessageReveal", len)?;
        if let Some(v) = self.reveal.as_ref() {
            match v {
                message_reveal::Reveal::DirectReveal(v) => {
                    struct_ser.serialize_field("directReveal", v)?;
                }
                message_reveal::Reveal::ZkReveal(v) => {
                    struct_ser.serialize_field("zkReveal", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageReveal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directReveal",
            "zkReveal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectReveal,
            ZkReveal,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "directReveal" => Ok(GeneratedField::DirectReveal),
                            "zkReveal" => Ok(GeneratedField::ZkReveal),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageReveal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.MessageReveal")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MessageReveal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reveal__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DirectReveal => {
                            if reveal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directReveal"));
                            }
                            reveal__ = map.next_value::<::std::option::Option<_>>()?.map(message_reveal::Reveal::DirectReveal)
;
                        }
                        GeneratedField::ZkReveal => {
                            if reveal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zkReveal"));
                            }
                            reveal__ = map.next_value::<::std::option::Option<_>>()?.map(message_reveal::Reveal::ZkReveal)
;
                        }
                    }
                }
                Ok(MessageReveal {
                    reveal: reveal__,
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.MessageReveal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for message_reveal::MessageRevealDirect {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.iv.is_empty() {
            len += 1;
        }
        if self.record_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.MessageReveal.MessageRevealDirect", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if !self.iv.is_empty() {
            struct_ser.serialize_field("iv", pbjson::private::base64::encode(&self.iv).as_str())?;
        }
        if self.record_number != 0 {
            struct_ser.serialize_field("recordNumber", &self.record_number)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for message_reveal::MessageRevealDirect {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "iv",
            "recordNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Iv,
            RecordNumber,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "iv" => Ok(GeneratedField::Iv),
                            "recordNumber" => Ok(GeneratedField::RecordNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = message_reveal::MessageRevealDirect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.MessageReveal.MessageRevealDirect")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<message_reveal::MessageRevealDirect, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut iv__ = None;
                let mut record_number__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Iv => {
                            if iv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iv"));
                            }
                            iv__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RecordNumber => {
                            if record_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordNumber"));
                            }
                            record_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(message_reveal::MessageRevealDirect {
                    key: key__.unwrap_or_default(),
                    iv: iv__.unwrap_or_default(),
                    record_number: record_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.MessageReveal.MessageRevealDirect", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for message_reveal::MessageRevealZk {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.proofs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.MessageReveal.MessageRevealZk", len)?;
        if !self.proofs.is_empty() {
            struct_ser.serialize_field("proofs", &self.proofs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for message_reveal::MessageRevealZk {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proofs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Proofs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "proofs" => Ok(GeneratedField::Proofs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = message_reveal::MessageRevealZk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.MessageReveal.MessageRevealZk")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<message_reveal::MessageRevealZk, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proofs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Proofs => {
                            if proofs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofs"));
                            }
                            proofs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(message_reveal::MessageRevealZk {
                    proofs: proofs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.MessageReveal.MessageRevealZk", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for message_reveal::ZkProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.proof_json.is_empty() {
            len += 1;
        }
        if !self.decrypted_redacted_ciphertext.is_empty() {
            len += 1;
        }
        if !self.redacted_plaintext.is_empty() {
            len += 1;
        }
        if self.start_idx != 0 {
            len += 1;
        }
        if !self.proof_data.is_empty() {
            len += 1;
        }
        if self.toprf.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.MessageReveal.ZKProof", len)?;
        if !self.proof_json.is_empty() {
            struct_ser.serialize_field("proofJson", &self.proof_json)?;
        }
        if !self.decrypted_redacted_ciphertext.is_empty() {
            struct_ser.serialize_field("decryptedRedactedCiphertext", pbjson::private::base64::encode(&self.decrypted_redacted_ciphertext).as_str())?;
        }
        if !self.redacted_plaintext.is_empty() {
            struct_ser.serialize_field("redactedPlaintext", pbjson::private::base64::encode(&self.redacted_plaintext).as_str())?;
        }
        if self.start_idx != 0 {
            struct_ser.serialize_field("startIdx", &self.start_idx)?;
        }
        if !self.proof_data.is_empty() {
            struct_ser.serialize_field("proofData", pbjson::private::base64::encode(&self.proof_data).as_str())?;
        }
        if let Some(v) = self.toprf.as_ref() {
            struct_ser.serialize_field("toprf", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for message_reveal::ZkProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proofJson",
            "decryptedRedactedCiphertext",
            "redactedPlaintext",
            "startIdx",
            "proofData",
            "toprf",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProofJson,
            DecryptedRedactedCiphertext,
            RedactedPlaintext,
            StartIdx,
            ProofData,
            Toprf,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "proofJson" => Ok(GeneratedField::ProofJson),
                            "decryptedRedactedCiphertext" => Ok(GeneratedField::DecryptedRedactedCiphertext),
                            "redactedPlaintext" => Ok(GeneratedField::RedactedPlaintext),
                            "startIdx" => Ok(GeneratedField::StartIdx),
                            "proofData" => Ok(GeneratedField::ProofData),
                            "toprf" => Ok(GeneratedField::Toprf),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = message_reveal::ZkProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.MessageReveal.ZKProof")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<message_reveal::ZkProof, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proof_json__ = None;
                let mut decrypted_redacted_ciphertext__ = None;
                let mut redacted_plaintext__ = None;
                let mut start_idx__ = None;
                let mut proof_data__ = None;
                let mut toprf__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProofJson => {
                            if proof_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofJson"));
                            }
                            proof_json__ = Some(map.next_value()?);
                        }
                        GeneratedField::DecryptedRedactedCiphertext => {
                            if decrypted_redacted_ciphertext__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decryptedRedactedCiphertext"));
                            }
                            decrypted_redacted_ciphertext__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RedactedPlaintext => {
                            if redacted_plaintext__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redactedPlaintext"));
                            }
                            redacted_plaintext__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartIdx => {
                            if start_idx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startIdx"));
                            }
                            start_idx__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProofData => {
                            if proof_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofData"));
                            }
                            proof_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Toprf => {
                            if toprf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toprf"));
                            }
                            toprf__ = map.next_value()?;
                        }
                    }
                }
                Ok(message_reveal::ZkProof {
                    proof_json: proof_json__.unwrap_or_default(),
                    decrypted_redacted_ciphertext: decrypted_redacted_ciphertext__.unwrap_or_default(),
                    redacted_plaintext: redacted_plaintext__.unwrap_or_default(),
                    start_idx: start_idx__.unwrap_or_default(),
                    proof_data: proof_data__.unwrap_or_default(),
                    toprf: toprf__,
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.MessageReveal.ZKProof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderClaimData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if self.timestamp_s != 0 {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        if !self.identifier.is_empty() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ProviderClaimData", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if self.timestamp_s != 0 {
            struct_ser.serialize_field("timestampS", &self.timestamp_s)?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", &self.context)?;
        }
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
        }
        if self.epoch != 0 {
            struct_ser.serialize_field("epoch", &self.epoch)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProviderClaimData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
            "parameters",
            "owner",
            "timestampS",
            "context",
            "identifier",
            "epoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Parameters,
            Owner,
            TimestampS,
            Context,
            Identifier,
            Epoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "owner" => Ok(GeneratedField::Owner),
                            "timestampS" => Ok(GeneratedField::TimestampS),
                            "context" => Ok(GeneratedField::Context),
                            "identifier" => Ok(GeneratedField::Identifier),
                            "epoch" => Ok(GeneratedField::Epoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderClaimData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ProviderClaimData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProviderClaimData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut parameters__ = None;
                let mut owner__ = None;
                let mut timestamp_s__ = None;
                let mut context__ = None;
                let mut identifier__ = None;
                let mut epoch__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimestampS => {
                            if timestamp_s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestampS"));
                            }
                            timestamp_s__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = Some(map.next_value()?);
                        }
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map.next_value()?);
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProviderClaimData {
                    provider: provider__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    timestamp_s: timestamp_s__.unwrap_or_default(),
                    context: context__.unwrap_or_default(),
                    identifier: identifier__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ProviderClaimData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderClaimInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.ProviderClaimInfo", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", &self.context)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProviderClaimInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
            "parameters",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Parameters,
            Context,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "context" => Ok(GeneratedField::Context),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderClaimInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.ProviderClaimInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProviderClaimInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut parameters__ = None;
                let mut context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ProviderClaimInfo {
                    provider: provider__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    context: context__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.ProviderClaimInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RpcMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.RPCMessage", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if let Some(v) = self.message.as_ref() {
            match v {
                rpc_message::Message::InitRequest(v) => {
                    struct_ser.serialize_field("initRequest", v)?;
                }
                rpc_message::Message::InitResponse(v) => {
                    struct_ser.serialize_field("initResponse", v)?;
                }
                rpc_message::Message::ConnectionTerminationAlert(v) => {
                    struct_ser.serialize_field("connectionTerminationAlert", v)?;
                }
                rpc_message::Message::RequestError(v) => {
                    struct_ser.serialize_field("requestError", v)?;
                }
                rpc_message::Message::CreateTunnelRequest(v) => {
                    struct_ser.serialize_field("createTunnelRequest", v)?;
                }
                rpc_message::Message::CreateTunnelResponse(v) => {
                    struct_ser.serialize_field("createTunnelResponse", v)?;
                }
                rpc_message::Message::DisconnectTunnelRequest(v) => {
                    struct_ser.serialize_field("disconnectTunnelRequest", v)?;
                }
                rpc_message::Message::DisconnectTunnelResponse(v) => {
                    struct_ser.serialize_field("disconnectTunnelResponse", v)?;
                }
                rpc_message::Message::TunnelMessage(v) => {
                    struct_ser.serialize_field("tunnelMessage", v)?;
                }
                rpc_message::Message::TunnelDisconnectEvent(v) => {
                    struct_ser.serialize_field("tunnelDisconnectEvent", v)?;
                }
                rpc_message::Message::ClaimTunnelRequest(v) => {
                    struct_ser.serialize_field("claimTunnelRequest", v)?;
                }
                rpc_message::Message::ClaimTunnelResponse(v) => {
                    struct_ser.serialize_field("claimTunnelResponse", v)?;
                }
                rpc_message::Message::CreateClaimOnChainRequest(v) => {
                    struct_ser.serialize_field("createClaimOnChainRequest", v)?;
                }
                rpc_message::Message::CreateClaimOnChainResponse(v) => {
                    struct_ser.serialize_field("createClaimOnChainResponse", v)?;
                }
                rpc_message::Message::CompleteClaimOnChainRequest(v) => {
                    struct_ser.serialize_field("completeClaimOnChainRequest", v)?;
                }
                rpc_message::Message::CompleteClaimOnChainResponse(v) => {
                    struct_ser.serialize_field("completeClaimOnChainResponse", v)?;
                }
                rpc_message::Message::ToprfRequest(v) => {
                    struct_ser.serialize_field("toprfRequest", v)?;
                }
                rpc_message::Message::ToprfResponse(v) => {
                    struct_ser.serialize_field("toprfResponse", v)?;
                }
                rpc_message::Message::CreateTaskOnMechainRequest(v) => {
                    struct_ser.serialize_field("createTaskOnMechainRequest", v)?;
                }
                rpc_message::Message::CreateTaskOnMechainResponse(v) => {
                    struct_ser.serialize_field("createTaskOnMechainResponse", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RpcMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "initRequest",
            "initResponse",
            "connectionTerminationAlert",
            "requestError",
            "createTunnelRequest",
            "createTunnelResponse",
            "disconnectTunnelRequest",
            "disconnectTunnelResponse",
            "tunnelMessage",
            "tunnelDisconnectEvent",
            "claimTunnelRequest",
            "claimTunnelResponse",
            "createClaimOnChainRequest",
            "createClaimOnChainResponse",
            "completeClaimOnChainRequest",
            "completeClaimOnChainResponse",
            "toprfRequest",
            "toprfResponse",
            "createTaskOnMechainRequest",
            "createTaskOnMechainResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            InitRequest,
            InitResponse,
            ConnectionTerminationAlert,
            RequestError,
            CreateTunnelRequest,
            CreateTunnelResponse,
            DisconnectTunnelRequest,
            DisconnectTunnelResponse,
            TunnelMessage,
            TunnelDisconnectEvent,
            ClaimTunnelRequest,
            ClaimTunnelResponse,
            CreateClaimOnChainRequest,
            CreateClaimOnChainResponse,
            CompleteClaimOnChainRequest,
            CompleteClaimOnChainResponse,
            ToprfRequest,
            ToprfResponse,
            CreateTaskOnMechainRequest,
            CreateTaskOnMechainResponse,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "initRequest" => Ok(GeneratedField::InitRequest),
                            "initResponse" => Ok(GeneratedField::InitResponse),
                            "connectionTerminationAlert" => Ok(GeneratedField::ConnectionTerminationAlert),
                            "requestError" => Ok(GeneratedField::RequestError),
                            "createTunnelRequest" => Ok(GeneratedField::CreateTunnelRequest),
                            "createTunnelResponse" => Ok(GeneratedField::CreateTunnelResponse),
                            "disconnectTunnelRequest" => Ok(GeneratedField::DisconnectTunnelRequest),
                            "disconnectTunnelResponse" => Ok(GeneratedField::DisconnectTunnelResponse),
                            "tunnelMessage" => Ok(GeneratedField::TunnelMessage),
                            "tunnelDisconnectEvent" => Ok(GeneratedField::TunnelDisconnectEvent),
                            "claimTunnelRequest" => Ok(GeneratedField::ClaimTunnelRequest),
                            "claimTunnelResponse" => Ok(GeneratedField::ClaimTunnelResponse),
                            "createClaimOnChainRequest" => Ok(GeneratedField::CreateClaimOnChainRequest),
                            "createClaimOnChainResponse" => Ok(GeneratedField::CreateClaimOnChainResponse),
                            "completeClaimOnChainRequest" => Ok(GeneratedField::CompleteClaimOnChainRequest),
                            "completeClaimOnChainResponse" => Ok(GeneratedField::CompleteClaimOnChainResponse),
                            "toprfRequest" => Ok(GeneratedField::ToprfRequest),
                            "toprfResponse" => Ok(GeneratedField::ToprfResponse),
                            "createTaskOnMechainRequest" => Ok(GeneratedField::CreateTaskOnMechainRequest),
                            "createTaskOnMechainResponse" => Ok(GeneratedField::CreateTaskOnMechainResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RpcMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.RPCMessage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RpcMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InitRequest => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initRequest"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::InitRequest)
;
                        }
                        GeneratedField::InitResponse => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initResponse"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::InitResponse)
;
                        }
                        GeneratedField::ConnectionTerminationAlert => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionTerminationAlert"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::ConnectionTerminationAlert)
;
                        }
                        GeneratedField::RequestError => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestError"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::RequestError)
;
                        }
                        GeneratedField::CreateTunnelRequest => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTunnelRequest"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::CreateTunnelRequest)
;
                        }
                        GeneratedField::CreateTunnelResponse => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTunnelResponse"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::CreateTunnelResponse)
;
                        }
                        GeneratedField::DisconnectTunnelRequest => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disconnectTunnelRequest"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::DisconnectTunnelRequest)
;
                        }
                        GeneratedField::DisconnectTunnelResponse => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disconnectTunnelResponse"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::DisconnectTunnelResponse)
;
                        }
                        GeneratedField::TunnelMessage => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tunnelMessage"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::TunnelMessage)
;
                        }
                        GeneratedField::TunnelDisconnectEvent => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tunnelDisconnectEvent"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::TunnelDisconnectEvent)
;
                        }
                        GeneratedField::ClaimTunnelRequest => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimTunnelRequest"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::ClaimTunnelRequest)
;
                        }
                        GeneratedField::ClaimTunnelResponse => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimTunnelResponse"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::ClaimTunnelResponse)
;
                        }
                        GeneratedField::CreateClaimOnChainRequest => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createClaimOnChainRequest"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::CreateClaimOnChainRequest)
;
                        }
                        GeneratedField::CreateClaimOnChainResponse => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createClaimOnChainResponse"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::CreateClaimOnChainResponse)
;
                        }
                        GeneratedField::CompleteClaimOnChainRequest => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completeClaimOnChainRequest"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::CompleteClaimOnChainRequest)
;
                        }
                        GeneratedField::CompleteClaimOnChainResponse => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completeClaimOnChainResponse"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::CompleteClaimOnChainResponse)
;
                        }
                        GeneratedField::ToprfRequest => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toprfRequest"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::ToprfRequest)
;
                        }
                        GeneratedField::ToprfResponse => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toprfResponse"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::ToprfResponse)
;
                        }
                        GeneratedField::CreateTaskOnMechainRequest => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTaskOnMechainRequest"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::CreateTaskOnMechainRequest)
;
                        }
                        GeneratedField::CreateTaskOnMechainResponse => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTaskOnMechainResponse"));
                            }
                            message__ = map.next_value::<::std::option::Option<_>>()?.map(rpc_message::Message::CreateTaskOnMechainResponse)
;
                        }
                    }
                }
                Ok(RpcMessage {
                    id: id__.unwrap_or_default(),
                    message: message__,
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.RPCMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RpcMessages {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.messages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.RPCMessages", len)?;
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RpcMessages {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "messages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Messages,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "messages" => Ok(GeneratedField::Messages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RpcMessages;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.RPCMessages")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RpcMessages, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut messages__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RpcMessages {
                    messages: messages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.RPCMessages", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestClaimOnAvsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        if !self.json_create_claim_request.is_empty() {
            len += 1;
        }
        if !self.request_signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.RequestClaimOnAvsRequest", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if !self.json_create_claim_request.is_empty() {
            struct_ser.serialize_field("jsonCreateClaimRequest", &self.json_create_claim_request)?;
        }
        if !self.request_signature.is_empty() {
            struct_ser.serialize_field("requestSignature", pbjson::private::base64::encode(&self.request_signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestClaimOnAvsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chainId",
            "jsonCreateClaimRequest",
            "requestSignature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            JsonCreateClaimRequest,
            RequestSignature,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" => Ok(GeneratedField::ChainId),
                            "jsonCreateClaimRequest" => Ok(GeneratedField::JsonCreateClaimRequest),
                            "requestSignature" => Ok(GeneratedField::RequestSignature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestClaimOnAvsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.RequestClaimOnAvsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RequestClaimOnAvsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut json_create_claim_request__ = None;
                let mut request_signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::JsonCreateClaimRequest => {
                            if json_create_claim_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonCreateClaimRequest"));
                            }
                            json_create_claim_request__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestSignature => {
                            if request_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestSignature"));
                            }
                            request_signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RequestClaimOnAvsRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    json_create_claim_request: json_create_claim_request__.unwrap_or_default(),
                    request_signature: request_signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.RequestClaimOnAvsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestClaimOnAvsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        if self.task_index != 0 {
            len += 1;
        }
        if !self.json_task.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.RequestClaimOnAvsResponse", len)?;
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", &self.tx_hash)?;
        }
        if self.task_index != 0 {
            struct_ser.serialize_field("taskIndex", &self.task_index)?;
        }
        if !self.json_task.is_empty() {
            struct_ser.serialize_field("jsonTask", &self.json_task)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestClaimOnAvsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txHash",
            "taskIndex",
            "jsonTask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxHash,
            TaskIndex,
            JsonTask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txHash" => Ok(GeneratedField::TxHash),
                            "taskIndex" => Ok(GeneratedField::TaskIndex),
                            "jsonTask" => Ok(GeneratedField::JsonTask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestClaimOnAvsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.RequestClaimOnAvsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RequestClaimOnAvsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tx_hash__ = None;
                let mut task_index__ = None;
                let mut json_task__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::TaskIndex => {
                            if task_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskIndex"));
                            }
                            task_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::JsonTask => {
                            if json_task__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonTask"));
                            }
                            json_task__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RequestClaimOnAvsResponse {
                    tx_hash: tx_hash__.unwrap_or_default(),
                    task_index: task_index__.unwrap_or_default(),
                    json_task: json_task__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.RequestClaimOnAvsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceSignatureType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "SERVICE_SIGNATURE_TYPE_UNKNOWN",
            Self::Eth => "SERVICE_SIGNATURE_TYPE_ETH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ServiceSignatureType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SERVICE_SIGNATURE_TYPE_UNKNOWN",
            "SERVICE_SIGNATURE_TYPE_ETH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceSignatureType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ServiceSignatureType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ServiceSignatureType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SERVICE_SIGNATURE_TYPE_UNKNOWN" => Ok(ServiceSignatureType::Unknown),
                    "SERVICE_SIGNATURE_TYPE_ETH" => Ok(ServiceSignatureType::Eth),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ToprfPayload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data_location.is_some() {
            len += 1;
        }
        if !self.nullifier.is_empty() {
            len += 1;
        }
        if !self.responses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.TOPRFPayload", len)?;
        if let Some(v) = self.data_location.as_ref() {
            struct_ser.serialize_field("dataLocation", v)?;
        }
        if !self.nullifier.is_empty() {
            struct_ser.serialize_field("nullifier", pbjson::private::base64::encode(&self.nullifier).as_str())?;
        }
        if !self.responses.is_empty() {
            struct_ser.serialize_field("responses", &self.responses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ToprfPayload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dataLocation",
            "nullifier",
            "responses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataLocation,
            Nullifier,
            Responses,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dataLocation" => Ok(GeneratedField::DataLocation),
                            "nullifier" => Ok(GeneratedField::Nullifier),
                            "responses" => Ok(GeneratedField::Responses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ToprfPayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.TOPRFPayload")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ToprfPayload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_location__ = None;
                let mut nullifier__ = None;
                let mut responses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DataLocation => {
                            if data_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataLocation"));
                            }
                            data_location__ = map.next_value()?;
                        }
                        GeneratedField::Nullifier => {
                            if nullifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullifier"));
                            }
                            nullifier__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Responses => {
                            if responses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responses"));
                            }
                            responses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ToprfPayload {
                    data_location: data_location__,
                    nullifier: nullifier__.unwrap_or_default(),
                    responses: responses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.TOPRFPayload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ToprfRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.masked_data.is_empty() {
            len += 1;
        }
        if self.engine != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.TOPRFRequest", len)?;
        if !self.masked_data.is_empty() {
            struct_ser.serialize_field("maskedData", pbjson::private::base64::encode(&self.masked_data).as_str())?;
        }
        if self.engine != 0 {
            let v = ZkProofEngine::from_i32(self.engine)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.engine)))?;
            struct_ser.serialize_field("engine", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ToprfRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "maskedData",
            "engine",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaskedData,
            Engine,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maskedData" => Ok(GeneratedField::MaskedData),
                            "engine" => Ok(GeneratedField::Engine),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ToprfRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.TOPRFRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ToprfRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut masked_data__ = None;
                let mut engine__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaskedData => {
                            if masked_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maskedData"));
                            }
                            masked_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Engine => {
                            if engine__.is_some() {
                                return Err(serde::de::Error::duplicate_field("engine"));
                            }
                            engine__ = Some(map.next_value::<ZkProofEngine>()? as i32);
                        }
                    }
                }
                Ok(ToprfRequest {
                    masked_data: masked_data__.unwrap_or_default(),
                    engine: engine__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.TOPRFRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ToprfResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.public_key_share.is_empty() {
            len += 1;
        }
        if !self.evaluated.is_empty() {
            len += 1;
        }
        if !self.c.is_empty() {
            len += 1;
        }
        if !self.r.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.TOPRFResponse", len)?;
        if !self.public_key_share.is_empty() {
            struct_ser.serialize_field("publicKeyShare", pbjson::private::base64::encode(&self.public_key_share).as_str())?;
        }
        if !self.evaluated.is_empty() {
            struct_ser.serialize_field("evaluated", pbjson::private::base64::encode(&self.evaluated).as_str())?;
        }
        if !self.c.is_empty() {
            struct_ser.serialize_field("c", pbjson::private::base64::encode(&self.c).as_str())?;
        }
        if !self.r.is_empty() {
            struct_ser.serialize_field("r", pbjson::private::base64::encode(&self.r).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ToprfResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "publicKeyShare",
            "evaluated",
            "c",
            "r",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKeyShare,
            Evaluated,
            C,
            R,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "publicKeyShare" => Ok(GeneratedField::PublicKeyShare),
                            "evaluated" => Ok(GeneratedField::Evaluated),
                            "c" => Ok(GeneratedField::C),
                            "r" => Ok(GeneratedField::R),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ToprfResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.TOPRFResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ToprfResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_key_share__ = None;
                let mut evaluated__ = None;
                let mut c__ = None;
                let mut r__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PublicKeyShare => {
                            if public_key_share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyShare"));
                            }
                            public_key_share__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Evaluated => {
                            if evaluated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evaluated"));
                            }
                            evaluated__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::C => {
                            if c__.is_some() {
                                return Err(serde::de::Error::duplicate_field("c"));
                            }
                            c__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::R => {
                            if r__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r"));
                            }
                            r__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ToprfResponse {
                    public_key_share: public_key_share__.unwrap_or_default(),
                    evaluated: evaluated__.unwrap_or_default(),
                    c: c__.unwrap_or_default(),
                    r: r__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.TOPRFResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranscriptMessageSenderType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "TRANSCRIPT_MESSAGE_SENDER_TYPE_UNKNOWN",
            Self::Client => "TRANSCRIPT_MESSAGE_SENDER_TYPE_CLIENT",
            Self::Server => "TRANSCRIPT_MESSAGE_SENDER_TYPE_SERVER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TranscriptMessageSenderType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TRANSCRIPT_MESSAGE_SENDER_TYPE_UNKNOWN",
            "TRANSCRIPT_MESSAGE_SENDER_TYPE_CLIENT",
            "TRANSCRIPT_MESSAGE_SENDER_TYPE_SERVER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranscriptMessageSenderType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(TranscriptMessageSenderType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(TranscriptMessageSenderType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TRANSCRIPT_MESSAGE_SENDER_TYPE_UNKNOWN" => Ok(TranscriptMessageSenderType::Unknown),
                    "TRANSCRIPT_MESSAGE_SENDER_TYPE_CLIENT" => Ok(TranscriptMessageSenderType::Client),
                    "TRANSCRIPT_MESSAGE_SENDER_TYPE_SERVER" => Ok(TranscriptMessageSenderType::Server),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TunnelDisconnectEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tunnel_id != 0 {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.TunnelDisconnectEvent", len)?;
        if self.tunnel_id != 0 {
            struct_ser.serialize_field("tunnelId", &self.tunnel_id)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TunnelDisconnectEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tunnelId",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TunnelId,
            Error,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tunnelId" => Ok(GeneratedField::TunnelId),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TunnelDisconnectEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.TunnelDisconnectEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TunnelDisconnectEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tunnel_id__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TunnelId => {
                            if tunnel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tunnelId"));
                            }
                            tunnel_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                    }
                }
                Ok(TunnelDisconnectEvent {
                    tunnel_id: tunnel_id__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.TunnelDisconnectEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TunnelMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tunnel_id != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("reclaim_attestor.TunnelMessage", len)?;
        if self.tunnel_id != 0 {
            struct_ser.serialize_field("tunnelId", &self.tunnel_id)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", pbjson::private::base64::encode(&self.message).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TunnelMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tunnelId",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TunnelId,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tunnelId" => Ok(GeneratedField::TunnelId),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TunnelMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct reclaim_attestor.TunnelMessage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TunnelMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tunnel_id__ = None;
                let mut message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TunnelId => {
                            if tunnel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tunnelId"));
                            }
                            tunnel_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TunnelMessage {
                    tunnel_id: tunnel_id__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("reclaim_attestor.TunnelMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZkProofEngine {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ZkEngineSnarkjs => "ZK_ENGINE_SNARKJS",
            Self::ZkEngineGnark => "ZK_ENGINE_GNARK",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZkProofEngine {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZK_ENGINE_SNARKJS",
            "ZK_ENGINE_GNARK",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZkProofEngine;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ZkProofEngine::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ZkProofEngine::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZK_ENGINE_SNARKJS" => Ok(ZkProofEngine::ZkEngineSnarkjs),
                    "ZK_ENGINE_GNARK" => Ok(ZkProofEngine::ZkEngineGnark),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
