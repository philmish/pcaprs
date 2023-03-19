use std::fmt::Display;

pub enum TransportProtocol {
    IPv6HopByHop,
    ICMP,
    IGMP,
    GGP,
    IPinIP,
    ST,
    TCP,
    CBT,
    UDP,
    EGP,
    IGP,
    NVP2,
    UNKNOWN(u8),
}

impl TransportProtocol {

    pub fn new(b: u8) -> Self {
        match b {
            0 => Self::IPv6HopByHop,
            1 => Self::ICMP,
            2 => Self::IGMP,
            3 => Self::GGP,
            4 => Self::IPinIP,
            5 => Self::ST,
            6 => Self::TCP,
            7 => Self::CBT,
            8 => Self::EGP,
            9 => Self::IGP,
            11 => Self::NVP2,
            17 => Self::UDP,
            _ => Self::UNKNOWN(b),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Self::IPv6HopByHop => "IPv6 Hop-By-Hop Option".to_string(),
            Self::ICMP => "ICMP".to_string(),
            Self::IGMP => "Internet Group Management Protocol".to_string(),
            Self::GGP => "Gateway-to-Gateway".to_string(),
            Self::IPinIP => "IP in IP (encapsulated)".to_string(),
            Self::ST => "Internet Stream Protocol".to_string(),
            Self::TCP => "TCP".to_string(),
            Self::CBT => "Core Based Trees".to_string(),
            Self::EGP => "Exterior Gateway".to_string(),
            Self::IGP => "Interior Gateway".to_string(),
            Self::NVP2 => "Network Voice Protocol".to_string(),
            Self::UDP => "UDP".to_string(),
            Self::UNKNOWN(b) => format!("Unknown {}", b),
        }
    }
}

impl Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Protocol: {}", self)
    }
}

impl Clone for TransportProtocol {
    fn clone(&self) -> Self {
        match self {
            Self::IPv6HopByHop => Self::IPv6HopByHop,
            Self::ICMP => Self::ICMP,
            Self::IGMP => Self::IGMP,
            Self::GGP => Self::GGP,
            Self::IPinIP => Self::IPinIP,
            Self::ST => Self::ST,
            Self::TCP => Self::TCP,
            Self::CBT => Self::CBT,
            Self::EGP => Self::EGP,
            Self::IGP => Self::IGP,
            Self::NVP2 => Self::NVP2,
            Self::UDP => Self::UDP,
            Self::UNKNOWN(b) => Self::UNKNOWN(*b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transport_proto() {
        let cases: &[(u8, TransportProtocol)] = &[
            (0, TransportProtocol::IPv6HopByHop),
            (1, TransportProtocol::IPv6HopByHop),
        ];
        

        for c in cases {
        }
    }

}
