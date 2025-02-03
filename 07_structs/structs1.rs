//structs 1
struct ColorRegularStruct {
    red : u8,
    green : u8,
    blue : u8,
}

struct ColorTupleStruct(u8,u8,u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {

    // instantiate and print the structs
    let regular = ColorRegularStruct { 
        red: 255, 
        green: 0, 
        blue: 0 
    };
    println!("Regular Struct - Red: {}, Green: {}, Blue: {}", regular.red, regular.green, regular.blue);

    let tuple = ColorTupleStruct(255, 0, 0);
    println!("Tuple Struct - Red: {}, Green: {}, Blue: {}", tuple.0, tuple.1, tuple.2);

    let unit = UnitStruct;
    println!("{unit:?}");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct {
            red:0,
            green:255,
            blue:0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
