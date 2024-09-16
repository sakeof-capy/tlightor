use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TrafficLightsSignal {
    Green,
    Red,
}

impl TrafficLightsSignal {
    pub fn allows_driving(&self) -> bool {
        *self == Self::Green
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Road {
    OneWay,
    TwoWay,
}

impl Road {
    pub fn to_road_signal(self, signal: TrafficLightsSignal) -> RoadSignal {
        match self {
            Self::OneWay => RoadSignal::OneWay(signal),
            Self::TwoWay => RoadSignal::TwoWay(signal),
        }
    }
}

#[derive(Clone, Copy)]
pub enum RoadSignal {
    OneWay(TrafficLightsSignal),
    TwoWay(TrafficLightsSignal),
}

impl Display for RoadSignal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let signal: TrafficLightsSignal = (*self).into();
        let road_type: Road = (*self).into();
        write!(f, "{:?} road with {:?} signal.", road_type, signal)
    }
}

impl From<RoadSignal> for TrafficLightsSignal {
    fn from(val: RoadSignal) -> Self {
        match val {
            RoadSignal::OneWay(signal) => signal,
            RoadSignal::TwoWay(signal) => signal,
        }
    }
}

impl From<RoadSignal> for Road {
    fn from(val: RoadSignal) -> Self {
        match val {
            RoadSignal::OneWay(_) => Road::OneWay,
            RoadSignal::TwoWay(_) => Road::TwoWay,
        }
    }
}

pub type SignalsMatrix = Vec<Vec<RoadSignal>>;
