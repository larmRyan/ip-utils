mod ipv4 {

    struct IPv4Address {
        address: String,
    }

    impl IPv4Address {
        /// Enumerate octets
        fn _enumerate(&self) {
            todo!();
        }

        /// Verify if the address is valid
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

        /// Convert the IPv4Address to decimal notation
        fn to_binary(&self) -> Result<&'static str, ()> {
            if !self.is_valid() {
                return Err(());
            }
            return Ok("TODO");
        }

        /// Convert the IPv4Address to decimal notation
        fn to_decimal(&self) -> Result<&'static str, ()> {
            if !self.is_valid() {
                return Err(());
            }

            return Ok("TODO");
        }

        fn is_loopback(&self) -> bool {
            return false;
        }

        fn is_multicast(&self) -> bool {
            return false;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_valid_address() {
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
        }

        #[test]
        fn test_invalid_address() {
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
    }
}
