use crate::Uuid;

use winapi::shared::guiddef;

#[cfg(feature = "guid")]
impl Uuid {
    /// Converts a winapi `GUID` into a [`Uuid`]
    ///
    /// This method will pass fields unchanged, so they must already
    /// be in the right endianness for a UUID.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const fn from_guid(guid: guiddef::GUID) -> Self {
        Uuid::from_fields(
            guid.Data1 as u32,
            guid.Data2 as u16,
            guid.Data3 as u16,
            &(guid.Data4 as [u8; 8]),
        )
    }

    /// Converts a [`Uuid`] into a winapi `GUID`
    ///
    /// This method will pass fields unchanged, so they must already
    /// be in the right endianness for a UUID.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub fn to_guid(&self) -> guiddef::GUID {
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
    use super::*;

    use crate::{std::string::ToString, Variant, Version};
    use winapi::{shared::guiddef, um::combaseapi::CoCreateGuid};

    #[test]
    fn test_parse_guid() {
        // This example GUID is directly from https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid
        let uuid = Uuid::parse_str("6B29FC40-CA47-1067-B31D-00DD010662DA").unwrap();

        assert_eq!(Variant::RFC4122, uuid.get_variant());
        assert_eq!(Some(Version::Mac), uuid.get_version());
    }

    #[test]
    fn test_new_native_guid() {
        let mut guid = guiddef::GUID {
            Data1: Default::default(),
            Data2: Default::default(),
            Data3: Default::default(),
            Data4: Default::default(),
        };
    
        unsafe {
            CoCreateGuid(&mut guid as *mut _);
        }
    
        let uuid = Uuid::from_guid(guid);
    
        assert_eq!(Variant::RFC4122, uuid.get_variant());
        assert_eq!(Some(Version::Random), uuid.get_version());
    }

    #[test]
    fn test_from_guid() {
        let guid = guiddef::GUID {
            Data1: 0x4a35229d,
            Data2: 0x5527,
            Data3: 0x4f30,
            Data4: [0x86, 0x47, 0x9d, 0xc5, 0x4e, 0x1e, 0xe1, 0xe8],
        };

        let uuid = Uuid::from_guid(guid);
        assert_eq!(
            "4a35229d-5527-4f30-8647-9dc54e1ee1e8",
            uuid.to_hyphenated().to_string()
        );
    }

    #[test]
    fn test_guid_roundtrip() {
        let guid_in = guiddef::GUID {
            Data1: 0x4a35229d,
            Data2: 0x5527,
            Data3: 0x4f30,
            Data4: [0x86, 0x47, 0x9d, 0xc5, 0x4e, 0x1e, 0xe1, 0xe8],
        };

        let uuid = Uuid::from_guid(guid_in);
        let guid_out = uuid.to_guid();

        assert_eq!(
            (guid_in.Data1, guid_in.Data2, guid_in.Data3, guid_in.Data4),
            (
                guid_out.Data1,
                guid_out.Data2,
                guid_out.Data3,
                guid_out.Data4
            )
        );
    }
}
