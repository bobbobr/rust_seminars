#![allow(unused)]
#[derive(Debug)]

struct Coord{
    latitude: f32,
    longtitude: f32,
}

//#[derive(Debug)]
//struct ApartmentPrice(f32); // structure , похоже на картеж 
struct ApartmentPrice{
    price: f32,
    currency: String,
}
use std::fmt::Debug;
impl Debug for ApartmentPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "debug {} {}", self.price, self.currency)
    }
}

impl std::fmt::Display for ApartmentPrice{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "display {} {}", self.price, self.currency)
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

#[derive(Debug)]
struct Apartment {
    room_areas: [u32; 3],
    geolocation: Coord,
    price_data: ApartmentPrice,

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

fn main(){
    let mut apartment = Apartment {
        room_areas: [15;3],
        geolocation: Coord{
            latitude: 55.762,
            longtitude: 33.617,
        },

        price_data: ApartmentPrice{price: 149000.0, currency: String::from("RUB")},
    };
    println!("{:?}",apartment);
    println!("{}",apartment.price_data);
    

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