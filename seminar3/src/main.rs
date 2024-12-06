#![allow(unused)]
#[derive(Debug)]
struct Coord{
    latitude: f32,
    longtitude: f32,
}

#[derive(Debug)]
//struct ApartmentPrice(f32); // structure , похоже на картеж 
struct ApartmentPrice{
    price: f32,
    currency: String,
}

impl ApartmentPrice{
    fn is_too_high(&self) -> bool{
        self.price > 150000.0
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


fn main(){
    let apartment = Apartment {
        room_areas: [15;3],
        geolocation: Coord{
            latitude: 55.762,
            longtitude: 33.617,
        },

        price_data: ApartmentPrice{price: 152000.0, currency: String::from("RUB")},
    };
    println!("{:?}",apartment);
    //apartment.total_price.f();
    println!("{:?}",apartment.price_data.is_too_high());
}