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
    fn remove_from_empty_curve() {
        let mut bezier = Bezier::<f32>::new(2,10);
        bezier.remove(0);
    }

    // This one fails but this is known
    #[test]
    fn detached_handle_does_not_generate_points() {
        let mut bezier = Bezier::<f32>::new(2,10);

        let detached = Handle::detached(point!(0.0,0.0),point!(1.0,1.0),Direction::Forward);
        let mirror = Handle::mirrored(point!(2.0,3.0),point!(2.0,4.0));

        bezier.push(detached);
        bezier.push(mirror);

        assert!(bezier.calculate().len() == 9);
    }

    #[test]
    fn knot_insert_result_is_same() {        
        let mut bezier_two = Bezier::new(20,2);
        let mut bezier_three = Bezier::new(10,3);
        bezier_two.splice(0..0,vec![
            Handle::mirrored(point!(0.0,0.0),point!(1.0,1.0)),
            Handle::mirrored(point!(4.0,1.0),point!(5.0,0.0)),
        ]);
        bezier_three.splice(0..0,vec![
            Handle::mirrored(point!(0.0,0.0),point!(1.0,1.0)),
            Handle::mirrored(point!(4.0,1.0),point!(5.0,0.0)),
        ]);
        let points_no_knot = bezier_two.calculate().to_owned();

        bezier_three.knot_insert(0.5);
        let points_knot = bezier_three.calculate().to_owned();

        let epsilon = 0.0001;
        for i in 0..points_knot.len() {
            assert!((points_knot[i].x - points_no_knot[i].x).abs() < epsilon);
            assert!((points_knot[i].y - points_no_knot[i].y).abs() < epsilon);
        }
    }
}