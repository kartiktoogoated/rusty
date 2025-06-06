struct Rect {
    width: u32,
    height: u32,
}

// this in js class

/*
    class Rect {
        constructor(wifth, height) {
            this.height = height;
            this.width = width;
        }
        
        area() {
        return this.width * this.height}
        }
    }

    const rect 1 = new Rect(10, 20)
    c.log(rect1.area)
 */
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };

    println!("area is {}", rect.area());
    println!("perimeter is {}", rect.perimeter());
}