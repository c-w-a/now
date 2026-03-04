// target definitions for pingray

#[derive(Debug, Clone)]
pub struct Target {
    pub name: &'static str,
    pub host: &'static str,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone)]
pub struct TargetGroup {
    pub name: &'static str,
    pub targets: &'static [Target],
}

pub const CANADIAN_NTP: TargetGroup = TargetGroup {
    name: "Canadian NTP Servers",
    targets: &[
        Target { name: "URegina",      host: "clock.uregina.ca:123",             lat: 50.45, lon: -104.62 },
        Target { name: "UCalgary",     host: "subitaneous.cpsc.ucalgary.ca:123", lat: 51.04, lon: -114.07 },
        Target { name: "TORIX-2",      host: "ntp2.torix.ca:123",                lat: 43.65, lon: -79.38 },
        Target { name: "TORIX-3",      host: "ntp3.torix.ca:123",                lat: 43.65, lon: -79.38 },
        Target { name: "Acorn-1",      host: "ntp1.acorn-ns.ca:123",             lat: 44.65, lon: -63.57 },
        Target { name: "Acorn-2",      host: "ntp2.acorn-ns.ca:123",             lat: 44.65, lon: -63.57 },
        Target { name: "NYY Sask",     host: "ntp.nyy.ca:123",                   lat: 52.13, lon: -106.67 },
        Target { name: "ZAF Regina",   host: "ntp.zaf.ca:123",                   lat: 50.45, lon: -104.62 },
        Target { name: "QIX-1",        host: "ntp1.qix.ca:123",                  lat: 45.50, lon: -73.57 },
        Target { name: "QIX-2",        host: "ntp2.qix.ca:123",                  lat: 45.50, lon: -73.57 },
        Target { name: "USask tick",   host: "tick.usask.ca:123",                lat: 52.13, lon: -106.67 },
        Target { name: "USask tock",   host: "tock.usask.ca:123",                lat: 52.13, lon: -106.67 },
        Target { name: "Wetmore NB",   host: "ntp.wetmore.ca:123",               lat: 45.27, lon: -66.06 },
    ],
};
