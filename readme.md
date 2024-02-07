# cubic-bezier

This crate provides functionality for working with cubic Bezier curves, such as creating, manipulating, and calculating points along cubic Bezier curves.

## Features

- Create cubic Bezier curves with control handles.
- Manipulate curves by adding, removing, or modifying control handles.
- Add control handles without modifying the curve.
- Calculate points along the curve with customizable detail.
- Caches calculated points.

## Example

```rust
use cubic_bezier::{point, Bezier, Handle};

let mut bezier = Bezier::new(10, 2);
bezier.push(Handle::mirrored(point!(-1.0, 1.0), point!(0.0, 0.0)));
bezier.push(Handle::mirrored(point!(1.0, 1.0), point!(2.0, 0.0)));

let points = bezier.calculate();
```

## Usage

Creating a new Bezier curve is done with the `new` method. You specify the level of detail and an estimation of the number of handles that will be added. 

```rust
let mut bezier = Bezier::new(10, 2);
```

After creating the curve, you can add control handles using the `push` method.

```rust
bezier.push(Handle::mirrored(point!(-1.0, 1.0), point!(0.0, 0.0)));
bezier.push(Handle::mirrored(point!(1.0, 1.0), point!(2.0, 0.0)));
```

To calculate points along the curve, call the `calculate` method.

```rust
let points = bezier.calculate();
```

This will return a vector of points representing the curve.

### Inserting a Handle

You can insert a handle without changing the appearance of the curve using the `knot_insert` method.

```rust
bezier.knot_insert(0.5);
```

### Debugging

You can access all control points for debugging purposes using the `all_part_point` method.

```rust
let control_points = bezier.all_part_point();
```

## License

This library is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.