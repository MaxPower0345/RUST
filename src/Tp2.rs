/*1-Definir la función llamada es_parque recibe como parámetro un número entero y retorna
true si el número es par, false caso contrario */
pub fn ej1(){
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");
    let num:u8 = buf.trim().parse().expect("No es u8");
    if es_par(num){
     println!("el valor ingresado es par");
    }
    else
    {
        println!("el valor ingresado es inpar");
    }

fn es_par(num:u8)->bool{
    num %  2== 0
    }
}
/*2- Definir la función llamada es_primo que recibe un número entero positivo mayor a 1 y
retorna true si es primo, false caso contrario */
pub fn ej2(){
    let mut buf = String::new();
    println!("Ingrese un valor");
    std::io::stdin().read_line(&mut buf).expect("Error");
    let num:u8 = buf.trim().parse().expect("No es u8");

    if es_primo(num){
        println!("el valor ingresado es primo");
       }
       else
       {
           println!("el valor ingresado no es primo");
       }

    fn es_primo(num:u8)-> bool{
        let mut es=true;
        for i in 2..num-1{
            if (num % i)==0{
                es=false;
            }
        }
      es  
    }
}
/*3- Definir la función llamada suma_paresque recibe como parámetro un arreglo de
números enteros y retorna la suma de los números pares. */
pub fn ej3(){
    let vector=[1,2,4];
    println!("{}",suma_pares(vector));
    fn suma_pares(vector:[u8;3])-> u8{
        let mut suma:u8= 0;
        for i in vector{
            if(i%2)==0{
                suma+=i;
            }
        }
        suma
    }
}
/*4- Definir la función llamada cantidad_imparesque recibe como parámetro un arreglo de
números enteros y retorna la cantidad de números impares */
pub fn ej4(){
    let vector=[1,2,4];
    println!("{}",suma_pares(vector));

    fn suma_pares(vector:[u8;3])-> u8{
        let mut suma:u8= 0;
        for i in vector{
            if(i%2)!=0{
                suma+=1;
            }
        }
        suma
    }
}
/*5-Defina la función llamada duplicar_valoresque recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro */
pub fn ej5() {
    let vector=[3.0,5.0,7.5];
    let vector2=duplicar_valores(vector);
    for i in vector2{
        println!("{}",i);
    }


    fn duplicar_valores(vector:[f64;3])-> [f64;3]{
        let mut vector2=[0.0,0.0,0.0];
        for i in 0..vector.len(){
            vector2[i]=vector[i]*2.0;
        }
        vector2    
    }
}
/*6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
arreglo */
pub fn ej6(){
    let vector=["pepe","carlos","nico","juan","alejandro"];
    let vector2 =longitud_de_cadenas(vector);
    for i in vector2{
        println!("{}",i);
    }

    fn longitud_de_cadenas(vector:[&str;5])-> [u8;5]{
        let mut vector2=[0,0,0,0,0];
        for i in 0..vector.len(){
            vector2[i]=vector[i].len() as u8;
        }
        vector2
    }
}
/*7-Definir la función llamada cantidad_de_mayoresque recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo */
pub fn ej7(){
    let vector=[1,2,3,4,5,6,7,8,9,10];
    let lim=5;
    let cant =longitud_de_cadenas(vector,lim);
    println!("{}",cant);

    fn longitud_de_cadenas(vector:[i32;10],lim:i32)-> u8{
        let mut cant:u8=0;
        for i in vector{
            if i>lim {
                cant+=1;
            }
        }
        cant
    }
}
/*8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
arreglos pasados por parámetro, correspondiendose el resultado con cada posición de los
arreglos pasados por parámetro */
pub fn ej8(){
    let vector=[1.0,4.5,9.10];
    let vector2=[9.0,4.5,0.90];
    let vectord=sumar_arreglos(vector, vector2);
    for i in vectord{
        println!("{}",i);
    }

    fn sumar_arreglos(vector:[f64;3],vector2:[f64;3])->[f64;3]{
            let mut vectord=[0.0,0.0,0.0];
            for i in 0..vector.len(){
                vectord[i]=vector[i]+vector2[i];
            }
            vectord
    }
}
/*9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive */
pub fn ej9(){
    let vector=[1,2,3,4,5,6,7,8,9,10];
    let min=4;
    let max=7;
    let cant=cantidad_en_rango(vector,max,min);
    println!("{}",cant);

    fn cantidad_en_rango(vector:[i32;10], max:i32, min:i32)-> i32{
        let mut cant:i32=0;
        for i in vector{
            if(i<max)&&(i>min){
                cant+=1;
            }
        }
        cant
    }
}
/*10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite. */
pub fn ej10(){
    let vector=["capi","igna","rulo","sherlock","ivi","joaco","nico"];
    let lim=4;
    let cant= cantidad_de_cadenas_mayor_a(vector,lim);
    println!("{}",cant);

    fn cantidad_de_cadenas_mayor_a(vector:[&str;7],lim:i32)-> i32{
        let mut cant=0;
        for i in vector{
            if i.len()>lim as  usize{
                cant+=1;
            }
        }
    cant
    }
}
/*11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo */
pub fn ej11(){
    let vector =[3,6,9];
    let factor=3;
    let vectord=multiplicar_valores(vector, factor);
    for i in vectord{
        println!("{}",i);
    }

    fn multiplicar_valores(vector:[i32;3],factor:i32)-> [i32;3]{
        let mut vectord=[0,0,0];
        for i in 0..vector.len(){
            vectord[i]=vector[i]*factor;
        }
        vectord
    }
}
/*12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
reemplaza todos los números pares por -1. */
pub fn ej12(){
    let vector =[1,2,3,4,5,6,7,8,9,10];
    let vector=reemplazar_pares(vector);
    for i in vector{
        println!("{}",i);
    }
    fn reemplazar_pares(mut vector:[i32;10])->[i32;10]{
        for i in 0..vector.len(){
            if (vector[i] % 2)==0{
                vector[i]=-1;
        }
    }
    vector
}
}
/*13-Definir una función llamada ordenar_nombresque recibe un arreglo de String y los
ordena en orden alfabético */
pub fn ej13(){
    let vector=["i","o","u","a","e"];
    let vector=ordenar_nombres(vector);
    println!("{:?}",vector);

    fn ordenar_nombres(mut vector:[&str;5])-> [&str;5]{
        vector.sort();
        vector
    }
}
/*14-Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor. */
pub fn ej14(){
    let mut num:f64=1.0;
    num=incrementar(1.0);
    println!("{}",num);
    fn incrementar(num:f64)-> f64{
        num+1.0
    }
}
