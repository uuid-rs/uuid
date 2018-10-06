use prelude::*;

use BytesError;

use winapi::shared::guiddef;

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
        let (data1, data2, data3, data4) = self.to_fields_le();
        
        guiddef::GUID {
            Data1: data1,
            Data2: data2,
            Data3: data3,
            Data4: data4,
        }
    }
}
