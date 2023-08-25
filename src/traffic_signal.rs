enum TrafficSignal {
    Red,
    Yellow,
    Green,
}

trait SignalDuration {
    fn duration(&self) -> u32;
}

impl SignalDuration for TrafficSignal {
    fn duration(&self) -> u32 {
        match self {
            TrafficSignal::Red => 30,
            TrafficSignal::Yellow => 5,
            TrafficSignal::Green => 45,
        }
    }
}

