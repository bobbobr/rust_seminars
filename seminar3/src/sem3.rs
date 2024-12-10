#![allow(unused)]
#[derive(Debug)]
struct Coord{
    latitude: f32,
    longtitude: f32,
}

#[derive(Debug)]
struct ApartmentPrice(f32); // structure , похоже на картеж 

#[derive(Debug)]
struct Apartment {
    room_areas: [u32; 3],
    geolocation: Coord,
    total_price: ApartmentPrice,

}

// type ApartmentPrice = f32; or we can use it as 

fn main(){
    let apartment = Apartment {
        room_areas: [15;3],
        geolocation: Coord{
            latitude: 55.762,
            longtitude: 33.617,
        },
        total_price: ApartmentPrice(152000.0),
    };
    println!("{:?}",apartment);
}