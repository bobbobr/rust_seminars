#![allow(unused)]
#[derive(Debug, Clone)]

struct Coord{
    latitude: f32,
    longtitude: f32,
}

impl Default for Coord {
    fn default() -> Self {
        Self { latitude: 55.762, longtitude: 33.617 }
 
    }
}

#[derive(Debug, Default, Clone)]
enum Currency{
    #[default]
    RUB,
    USD,
    GBP,
    EUR
}
#[derive(Debug, Default, Clone)]
//struct ApartmentPrice(f32); // structure , похоже на картеж 
struct ApartmentPrice{
    price: f32,
    currency: Currency,
}
// use std::fmt::Debug;
// impl Debug for ApartmentPrice {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "debug {} {}", self.price, self.currency)
//     }
// }

impl std::fmt::Display for ApartmentPrice{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "display {} {:?}", self.price, self.currency)
    }
}


impl ApartmentPrice{
    fn is_too_high(&self) -> bool{
        self.price >= 150000.0
    }
    fn smth() -> i32{
        150
    }
    fn change_price(&mut self, delta: f32){
        self.price += delta
    }
}

#[derive(Debug, Default, Clone)]
struct Apartment {
    room_areas: [u32; 3],
    geolocation: Coord,
    price_data: ApartmentPrice,

}

fn try_location(a: &Apartment) -> Option<&Coord>{
    let Coord {latitude,longtitude} = a.geolocation;
    if latitude == 0.0 || longtitude == 0.0 {
        None
    } else {
        Some(&a.geolocation)
    }
}
// impl ApartmentPrice{
//     fn f(&self) {
//         println!("{}", self.0)
//     }
// }

// type ApartmentPrice = f32; or we can use it as 

trait HighPriceTrait{
    fn is_too_high(&self) -> bool{
        unimplemented!()
    }
}

impl HighPriceTrait for ApartmentPrice{
    fn is_too_high(&self) -> bool{
        self.price >=200000.0
    }
}

fn div(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }

}

fn get_location(a: &Apartment) -> Result<&Coord, &'static str> {
    let Coord {latitude,longtitude} = a.geolocation;
    if latitude == 0.0{
        Err("Latitude must be not zero")
    } else if longtitude == 0.0{
        Err("Longitude must be not zero")
    } else{
        Ok(&a.geolocation)
    }

}
fn main(){

    let a = 10.0;
    let b = 2.0;
    let c = div(a, b);
    if c.is_none() {
        println!("Couldn't divide");
    } else {
        println!("{}", c.unwrap());
    }


    let mut apartment = Apartment {
        room_areas: [15;3],

        //..Default::default()
        geolocation: Coord{
            latitude: 55.762,
            longtitude: 42.0,
        },

        price_data: ApartmentPrice{price: 149000.0, currency: Currency::USD},
    };

    let result = get_location(&apartment);
    match result {
        Err(msg) => println!("Couldn't get location: {}", msg),
        Ok(coords) => println!("The aparment is located at {:?}", coords),
    }

    let result = try_location(&apartment);
    if result.is_none() {
        panic!("Couldn't find the aparment");
    }
    let coords = result.expect("Couldn't find the aparment");
    println!("{:?}", coords);

    let x = 5;
    match x{
        1..=5 => println!("One 1-5"),
        _ => println!("Something else")
    }
    // if let Some(&coords) = try_location(&third_apartment){
    //     println!("The aparment is located at {:?}", coords);
    // } else{
    //     panic!("Couldn't find the apartemt")
    // }
    // println!("{:?}",apartment);
    // println!("{}",apartment.price_data);

    // let mut new_apartment = apartment.clone();
    // println!("{:?}",new_apartment);
    // //let apartment = Apartment::default();

    // println!("{:?}",apartment);
    // println!("{:?}",new_apartment.price_data.change_price(1000.0));
    // let third_apartment = Apartment {
    //     room_areas: [12,13,14],
    //     ..new_apartment
    // };

    // println!("{:?}",third_apartment);

    // //apartment.total_price.f();
    // println!("{:?}",apartment.price_data.is_too_high());
    // println!("{:?}",ApartmentPrice::smth());
    // println!("{:?}",apartment.price_data.change_price(1000.0));
    // println!("{:?}",apartment.price_data.is_too_high());

    // println!("{:?}",apartment);

    // println!("{:?}", HighPriceTrait::is_too_high(&apartment.price_data)); // or
    // println!("{:?}", <ApartmentPrice as HighPriceTrait>::is_too_high(&apartment.price_data));
    // println!("{:?}", (&apartment.price_data as &dyn HighPriceTrait).is_too_high());

}