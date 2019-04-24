This kind of essay is to fulfill a task for Operating System course. I choose Hamming because i think its quite easy to explain.

# Hamming
  Hamming Distance is the mistakes occured when two strands of DNA is compared and the differences between them are counted.
  
  This task is required to calculate the Hamming Distance beetween two DNA strands.
  
 **examples**:given two DNA (GAGCCTACTAACGGGAT) and (CATCGTAATGACGGCCT), they have 7 differences, and therefore the Hamming Distance is 7.
  
  
  The Hamming Distances is **only defined for sequences of equal length**, so an attempt to calculate it between sequences of different lengths should not work.
  
 ## Solution
 My first thought to solve this problem is to compare each elements of each given DNA. So, when the elements doesn't match, the counters will increment.
 
 My methods of solving this problem are using: .chars() to convert the strings into char type. So, its split the strings into a single char elements. Then, the strings that are already converted, inserted into an empty 'Vec' so, 'Vec' contains the elements of chars. The codes are shown below :
 
 ```rust
    let char1 = s1.chars(); //convert the given DNA string into char type
    let char2 = s2.chars(); //convert the given second DNA string into char type

    for i in char1{   
        list.push(i)   //insert each char element into empty 'vec'
    }
    for x in char2{
        list1.push(x)    //insert each char element into other empty 'vec'
    }
 ```
My next step is to create conditions whether the lenghts of the first and second DNA are the same or not. If the lenghts are match, the progress is continue into iteration of the condition. If the elements of the first DNA didn't match the second DNA elements, it will return Some(count+=1). Since its using Option types, the return must be Some or None. The condition will return None if the requirement didn't match.

``` rust
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
```


## Full Code
Here is my full code for the Hamming problem.
 
```rust
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
    
}
```


