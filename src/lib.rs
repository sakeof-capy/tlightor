pub mod types;
pub mod algorithm;

#[cfg(test)]
mod tests {
    use crate::algorithm::*;
    use crate::types::*;

    #[test]
    fn manage_empty_input() {
        let roads = [];
        let signal_matrix = manage_roads(&roads);
        assert!(validate_output(&roads, &signal_matrix));
    }

    #[test]
    fn manage_some_one_way_roads() {
        const ROADS_COUTN: usize = 10_000;
        let roads = [Road::OneWay; ROADS_COUTN];
        let signal_matrix = manage_roads(&roads);
        assert!(validate_output(&roads, &signal_matrix));
    }

    #[test]
    fn manage_many_one_way_roads() {
        const ROADS_COUTN: usize = 10_000;
        let roads = [Road::OneWay; ROADS_COUTN];
        let signal_matrix = manage_roads(&roads);
        assert!(validate_output(&roads, &signal_matrix));
    }

    #[test]
    fn manage_some_two_way_roads() {
        const ROADS_COUTN: usize = 10;
        let roads = [Road::TwoWay; ROADS_COUTN];
        let signal_matrix = manage_roads(&roads);
        assert!(validate_output(&roads, &signal_matrix));
    }

    #[test]
    fn manage_many_two_way_roads() {
        const ROADS_COUTN: usize = 10_000;
        let roads = vec![Road::TwoWay; ROADS_COUTN];
        let signal_matrix = manage_roads(&roads);
        assert!(validate_output(&roads, &signal_matrix));
    }

    #[test]
    fn manage_some_any_type_roads() {
        let roads = [
            Road::OneWay,
            Road::TwoWay,
            Road::TwoWay,
            Road::TwoWay,
            Road::OneWay,
            Road::OneWay,
            Road::TwoWay,
            Road::OneWay,
            Road::TwoWay,
            Road::TwoWay,
            Road::TwoWay,
        ];
        let signal_matrix = manage_roads(&roads);
        assert!(validate_output(&roads, &signal_matrix));
    }

    #[test]
    fn manage_many_any_type_roads() {
        const ROADS_COUTN: usize = 10_000;
        let roads = [Road::OneWay, Road::TwoWay]
            .iter()        
            .cloned()      
            .cycle()       
            .take(ROADS_COUTN)  
            .collect::<Vec<Road>>();

        let signal_matrix = manage_roads(&roads);
        assert!(validate_output(&roads, &signal_matrix));
    }
}
