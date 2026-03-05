// target definitions for pingray

#[derive(Debug, Clone, Copy)]
pub enum ProbeMethod {
    Ntp,
    HttpHead,
    DnsQuery,
}

#[derive(Debug, Clone)]
pub struct Target {
    pub name: &'static str,
    pub host: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub probe_method: ProbeMethod,
}

#[derive(Debug, Clone)]
pub struct TargetGroup {
    pub name: &'static str,
    pub targets: &'static [Target],
}

pub const CANADIAN_NTP: TargetGroup = TargetGroup {
    name: "Canadian NTP Servers",
    targets: &[
        Target { name: "URegina",      host: "clock.uregina.ca:123",             lat: 50.45, lon: -104.62, probe_method: ProbeMethod::Ntp },
        Target { name: "UCalgary",     host: "subitaneous.cpsc.ucalgary.ca:123", lat: 51.04, lon: -114.07, probe_method: ProbeMethod::Ntp },
        Target { name: "TORIX-2",      host: "ntp2.torix.ca:123",                lat: 43.65, lon: -79.38,  probe_method: ProbeMethod::Ntp },
        Target { name: "TORIX-3",      host: "ntp3.torix.ca:123",                lat: 43.65, lon: -79.38,  probe_method: ProbeMethod::Ntp },
        Target { name: "Acorn-1",      host: "ntp1.acorn-ns.ca:123",             lat: 44.65, lon: -63.57,  probe_method: ProbeMethod::Ntp },
        Target { name: "Acorn-2",      host: "ntp2.acorn-ns.ca:123",             lat: 44.65, lon: -63.57,  probe_method: ProbeMethod::Ntp },
        Target { name: "NYY Sask",     host: "ntp.nyy.ca:123",                   lat: 52.13, lon: -106.67, probe_method: ProbeMethod::Ntp },
        Target { name: "ZAF Regina",   host: "ntp.zaf.ca:123",                   lat: 50.45, lon: -104.62, probe_method: ProbeMethod::Ntp },
        Target { name: "QIX-1",        host: "ntp1.qix.ca:123",                  lat: 45.50, lon: -73.57,  probe_method: ProbeMethod::Ntp },
        Target { name: "QIX-2",        host: "ntp2.qix.ca:123",                  lat: 45.50, lon: -73.57,  probe_method: ProbeMethod::Ntp },
        Target { name: "USask tick",   host: "tick.usask.ca:123",                lat: 52.13, lon: -106.67, probe_method: ProbeMethod::Ntp },
        Target { name: "USask tock",   host: "tock.usask.ca:123",                lat: 52.13, lon: -106.67, probe_method: ProbeMethod::Ntp },
        Target { name: "Wetmore NB",   host: "ntp.wetmore.ca:123",               lat: 45.27, lon: -66.06,  probe_method: ProbeMethod::Ntp },
    ],
};

// 13 root DNS servers (a-m.root-servers.net), coordinates at operator HQ
pub const ROOT_DNS: TargetGroup = TargetGroup {
    name: "Root DNS Servers",
    targets: &[
        Target { name: "A (Verisign)",  host: "a.root-servers.net:53",  lat: 38.88, lon: -77.25,  probe_method: ProbeMethod::DnsQuery }, // Dulles, VA
        Target { name: "B (ISI)",       host: "b.root-servers.net:53",  lat: 33.98, lon: -118.46, probe_method: ProbeMethod::DnsQuery }, // Marina del Rey, CA
        Target { name: "C (Cogent)",    host: "c.root-servers.net:53",  lat: 38.95, lon: -77.34,  probe_method: ProbeMethod::DnsQuery }, // Herndon, VA
        Target { name: "D (UMD)",       host: "d.root-servers.net:53",  lat: 38.99, lon: -76.94,  probe_method: ProbeMethod::DnsQuery }, // College Park, MD
        Target { name: "E (NASA)",      host: "e.root-servers.net:53",  lat: 37.41, lon: -122.06, probe_method: ProbeMethod::DnsQuery }, // Mountain View, CA
        Target { name: "F (ISC)",       host: "f.root-servers.net:53",  lat: 37.79, lon: -122.40, probe_method: ProbeMethod::DnsQuery }, // San Francisco, CA
        Target { name: "G (DISA)",      host: "g.root-servers.net:53",  lat: 38.75, lon: -77.47,  probe_method: ProbeMethod::DnsQuery }, // Columbus, OH (anycast)
        Target { name: "H (USArmy)",    host: "h.root-servers.net:53",  lat: 38.90, lon: -77.02,  probe_method: ProbeMethod::DnsQuery }, // Aberdeen, MD
        Target { name: "I (Netnod)",    host: "i.root-servers.net:53",  lat: 59.33, lon:  18.07,  probe_method: ProbeMethod::DnsQuery }, // Stockholm
        Target { name: "J (Verisign)",  host: "j.root-servers.net:53",  lat: 38.88, lon: -77.25,  probe_method: ProbeMethod::DnsQuery }, // Dulles, VA
        Target { name: "K (RIPE)",      host: "k.root-servers.net:53",  lat: 52.37, lon:   4.90,  probe_method: ProbeMethod::DnsQuery }, // Amsterdam
        Target { name: "L (ICANN)",     host: "l.root-servers.net:53",  lat: 33.98, lon: -118.46, probe_method: ProbeMethod::DnsQuery }, // Los Angeles, CA
        Target { name: "M (WIDE)",      host: "m.root-servers.net:53",  lat: 35.69, lon: 139.69,  probe_method: ProbeMethod::DnsQuery }, // Tokyo
    ],
};

// Country ccTLD authoritative nameservers
pub const CCTLD_DNS: TargetGroup = TargetGroup {
    name: "Country ccTLD Nameservers",
    targets: &[
        // Europe
        Target { name: ".de (DE)",      host: "a.nic.de:53",           lat: 50.12, lon:   8.68, probe_method: ProbeMethod::DnsQuery }, // Frankfurt
        Target { name: ".uk (GB)",      host: "nsa.nic.uk:53",         lat: 51.51, lon:  -0.13, probe_method: ProbeMethod::DnsQuery }, // London
        Target { name: ".fr (FR)",      host: "d.nic.fr:53",           lat: 48.86, lon:   2.35, probe_method: ProbeMethod::DnsQuery }, // Paris
        Target { name: ".nl (NL)",      host: "ns1.dns.nl:53",         lat: 52.37, lon:   4.90, probe_method: ProbeMethod::DnsQuery }, // Amsterdam
        Target { name: ".se (SE)",      host: "a.ns.se:53",            lat: 59.33, lon:  18.07, probe_method: ProbeMethod::DnsQuery }, // Stockholm
        Target { name: ".ch (CH)",      host: "a.nic.ch:53",           lat: 47.38, lon:   8.54, probe_method: ProbeMethod::DnsQuery }, // Zurich
        Target { name: ".it (IT)",      host: "a.dns.it:53",           lat: 43.72, lon:  11.25, probe_method: ProbeMethod::DnsQuery }, // Florence (CNR)
        Target { name: ".es (ES)",      host: "a.nic.es:53",           lat: 40.42, lon:  -3.70, probe_method: ProbeMethod::DnsQuery }, // Madrid
        Target { name: ".pl (PL)",      host: "a-dns.pl:53",           lat: 52.23, lon:  21.01, probe_method: ProbeMethod::DnsQuery }, // Warsaw
        Target { name: ".no (NO)",      host: "nsa.nic.no:53",         lat: 59.91, lon:  10.75, probe_method: ProbeMethod::DnsQuery }, // Oslo
        Target { name: ".fi (FI)",      host: "a.fi:53",               lat: 60.17, lon:  24.94, probe_method: ProbeMethod::DnsQuery }, // Helsinki
        Target { name: ".cz (CZ)",      host: "a.ns.nic.cz:53",       lat: 50.08, lon:  14.43, probe_method: ProbeMethod::DnsQuery }, // Prague
        Target { name: ".ru (RU)",      host: "a.dns.ripn.net:53",     lat: 55.76, lon:  37.62, probe_method: ProbeMethod::DnsQuery }, // Moscow
        Target { name: ".pt (PT)",      host: "a.dns.pt:53",           lat: 38.72, lon:  -9.14, probe_method: ProbeMethod::DnsQuery }, // Lisbon
        // Americas
        Target { name: ".br (BR)",      host: "a.dns.br:53",           lat: -23.55, lon: -46.63, probe_method: ProbeMethod::DnsQuery }, // São Paulo
        Target { name: ".mx (MX)",      host: "m.mx-ns.mx:53",        lat: 19.43, lon: -99.13,  probe_method: ProbeMethod::DnsQuery }, // Mexico City
        Target { name: ".ar (AR)",      host: "a.dns.ar:53",           lat: -34.60, lon: -58.38, probe_method: ProbeMethod::DnsQuery }, // Buenos Aires
        Target { name: ".cl (CL)",      host: "a.nic.cl:53",           lat: -33.45, lon: -70.67, probe_method: ProbeMethod::DnsQuery }, // Santiago
        Target { name: ".co (CO)",      host: "ns1.cointernet.com.co:53", lat: 4.71, lon: -74.07, probe_method: ProbeMethod::DnsQuery }, // Bogotá
        Target { name: ".ca (CA)",      host: "any.ca-servers.ca:53",  lat: 45.42, lon: -75.70,  probe_method: ProbeMethod::DnsQuery }, // Ottawa
        // Asia-Pacific
        Target { name: ".jp (JP)",      host: "a.dns.jp:53",           lat: 35.69, lon: 139.69, probe_method: ProbeMethod::DnsQuery }, // Tokyo
        Target { name: ".cn (CN)",      host: "a.dns.cn:53",           lat: 39.90, lon: 116.40, probe_method: ProbeMethod::DnsQuery }, // Beijing
        Target { name: ".kr (KR)",      host: "b.dns.kr:53",           lat: 37.57, lon: 126.98, probe_method: ProbeMethod::DnsQuery }, // Seoul
        Target { name: ".in (IN)",      host: "ns1.registry.in:53",    lat: 19.08, lon:  72.88, probe_method: ProbeMethod::DnsQuery }, // Mumbai
        Target { name: ".au (AU)",      host: "a.au:53",               lat: -33.87, lon: 151.21, probe_method: ProbeMethod::DnsQuery }, // Sydney
        Target { name: ".nz (NZ)",      host: "ns1.dns.net.nz:53",    lat: -41.29, lon: 174.78, probe_method: ProbeMethod::DnsQuery }, // Wellington
        Target { name: ".sg (SG)",      host: "dsany.sgnic.sg:53",     lat:  1.29, lon: 103.85, probe_method: ProbeMethod::DnsQuery }, // Singapore
        Target { name: ".th (TH)",      host: "a.thains.co.th:53",    lat: 13.76, lon: 100.50, probe_method: ProbeMethod::DnsQuery }, // Bangkok
        Target { name: ".id (ID)",      host: "ns1.id:53",             lat: -6.21, lon: 106.85, probe_method: ProbeMethod::DnsQuery }, // Jakarta
        Target { name: ".tw (TW)",      host: "a.twnic.net.tw:53",    lat: 25.03, lon: 121.57, probe_method: ProbeMethod::DnsQuery }, // Taipei
        // Middle East & Africa
        Target { name: ".za (ZA)",      host: "za-ns.anycast.co.za:53", lat: -33.92, lon: 18.42, probe_method: ProbeMethod::DnsQuery }, // Cape Town
        Target { name: ".ke (KE)",      host: "ns1.kenic.or.ke:53",   lat: -1.29, lon:  36.82, probe_method: ProbeMethod::DnsQuery }, // Nairobi
        Target { name: ".ng (NG)",      host: "a.nic.net.ng:53",      lat:  6.52, lon:   3.38, probe_method: ProbeMethod::DnsQuery }, // Lagos
        Target { name: ".eg (EG)",      host: "ns5.univie.ac.at:53",  lat: 30.04, lon:  31.24, probe_method: ProbeMethod::DnsQuery }, // Cairo
        Target { name: ".il (IL)",      host: "ns1.ns.il:53",         lat: 32.07, lon:  34.78, probe_method: ProbeMethod::DnsQuery }, // Tel Aviv
        Target { name: ".ae (AE)",      host: "ns1.aedns.ae:53",      lat: 25.20, lon:  55.27, probe_method: ProbeMethod::DnsQuery }, // Dubai
        Target { name: ".sa (SA)",      host: "a1.nic.sa:53",         lat: 24.71, lon:  46.68, probe_method: ProbeMethod::DnsQuery }, // Riyadh
        Target { name: ".tr (TR)",      host: "ns1.nic.tr:53",        lat: 39.93, lon:  32.87, probe_method: ProbeMethod::DnsQuery }, // Ankara
    ],
};

// Global NTP pool servers (country pools)
pub const GLOBAL_NTP: TargetGroup = TargetGroup {
    name: "Global NTP Pool",
    targets: &[
        // North America
        Target { name: "US Pool",       host: "us.pool.ntp.org:123",    lat: 39.83, lon: -98.58,  probe_method: ProbeMethod::Ntp },
        Target { name: "CA Pool",       host: "ca.pool.ntp.org:123",    lat: 56.13, lon: -106.35, probe_method: ProbeMethod::Ntp },
        Target { name: "MX Pool",       host: "mx.pool.ntp.org:123",    lat: 23.63, lon: -102.55, probe_method: ProbeMethod::Ntp },
        // South America
        Target { name: "BR Pool",       host: "br.pool.ntp.org:123",    lat: -14.24, lon: -51.93, probe_method: ProbeMethod::Ntp },
        Target { name: "AR Pool",       host: "ar.pool.ntp.org:123",    lat: -38.42, lon: -63.62, probe_method: ProbeMethod::Ntp },
        Target { name: "CL Pool",       host: "cl.pool.ntp.org:123",    lat: -35.68, lon: -71.54, probe_method: ProbeMethod::Ntp },
        // Europe
        Target { name: "DE Pool",       host: "de.pool.ntp.org:123",    lat: 51.17, lon:  10.45, probe_method: ProbeMethod::Ntp },
        Target { name: "UK Pool",       host: "uk.pool.ntp.org:123",    lat: 55.38, lon:  -3.44, probe_method: ProbeMethod::Ntp },
        Target { name: "FR Pool",       host: "fr.pool.ntp.org:123",    lat: 46.23, lon:   2.21, probe_method: ProbeMethod::Ntp },
        Target { name: "NL Pool",       host: "nl.pool.ntp.org:123",    lat: 52.13, lon:   5.29, probe_method: ProbeMethod::Ntp },
        Target { name: "SE Pool",       host: "se.pool.ntp.org:123",    lat: 60.13, lon:  18.64, probe_method: ProbeMethod::Ntp },
        Target { name: "IT Pool",       host: "it.pool.ntp.org:123",    lat: 41.87, lon:  12.57, probe_method: ProbeMethod::Ntp },
        Target { name: "ES Pool",       host: "es.pool.ntp.org:123",    lat: 40.46, lon:  -3.75, probe_method: ProbeMethod::Ntp },
        Target { name: "PL Pool",       host: "pl.pool.ntp.org:123",    lat: 51.92, lon:  19.15, probe_method: ProbeMethod::Ntp },
        Target { name: "RU Pool",       host: "ru.pool.ntp.org:123",    lat: 61.52, lon: 105.32, probe_method: ProbeMethod::Ntp },
        Target { name: "CH Pool",       host: "ch.pool.ntp.org:123",    lat: 46.82, lon:   8.23, probe_method: ProbeMethod::Ntp },
        Target { name: "NO Pool",       host: "no.pool.ntp.org:123",    lat: 60.47, lon:   8.47, probe_method: ProbeMethod::Ntp },
        // Asia-Pacific
        Target { name: "JP Pool",       host: "jp.pool.ntp.org:123",    lat: 36.20, lon: 138.25, probe_method: ProbeMethod::Ntp },
        Target { name: "CN Pool",       host: "cn.pool.ntp.org:123",    lat: 35.86, lon: 104.20, probe_method: ProbeMethod::Ntp },
        Target { name: "KR Pool",       host: "kr.pool.ntp.org:123",    lat: 35.91, lon: 127.77, probe_method: ProbeMethod::Ntp },
        Target { name: "IN Pool",       host: "in.pool.ntp.org:123",    lat: 20.59, lon:  78.96, probe_method: ProbeMethod::Ntp },
        Target { name: "AU Pool",       host: "au.pool.ntp.org:123",    lat: -25.27, lon: 133.78, probe_method: ProbeMethod::Ntp },
        Target { name: "NZ Pool",       host: "nz.pool.ntp.org:123",    lat: -40.90, lon: 174.89, probe_method: ProbeMethod::Ntp },
        Target { name: "SG Pool",       host: "sg.pool.ntp.org:123",    lat:  1.35, lon: 103.82, probe_method: ProbeMethod::Ntp },
        Target { name: "TH Pool",       host: "th.pool.ntp.org:123",    lat: 15.87, lon: 100.99, probe_method: ProbeMethod::Ntp },
        Target { name: "TW Pool",       host: "tw.pool.ntp.org:123",    lat: 23.70, lon: 120.96, probe_method: ProbeMethod::Ntp },
        Target { name: "ID Pool",       host: "id.pool.ntp.org:123",    lat: -0.79, lon: 113.92, probe_method: ProbeMethod::Ntp },
        // Middle East & Africa
        Target { name: "ZA Pool",       host: "za.pool.ntp.org:123",    lat: -30.56, lon:  22.94, probe_method: ProbeMethod::Ntp },
        Target { name: "IL Pool",       host: "il.pool.ntp.org:123",    lat: 31.05, lon:  34.85, probe_method: ProbeMethod::Ntp },
        Target { name: "TR Pool",       host: "tr.pool.ntp.org:123",    lat: 38.96, lon:  35.24, probe_method: ProbeMethod::Ntp },
        Target { name: "KE Pool",       host: "ke.pool.ntp.org:123",    lat: -0.02, lon:  37.91, probe_method: ProbeMethod::Ntp },
    ],
};

// Major Internet Exchange Points
pub const IXP_SERVERS: TargetGroup = TargetGroup {
    name: "Internet Exchange Points",
    targets: &[
        Target { name: "DE-CIX",       host: "de-cix.net",            lat: 50.12, lon:   8.68, probe_method: ProbeMethod::HttpHead }, // Frankfurt
        Target { name: "AMS-IX",        host: "ams-ix.net",            lat: 52.37, lon:   4.90, probe_method: ProbeMethod::HttpHead }, // Amsterdam
        Target { name: "LINX",          host: "linx.net",              lat: 51.51, lon:  -0.13, probe_method: ProbeMethod::HttpHead }, // London
        Target { name: "IX.br",         host: "ix.br",                 lat: -23.55, lon: -46.63, probe_method: ProbeMethod::HttpHead }, // São Paulo
        Target { name: "JPNAP",         host: "jpnap.net",             lat: 35.69, lon: 139.69, probe_method: ProbeMethod::HttpHead }, // Tokyo
        Target { name: "Equinix IX",    host: "equinix.com",           lat: 37.39, lon: -122.03, probe_method: ProbeMethod::HttpHead }, // Sunnyvale
        Target { name: "HKIX",          host: "hkix.net",              lat: 22.28, lon: 114.16, probe_method: ProbeMethod::HttpHead }, // Hong Kong
        Target { name: "MSK-IX",        host: "msk-ix.ru",             lat: 55.76, lon:  37.62, probe_method: ProbeMethod::HttpHead }, // Moscow
        Target { name: "France-IX",     host: "franceix.net",          lat: 48.86, lon:   2.35, probe_method: ProbeMethod::HttpHead }, // Paris
        Target { name: "TORIX",         host: "torix.ca",              lat: 43.65, lon: -79.38, probe_method: ProbeMethod::HttpHead }, // Toronto
        Target { name: "SIX",           host: "seattleix.net",         lat: 47.61, lon: -122.33, probe_method: ProbeMethod::HttpHead }, // Seattle
        Target { name: "NYIIX",         host: "nyiix.net",             lat: 40.71, lon: -74.01, probe_method: ProbeMethod::HttpHead }, // New York
        Target { name: "SwissIX",       host: "swissix.ch",            lat: 47.38, lon:   8.54, probe_method: ProbeMethod::HttpHead }, // Zurich
        Target { name: "NAPAfrica",     host: "napafrica.net",         lat: -26.20, lon:  28.05, probe_method: ProbeMethod::HttpHead }, // Johannesburg
        Target { name: "MIX",           host: "mix-it.net",            lat: 45.46, lon:   9.19, probe_method: ProbeMethod::HttpHead }, // Milan
        Target { name: "SGIX",          host: "sgix.sg",               lat:  1.29, lon: 103.85, probe_method: ProbeMethod::HttpHead }, // Singapore
        Target { name: "ANIX",          host: "anix.net.au",           lat: -33.87, lon: 151.21, probe_method: ProbeMethod::HttpHead }, // Sydney
        Target { name: "QIX",           host: "qix.ca",                lat: 45.50, lon: -73.57, probe_method: ProbeMethod::HttpHead }, // Montreal
        Target { name: "KINX",          host: "kinx.net",              lat: 37.57, lon: 126.98, probe_method: ProbeMethod::HttpHead }, // Seoul
        Target { name: "ESPANIX",       host: "espanix.net",           lat: 40.42, lon:  -3.70, probe_method: ProbeMethod::HttpHead }, // Madrid
    ],
};
