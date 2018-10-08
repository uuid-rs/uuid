use prelude::*;

use BytesError;

use winapi::shared::guiddef;

#[cfg(feature = "guid")]
impl Uuid {
    /// Attempts to create a [`Uuid`] from a winapi `GUID`
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    fn from_guid(guid: guiddef::GUID) -> Result<Uuid, BytesError> {
        Uuid::from_fields(
            guid.Data1 as u32,
            guid.Data2 as u16,
            guid.Data3 as u16,
            &(guid.Data4 as [u8; 8]),
        )
    }

    /// Converts a [`Uuid`] into a little endian winapi `GUID`
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    fn to_guid(&self) -> guiddef::GUID {
        let (data1, data2, data3, data4) = self.as_fields();

        guiddef::GUID {
            Data1: data1,
            Data2: data2,
            Data3: data3,
            Data4: *data4,
        }
    }
}

#[cfg(feature = "guid")]
#[cfg(test)]
mod tests {
    use prelude::*;
    use std::str::FromStr;
    use winapi::shared::guiddef;

    #[test]
    fn test_to_uuid() {
        let uuid =
            Uuid::from_str("735d359d-4bc4-4e07-8c49-eb3e99a048dc").unwrap();
        let guid = uuid.to_guid();
        assert_eq!(Ok(uuid), Uuid::from_guid(guid));
    }

    #[test]
    fn test_from_guid() {
        let guid = guiddef::GUID {
            Data1: 0x4a35229d,
            Data2: 0x5527,
            Data3: 0x4f30,
            Data4: [0x86, 0x47, 0x9d, 0xc5, 0x4e, 0x1e, 0xe1, 0xe8],
        };

        let uuid = Uuid::from_guid(guid).unwrap();
        assert_eq!(
            "4a35229d-5527-4f30-8647-9dc54e1ee1e8",
            uuid.to_hyphenated().to_string()
        );
        assert_eq!(Ok(uuid), Uuid::from_guid(guid));
    }
}
