use random_number::random;
pub fn tp1entregable(){
    let array=[10,3,5,7,88];
    println!("{:?}",get_par_random(array));
}

fn get_par_random(array:[i32;5])-> i32{
    let r:i32=random!(0,4);
    let mut dato= array[r as usize];
    if dato % 2==1{
        dato=-1;
    }
    dato
}   
