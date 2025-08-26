use bech32::primitives::decode::CheckedHrpstring;
use bech32::{encode, Bech32, Hrp};
use cosmwasm_std::testing::MockApi;
use cosmwasm_std::{
    Addr, Api, CanonicalAddr, RecoverPubkeyError, StdError, StdResult, VerificationError,
};
use sha2::{Digest, Sha256};

/// Implementation of the `Api` trait that uses [`Bech32`] format
/// for humanizing canonical addresses.
///
/// [`Bech32`]:https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki
pub struct MockApiBech32 {
    api: MockApi,
    prefix: String,
}

impl MockApiBech32 {
    /// Returns `Api` implementation that uses specified prefix
    /// to generate addresses in **Bech32** format.
    ///
    /// # Example
    ///
    /// ```
    /// use cw_multi_test::addons::MockApiBech32;
    ///
    /// let api = MockApiBech32::new("juno");
    /// let addr = api.addr_make("creator");
    /// assert_eq!(addr.as_str(),
    ///            "juno1h34lmpywh4upnjdg90cjf4j70aee6z8qqfspugamjp42e4q28kqsksmtyp");
    /// ```
    pub fn new(prefix: &str) -> Self {
        Self {
            api: MockApi::default(),
            prefix: prefix.to_string(),
        }
    }
    pub fn with_prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Does basic validation of the number of bytes in a canonical address
    fn validate_length(bytes: &[u8]) -> StdResult<()> {
        match bytes.len() {
            1..=255 => Ok(()),
            _ => Err(StdError::msg("Invalid canonical address length")),
        }
    }
}

impl Api for MockApiBech32 {
    /// Takes a human readable address in **Bech32** format and checks if it is valid.
    ///
    /// If the validation succeeds, an `Addr` containing the same string as the input is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use cosmwasm_std::Api;
    /// use cw_multi_test::addons::MockApiBech32;
    ///
    /// let api = MockApiBech32::new("juno");
    /// let addr = api.addr_make("creator");
    /// assert_eq!(api.addr_validate(addr.as_str()).unwrap().as_str(),
    ///            addr.as_str());
    /// ```
    fn addr_validate(&self, input: &str) -> StdResult<Addr> {
        self.addr_humanize(&self.addr_canonicalize(input)?)
    }

    /// Takes a human readable address in **Bech32** format and returns
    /// a canonical binary representation of it.
    ///
    /// # Example
    ///
    /// ```
    /// use cosmwasm_std::Api;
    /// use cw_multi_test::addons::MockApiBech32;
    ///
    /// let api = MockApiBech32::new("juno");
    /// let addr = api.addr_make("creator");
    /// assert_eq!(api.addr_canonicalize(addr.as_str()).unwrap().to_string(),
    ///            "BC6BFD848EBD7819C9A82BF124D65E7F739D08E002601E23BB906AACD40A3D81");
    /// ```
    fn addr_canonicalize(&self, input: &str) -> StdResult<CanonicalAddr> {
        let hrp_str = CheckedHrpstring::new::<Bech32>(input)?;

        if !hrp_str
            .hrp()
            .as_bytes()
            .eq_ignore_ascii_case(self.prefix.as_bytes())
        {
            return Err(StdError::msg("Wrong bech32 prefix"));
        }

        let bytes: Vec<u8> = hrp_str.byte_iter().collect();
        validate_length(&bytes)?;
        Ok(bytes.into())
    }

    /// Takes a canonical address and returns a human readable address in **Bech32** format.
    ///
    /// This is the inverse operation of [`addr_canonicalize`].
    ///
    /// [`addr_canonicalize`]: MockApiBech32::addr_canonicalize
    ///
    /// # Example
    ///
    /// ```
    /// use cosmwasm_std::Api;
    /// use cw_multi_test::addons::MockApiBech32;
    ///
    /// let api = MockApiBech32::new("juno");
    /// let addr = api.addr_make("creator");
    /// let canonical_addr = api.addr_canonicalize(addr.as_str()).unwrap();
    /// assert_eq!(api.addr_humanize(&canonical_addr).unwrap().as_str(),
    ///            addr.as_str());
    /// ```
    fn addr_humanize(&self, canonical: &CanonicalAddr) -> StdResult<Addr> {
        validate_length(canonical.as_ref())?;

        let prefix = Hrp::parse(&self.prefix)?;
        Ok(encode::<Bech32>(prefix, canonical.as_slice()).map(Addr::unchecked)?)
    }

    fn secp256k1_verify(
        &self,
        message_hash: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        self.api
            .secp256k1_verify(message_hash, signature, public_key)
    }

    fn secp256k1_recover_pubkey(
        &self,
        message_hash: &[u8],
        signature: &[u8],
        recovery_param: u8,
    ) -> Result<Vec<u8>, RecoverPubkeyError> {
        self.api
            .secp256k1_recover_pubkey(message_hash, signature, recovery_param)
    }

    fn ed25519_verify(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        self.api.ed25519_verify(message, signature, public_key)
    }

    fn ed25519_batch_verify(
        &self,
        messages: &[&[u8]],
        signatures: &[&[u8]],
        public_keys: &[&[u8]],
    ) -> Result<bool, VerificationError> {
        self.api
            .ed25519_batch_verify(messages, signatures, public_keys)
    }

    fn debug(&self, message: &str) {
        self.api.debug(message)
    }
}

impl MockApiBech32 {
    /// Returns an address in **Bech32** format, built from provided input string.
    ///
    /// # Example
    ///
    /// ```
    /// use cw_multi_test::addons::MockApiBech32;
    ///
    /// let api = MockApiBech32::new("juno");
    /// let addr = api.addr_make("creator");
    /// assert_eq!(addr.as_str(),
    ///            "juno1h34lmpywh4upnjdg90cjf4j70aee6z8qqfspugamjp42e4q28kqsksmtyp");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics when generating a valid address in **Bech32**
    /// format is not possible, especially when prefix is too long or empty.
    pub fn addr_make(&self, input: &str) -> Addr {
        let digest = Sha256::digest(input);

        let prefix = match Hrp::parse(&self.prefix) {
            Ok(prefix) => prefix,
            Err(reason) => panic!("Generating address failed with reason: {reason}"),
        };

        match encode::<Bech32>(prefix, &digest) {
            Ok(address) => Addr::unchecked(address),
            Err(reason) => panic!("Generating address failed with reason: {reason}"),
        }
    }
}

fn validate_length(bytes: &[u8]) -> StdResult<()> {
    match bytes.len() {
        1..=255 => Ok(()),
        _ => Err(StdError::msg("Invalid canonical address length")),
    }
}
