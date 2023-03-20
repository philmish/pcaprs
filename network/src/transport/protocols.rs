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

        assert!(matches!(TransportProtocol::new(0), TransportProtocol::IPv6HopByHop));
        assert!(matches!(TransportProtocol::new(1), TransportProtocol::ICMP));
        assert!(matches!(TransportProtocol::new(2), TransportProtocol::IGMP));
        assert!(matches!(TransportProtocol::new(3), TransportProtocol::GGP));
        assert!(matches!(TransportProtocol::new(4), TransportProtocol::IPinIP));
        assert!(matches!(TransportProtocol::new(5), TransportProtocol::ST));
        assert!(matches!(TransportProtocol::new(6), TransportProtocol::TCP));
        assert!(matches!(TransportProtocol::new(7), TransportProtocol::CBT));
        assert!(matches!(TransportProtocol::new(8), TransportProtocol::EGP));
        assert!(matches!(TransportProtocol::new(9), TransportProtocol::IGP));
        assert!(matches!(TransportProtocol::new(11), TransportProtocol::NVP2));
        assert!(matches!(TransportProtocol::new(17), TransportProtocol::UDP));
        assert!(matches!(TransportProtocol::new(18), TransportProtocol::UNKNOWN(18)));
        
    }

    #[test]
    fn test_clone_transport_proto() {

        assert!(matches!(TransportProtocol::IPv6HopByHop.clone(), TransportProtocol::IPv6HopByHop));
        assert!(matches!(TransportProtocol::ICMP.clone(), TransportProtocol::ICMP));
        assert!(matches!(TransportProtocol::IGMP.clone(), TransportProtocol::IGMP));
        assert!(matches!(TransportProtocol::GGP.clone(), TransportProtocol::GGP));
        assert!(matches!(TransportProtocol::IPinIP.clone(), TransportProtocol::IPinIP));
        assert!(matches!(TransportProtocol::ST.clone(), TransportProtocol::ST));
        assert!(matches!(TransportProtocol::TCP.clone(), TransportProtocol::TCP));
        assert!(matches!(TransportProtocol::CBT.clone(), TransportProtocol::CBT));
        assert!(matches!(TransportProtocol::EGP.clone(), TransportProtocol::EGP));
        assert!(matches!(TransportProtocol::IGP.clone(), TransportProtocol::IGP));
        assert!(matches!(TransportProtocol::NVP2.clone(), TransportProtocol::NVP2));
        assert!(matches!(TransportProtocol::UDP.clone(), TransportProtocol::UDP));
        assert!(matches!(TransportProtocol::UNKNOWN(18).clone(), TransportProtocol::UNKNOWN(18)));
        
    }

    #[test]
    fn test_transport_proto_to_str() {

        assert_eq!(TransportProtocol::IPv6HopByHop.to_str(), "IPv6 Hop-By-Hop Option".to_string());
        assert_eq!(TransportProtocol::ICMP.to_str(), "ICMP".to_string());
        assert_eq!(TransportProtocol::IGMP.to_str(), "Internet Group Management Protocol".to_string());
        assert_eq!(TransportProtocol::GGP.to_str(), "Gateway-to-Gateway".to_string());
        assert_eq!(TransportProtocol::IPinIP.to_str(), "IP in IP (encapsulated)".to_string());
        assert_eq!(TransportProtocol::ST.to_str(), "Internet Stream Protocol".to_string());
        assert_eq!(TransportProtocol::TCP.to_str(), "TCP".to_string());
        assert_eq!(TransportProtocol::CBT.to_str(), "Core Based Trees".to_string());
        assert_eq!(TransportProtocol::EGP.to_str(), "Exterior Gateway".to_string());
        assert_eq!(TransportProtocol::IGP.to_str(), "Interior Gateway".to_string());
        assert_eq!(TransportProtocol::NVP2.to_str(), "Network Voice Protocol".to_string());
        assert_eq!(TransportProtocol::UDP.to_str(), "UDP".to_string());
        assert_eq!(TransportProtocol::UNKNOWN(18).to_str(), format!("Unknown {}", 18));
        
    }

}
