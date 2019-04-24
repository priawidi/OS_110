//Mentored by Hafizhun Alim a.k.a Cah Kangkung

pub struct Triangle{
    side_a : u64,
    side_b : u64,
    side_c : u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        //unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
        // this function is to build the triangle from its sides
        if sides[0] <= 0 || sides[1] <= 0 || sides[2] <= 0{
            None
        }
        else if (sides[0]+sides[1]) <= sides[2] || (sides[1]+sides[2]) <= sides[0] || (sides[2]+sides[0]) <= sides[1] {
            None

        }
        else{
            Some(Triangle{side_a: sides[0] , side_b: sides[1], side_c: sides[2]})
        }
        
        

    }

    pub fn is_equilateral(&self) -> bool {
        //unimplemented!("Determine if the Triangle is equilateral.");
        if self.side_a == self.side_b && self.side_b == self.side_c && self.side_c == self.side_a{
            true
        }
        else{
            false
        }
    }

    pub fn is_scalene(&self) -> bool {
        //unimplemented!("Determine if the Triangle is scalene.");
        if self.side_a != self.side_b && self.side_b != self.side_c && self.side_c != self.side_a{
            true
        }
        else {
            false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        //unimplemented!("Determine if the Triangle is isosceles.");
        if self.side_a == self.side_b || self.side_b == self.side_c || self.side_c == self.side_a{
            true
        }
        else{
            false
        }
    }
}
