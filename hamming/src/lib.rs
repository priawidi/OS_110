/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    //unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
    let mut count = 0;
    let mut list = Vec::new();
    let mut list1 = Vec::new();
    let len1 = s1.len();
    let len2 = s2.len();
    let char1 = s1.chars();
    let char2 = s2.chars();

    for i in char1{
        list.push(i)
    }
    for x in char2{
        list1.push(x)
    }
    if len1 == len2{
        for a in 0.. list.len(){
            if list[a] != list1[a]{
                    
                    Some(count= count+1);
                }
            
        }
        Some(count)
    }
    else{
        None
    }
    
    /*if len1 == len2{

        let char1 = s1.chars();
        let char2 = s2.chars();
        for i in 0.. len1{
            for x in 0.. len2{
                if s1[i] == s2[x]{
                    Some(count+=1)
                }
                if s1[i] != s2[x]{
                    None
                }
            }
        }
    }
    Count*/
}
