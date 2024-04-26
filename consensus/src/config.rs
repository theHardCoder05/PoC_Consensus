use crypto::PublicKey;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

pub type Stake = u32;
pub type EpochNumber = u128;

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    pub timeout_delay: u64,
    pub sync_retry_delay: u64,
}


impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            timeout_delay: 1000,
            sync_retry_delay: 1000,
        }
    }
}

impl Parameters {
    pub fn log(&self) {
        info!("Timeout delay: {}", self.timeout_delay);
        info!("Sync retry delay: {}", self.sync_retry_delay);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Authority {
    pub stake: Stake,
    pub address: SocketAddr,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Committee {
    pub authorities: HashMap<PublicKey, Authority>,
    pub epoch: EpochNumber,
}   


    impl Committee {
        pub fn new(info: Vec<(PublicKey, Stake, SocketAddr)>, epoch: EpochNumber) -> Self {
            Self {
            
            authorities: info.into_iter()
            .map(|(name, stake, address)|{
                    let authority = Authority { stake, address };
                    (name, authority)
            }).collect(),
            epoch,
            }
        }

    /// Returns the number of authorities in the committee.
    ///
    /// # Example
    ///
    /// ```
    /// # use your_crate::Committee;
    /// let committee = Committee::new(/* parameters */);
    /// let size = committee.size();
    /// println!("The size of the committee is {}", size);
    /// ```
    ///
    /// # Returns
    ///
    /// The number of authorities in the committee.
    pub fn size(&self) -> usize {
        self.authorities.len()
    }

    pub fn stake(&self, name: &PublicKey) -> Stake {
        self.authorities.get(name).map_or_else(|| 0,|x:&Authority| x.stake)
    }

    pub fn qourum_threshold(&self) -> Stake {
        let total_votes = self.authorities.values().map(|x | x.stake).sum::<Stake>();
        2 * total_votes / 3 + 1
    }

    pub fn address(&self, name: &PublicKey) -> Option<SocketAddr> {
        self.authorities.get(name).map(|x| x.address)
    }

    pub fn boardcast_addresses(&self, myself: &PublicKey) -> Vec<PublicKey, SocketAddr> {
        self.authorities.iter().filter(|(name, _)| name != &myself)
        .map(|(name, x)| (*name , x.address)).collect()
    }

}
