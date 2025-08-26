use crate::wasm_emulation::query::gas::{GAS_COST_CANONICALIZE, GAS_COST_HUMANIZE};
use bech32::{Bech32, Hrp};
use cosmwasm_std::Addr;
use cosmwasm_vm::{BackendApi, BackendError, GasInfo};
use sha2::{Digest, Sha256};
use std::ops::AddAssign;

const SHORT_CANON_LEN: usize = 20;
const LONG_CANON_LEN: usize = 32;

pub fn bytes_from_bech32(address: &str, prefix: &str) -> Result<Vec<u8>, BackendError> {
    if address.is_empty() {
        return Err(BackendError::Unknown {
            msg: "empty address string is not allowed".to_string(),
        });
    }

    let (hrp, data) = bech32::decode(address).map_err(|e| BackendError::Unknown {
        msg: format!("Invalid Bech32 address : Err {}", e),
    })?;
    if hrp != Hrp::parse(prefix).unwrap() {
        return Err(BackendError::Unknown {
            msg: format!("invalid Bech32 prefix; expected {}, got {}", prefix, hrp),
        });
    }

    Ok(data.into_iter().collect::<Vec<u8>>())
}

pub const MAX_PREFIX_CHARS: usize = 10;
// Prefixes are limited to MAX_PREFIX_CHARS chars
// This allows one to specify a string prefix and still implement Copy
#[derive(Clone, Copy)]
pub struct RealApi {
    pub prefix: [char; MAX_PREFIX_CHARS],
}

impl RealApi {
    pub fn new(prefix: &str) -> RealApi {
        if prefix.len() > MAX_PREFIX_CHARS {
            panic!("More chars in the prefix than {}", MAX_PREFIX_CHARS);
        }

        let mut api_prefix = ['\0'; 10];
        for (i, c) in prefix.chars().enumerate() {
            api_prefix[i] = c;
        }
        Self { prefix: api_prefix }
    }

    pub fn get_prefix(&self) -> String {
        let mut prefix = Vec::new();

        for &c in self.prefix.iter() {
            if c != '\0' {
                prefix.push(c);
            }
        }
        prefix.iter().collect()
    }

    pub fn next_address(&self, count: usize) -> Addr {
        let mut canon = format!("ADDRESS_{}", count).as_bytes().to_vec();
        canon.resize(SHORT_CANON_LEN, 0);
        Addr::unchecked(self.addr_humanize(&canon).0.unwrap())
    }

    pub fn next_contract_address(&self, count: usize) -> Addr {
        let mut canon = format!("CONTRACT_{}", count).as_bytes().to_vec();
        canon.resize(LONG_CANON_LEN, 0);
        Addr::unchecked(self.addr_humanize(&canon).0.unwrap())
    }
}

impl BackendApi for RealApi {
    fn addr_validate(&self, input: &str) -> cosmwasm_vm::BackendResult<()> {
        let (canon, mut gas_cost) = self.addr_canonicalize(input);

        if let Err(e) = canon {
            return (Err(e), gas_cost);
        }
        let canon = canon.unwrap();

        let (new_human, human_gas_cost) = self.addr_humanize(&canon);

        if let Err(e) = new_human {
            gas_cost.add_assign(human_gas_cost);
            return (Err(e), gas_cost);
        }
        let new_human = new_human.unwrap();

        if input == new_human {
            (Ok(()), gas_cost)
        } else {
            (
                Err(BackendError::user_err(format!(
                    "Address invalid : {}",
                    input
                ))),
                gas_cost,
            )
        }
    }

    fn addr_canonicalize(&self, human: &str) -> cosmwasm_vm::BackendResult<Vec<u8>> {
        let gas_cost = GasInfo::with_externally_used(GAS_COST_CANONICALIZE);
        if human.trim().is_empty() {
            return (
                Err(BackendError::Unknown {
                    msg: "empty address string is not allowed".to_string(),
                }),
                gas_cost,
            );
        }

        (bytes_from_bech32(human, &self.get_prefix()), gas_cost)
    }

    fn addr_humanize(&self, canonical: &[u8]) -> cosmwasm_vm::BackendResult<String> {
        let gas_cost = GasInfo::with_externally_used(GAS_COST_HUMANIZE);

        if canonical.len() != SHORT_CANON_LEN && canonical.len() != LONG_CANON_LEN {
            return (
                Err(BackendError::Unknown {
                    msg: "Canon address doesn't have the right length".to_string(),
                }),
                gas_cost,
            );
        }

        if canonical.is_empty() {
            return (Ok("".to_string()), gas_cost);
        }

        let human = bech32::encode::<Bech32>(Hrp::parse(&self.get_prefix()).unwrap(), &canonical)
            .map_err(|e| BackendError::Unknown { msg: e.to_string() });

        (human, gas_cost)
    }
}

#[cfg(test)]
mod test {
    use super::RealApi;

    #[test]
    fn prefix() {
        let prefix = "migaloo";

        let api = RealApi::new(prefix);

        let final_prefix = api.get_prefix();
        assert_eq!(prefix, final_prefix);
    }

    #[test]
    #[should_panic]
    fn too_long_prefix() {
        let prefix = "migaloowithotherchars";
        RealApi::new(prefix);
    }
}
