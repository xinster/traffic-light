fn main() {
    println!("This is the first homework of class 4. Use enum traffice light to build a trait. The trait has a function to return duration. Different light has different duration.\n");
    println!("Red traffic light durstion is {} seconds", LightColor::light_time(LightColor::Red));
    println!("Green traffic light durstion is {} seconds", LightColor::light_time(LightColor::Green));
    println!("Yellow traffic light durstion is {} seconds", LightColor::light_time(LightColor::Yellow));

}

pub enum LightColor{
    Red,
    Green,
    Yellow
}

pub trait Lights {
    fn light_time(&self)->LightColor;

}

impl LightColor{
    pub fn light_time(light:LightColor)->u8{
        match light{
            LightColor::Red =>15,
            LightColor::Green =>30,
            LightColor::Yellow =>5
        }
    }
}