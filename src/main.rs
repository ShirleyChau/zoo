
fn main() {
    println!("Hello, world!");
}
enum FoodType{
    vegetables,meta
}
struct Food{
    food_type:FoodType,
    quality:f64,
}
enum StoolsType{
    drying,wets,slack//干的，湿的，拉稀的
}
struct Stools{//粪便
    weight:f64,
    stools_type:StoolsType,

}
trait organism{
    fn age(&self)->u8;
    fn height(&self)->f64;
    fn move(&self)->bool;
    fn grow(&self);
}
trait animal{
    fn eat(&mut self,food: Food);
    fn excreta(&self)->Stools;
    fn alive(&self)->bool;
}
trait plant{}
struct AnimalInfo{
    age:u8,
    height:f64,
    weight:f64,
    hungry:u8,
    life:bool,
    vegetables_rate:f64,
    meat_rate:f64,
    track:Vec<Food>,
}
struct Dog{
    animal_info:AnimalInfo,
}
pub impl Dog{
    fn new()->Dog{
        Dog{
            animal_info:AnimalInfo{
            age:0,
            height:30,
            weight:1,
            hungry:30,
            life:true,
            vegetables_rate:0.1,
            meat_rate:0.2,
            track:vec!(),
            }
        }
    }
    fn reduce_hungry(&mut self,food:Food){
        match food.food_type{
            FoodType::vegetables=>{
                if self.hungry-food.quality*self.vegetables_rate<0{
                    food.weight=self.hungry/self.vegetables_rate;
                    self.track.push(food);
                    self.hungry=0;
                }
                else{
                    self.hungry-=food.quality*self.vegetables_rate;
                }
            FoodType::meat=>{
                if self.hungry-food.quality*self.meat_rate<0{
                    food.weight=self.hungry/self.meat_rate;
                    self.track.push(food);
                    self.hungry=0;
                }
                else{
                self.hungry-=food.quality*self.meat_rate;
                }
        }
    }
    fn add_hungry(&mut self){
    }


}
pub impl Dog for animal{
    fn eat(&mut self,food:Food){
        self.reduce_hungry(food);
    }
    fn excreta(&self)->Stools;





