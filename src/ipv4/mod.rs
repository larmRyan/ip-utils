mod ipv4 {

    // TODO: FQDN and reverse DNS lookups

    struct IPv4Address {
        address: String,
    }

    impl IPv4Address {
        /// Return true if this address is a valid IPv4 address
        /// and false otherwise
        fn is_valid(&self) -> bool {
            let octets = self.address.split(".");
            for octet in octets {
                let o: i32 = octet.parse().expect("Expected number");
                if o > 255 || o < 0 {
                    return false;
                }
            }
            return true;
        }

        /// Return true if this address is a loopback address
        /// and false otherwise
        fn is_loopback(&self) -> bool {
            return self.address.eq("127.0.0.1");
        }

        /// Return true if this address is a multicast address
        /// and false otherwise
        fn is_multicast(&self) -> bool {
            return false;
        }
    }

    /// Represents an IPv4 network
    struct IPv4Network {
        host: String,

        /// The network prefix
        prefix: u64,
    }

    impl IPv4Network {
        /// Return true if this network contains the given address
        /// and false otherwise
        fn contains(&self, address: &IPv4Address) -> bool {
            return false;
        }

        /// Return all possible hosts within this network
        fn get_all_hosts(&self) -> Vec<String> {
            todo!();
        }

        /// Return all addresses within the intersect of this network
        /// and another network
        fn get_intersect(&self, other: IPv4Network) {
            todo!();
        }

        fn subnet_mask(&self) {
            todo!();
        }

        fn to_cidr(&self) {
            todo!();
        }

        // TODO Get class of the network
        fn class(&self) {
            todo!();
        }
    }

    // TODO Clean up and simplify tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_valid() {
            let address = IPv4Address {
                address: String::from("127.0.0.1"),
            };
            assert!(address.is_valid());

            let address = IPv4Address {
                address: String::from("0.0.0.0"),
            };
            assert!(address.is_valid());

            let address = IPv4Address {
                address: String::from("255.255.255.255"),
            };
            assert!(address.is_valid());

            let address = IPv4Address {
                address: String::from("256.100.0.2"),
            };
            assert!(!address.is_valid());

            let address = IPv4Address {
                address: String::from("255.255.255.300"),
            };
            assert!(!address.is_valid());

            let address = IPv4Address {
                address: String::from("0.0.0.256"),
            };
            assert!(!address.is_valid());
        }

        #[test]
        fn test_is_loopback() {
            let address = IPv4Address {
                address: String::from("127.0.0.1"),
            };
            assert!(address.is_loopback());

            let address = IPv4Address {
                address: String::from("127.0.0.0"),
            };
            assert!(!address.is_loopback());
        }
    }
}
