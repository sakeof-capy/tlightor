use crate::types::*;

pub fn manage_roads(roads: &[Road]) -> SignalsMatrix {
    let mut signals_matrix = SignalsMatrix::with_capacity(roads.len());
    let default_road_row = roads
        .iter() 
        .map(|r| r.to_road_signal(TrafficLightsSignal::Red)) 
        .collect::<Vec<RoadSignal>>();

    for (index, road) in roads.iter().enumerate() {
        let mut signals_row = default_road_row.clone();
        let road_signal = (*road).to_road_signal(TrafficLightsSignal::Green);
        signals_row[index] = road_signal;
        signals_matrix.push(signals_row);
    }

    signals_matrix
}

pub fn validate_output(roads: &[Road], signal_matrix: &SignalsMatrix) -> bool {

    let count = roads.len() as u128;
    let mut road_index_sum: u128 = count * (count + 1) / 2; // sum of all road (indices + 1)

    for signals in signal_matrix {
        if roads.len() != signals.len() {
            return false;
        }

        let mut has_someone_driving = false;

        for (index, &road_signal) in signals.into_iter().enumerate() {
            let signal: TrafficLightsSignal = road_signal.into();
            
            if !signal.allows_driving() {
                continue;
            }

            if has_someone_driving {
                return false;
            }

            has_someone_driving = true;
            road_index_sum -= index as u128 + 1;
        }
    }

    road_index_sum == 0
}

