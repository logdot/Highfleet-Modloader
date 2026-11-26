use crate::Result;

/// A loaded module in the current process.
///
/// Module lookup and address arithmetic don't need a patch session or access
/// to the loader's patch engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProcessModule {
    base: usize,
}

impl ProcessModule {
    /// Finds the main executable of the current Windows process.
    #[cfg(windows)]
    pub fn main() -> Result<Self> {
        use patchy_code::PatchError;
        use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

        // SAFETY: A null name requests the current executable.
        let module = unsafe { GetModuleHandleW(std::ptr::null()) };
        if module.is_null() {
            let error = std::io::Error::last_os_error();
            return Err(PatchError::ModuleLookup(error.to_string()).into());
        }
        Ok(Self::from_base(module as usize))
    }

    /// Creates a descriptor without checking that a module is loaded there.
    pub const fn from_base(base: usize) -> Self {
        Self { base }
    }

    /// Returns the module's runtime base address.
    pub const fn base_address(self) -> usize {
        self.base
    }

    /// Resolves an RVA against this module, rejecting address overflow.
    pub fn resolve_rva(self, rva: usize) -> Result<usize> {
        self.base
            .checked_add(rva)
            .ok_or_else(|| patchy_code::PatchError::AddressOverflow.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_rvas_against_the_supplied_base() {
        let module = ProcessModule::from_base(0x140000000);
        assert_eq!(module.base_address(), 0x140000000);
        assert_eq!(module.resolve_rva(0x36bb5).unwrap(), 0x140036bb5);
    }

    #[test]
    fn rejects_an_rva_that_would_wrap_the_address() {
        assert!(ProcessModule::from_base(usize::MAX).resolve_rva(1).is_err());
    }
}
