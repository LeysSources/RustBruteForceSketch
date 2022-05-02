fn main() {
    //size
    brute(5);
}
fn brute(max_size:u32){
    let mut arr:Vec<i32> = Vec::new();
    while &max_size  > &arr.len().try_into().unwrap(){
        arr.push(32);

    }
    
    

    let mut index = arr.len() - 1;
    while arr[0] < 123 {
       
            if arr[index] == 123{
                arr[index] = 33;
                arr[index - 1] = arr[index - 1] + 1;
               print(&arr);
                index = index - 1;
            }
        
         else{
                index = arr.len() - 1;
                arr[index] = arr[index] + 1;
              print(&arr);
            }
            }
    print(&arr);
}



fn print(arr:&Vec<i32>) {
    
    for x in arr{
      if x != &32{

      
       let f =  *x as u8;
        print!("{}",f as char);
    }
    }
    println!();
}