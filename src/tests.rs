#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn handle_operations_do_not_panic() {
        let mut bezier = Bezier::new(2,10);
        let handle = Handle::mirrored(point!(0.0,0.0),point!(1.0,1.0));

        bezier.insert(0,handle.clone());
        bezier.push(handle.clone());
        bezier.knot_insert(0.5);
        bezier.drain(0..1);
        bezier.splice(0..0, vec![handle.clone(),handle.clone()]);
    }

    #[test]
    fn wrong_inputs_return_err() {

    }
    
    #[test]
    fn handle_creation_continuity() {

    }
    
    #[test]
    fn knot_insert_result_is_same() {        
        let mut bezier = Bezier::new(10,3);
        bezier.splice(0..0,vec![
            Handle::mirrored(point!(0.0,0.0),point!(1.0,1.0)),
            Handle::mirrored(point!(4.0,1.0),point!(5.0,0.0)),
        ]);
        let points_before = bezier.calculate().to_owned();

        bezier.knot_insert(0.5);
        let points_after = bezier.calculate().to_owned();

        assert_eq!(points_before,points_after);
    }
}