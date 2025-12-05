// Strategy Pattern
// for Duck Simulation
/*
  Duck      Swim    Fly     Quack   display
Mallard      T      TTT      TTT       X
Sherkes      T       T        T        Y
Rubher       T       F        F        Z
*/
trait Flyable{
    fn fly(&self);
}
struct FastFly;
impl Flyable for FastFly{
    fn fly(&self) {
        println!("Flyable FastFly");
    }
}
struct MedFly;
impl Flyable for MedFly{
    fn fly(&self) {
        println!("Flyable MedFly");
    }
}

trait Quackable{
    fn quack(&self);
}
struct HighQuack;
impl Quackable for HighQuack{
    fn quack(&self) {
        println!("Quackable HighQuack");
    }
}
struct MedQuack;
impl Quackable for MedQuack{
    fn quack(&self) {
        println!("Quackable MedQuack")
    }
}

trait Duck{
    fn swim(&self){
        println!("Duck Swimming");
    }
    fn display(&self);
}

struct Mallard<'a>{
    flyable: Option<&'a dyn Flyable>,
    quackable: Option<&'a dyn Quackable>,
}
impl<'a> Mallard<'a>{
    fn new()->Self{
        Self { 
            flyable: None,
            quackable: None,
        }
    }
    fn fly(&self){
        if let Some(fly) = self.flyable{
            fly.fly();
        }
    }
    fn quack(&self){
        if let Some(quack) = self.quackable{
            quack.quack();
        }
    }
    fn set_fly(&mut self, fly:&'a dyn Flyable){
        self.flyable = Some(fly);
    }
    fn set_quack(&mut self, quack:&'a dyn Quackable){
        self.quackable = Some(quack);
    }
}
impl<'a> Duck for Mallard<'a>{
    fn display(&self) {
        println!("I'm Mallard Duck");
    }
}

struct Sherkes<'a>{
    flyable: Option<&'a dyn Flyable>,
    quackable: Option<&'a dyn Quackable>,
}
impl<'a> Sherkes<'a>{
    fn new()->Self{
        Self{
            flyable: None,
            quackable: None
        }
    }
    fn fly(&self){
        if let Some(fly) = self.flyable{
            fly.fly();
        }
    }
    fn quack(&self){
        if let Some(quack) = self.quackable{
            quack.quack();
        }
    }
    fn set_fly(&mut self, fly:&'a dyn Flyable){
        self.flyable = Some(fly);
    }
    fn set_quack(&mut self, quack:&'a dyn Quackable){
        self.quackable = Some(quack);
    }
}
impl<'a> Duck for Sherkes<'a>{
    fn display(&self){
        println!("I'm Sherkes Duck");
    }
}
struct Rubher;
impl Rubher{
    fn new()->Self{
        Self
    }
}
impl Duck for Rubher{
    fn display(&self){
        println!("I'm Rubher Ducks");
    }
}
fn main() {
    let fast_fly = FastFly;
    let med_fly = MedFly;
    let high_quack = HighQuack;
    let med_quack = MedQuack;

    let mut mallard = Mallard::new();
    mallard.set_fly(&fast_fly);
    mallard.set_quack(&high_quack);
    println!("Test Mallard Duck");
    mallard.swim();
    mallard.display();
    mallard.fly();
    mallard.quack();
    println!();
    
    let mut sherkes = Sherkes::new();
    println!("Test Sherkes Duck");
    sherkes.set_fly(&med_fly);
    sherkes.set_quack(&med_quack);
    sherkes.swim();
    sherkes.display();
    sherkes.fly();
    sherkes.quack();
    println!();
    
    let rubher = Rubher::new();
    println!("Test Rubher Duck");
    rubher.swim();
    rubher.display();
    println!();
}
