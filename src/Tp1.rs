    pub fn Tp1(){
    /*1- Escribir un programa que defina una variable de tipo flotante con algún valor, y luego 
    permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y 
    restar su valor. Se deben imprimir los resultados.*/

    println!("Ingrese un operando");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");
    let num2:f64 = buf.trim().parse().expect("No es f64");
    let num1= 3.5;
    let suma= num1 + num2;
    let resta = num1 - num2;
    let mul = num1 * num2;
    let div= num1 / num2;
    println!("{}",suma);
    println!("{}",resta);
    println!("{}",mul);
    println!("{}",div);

    /* 2- Escribir un programa que defina una variable de tipo entero sin signo, y luego imprima su
    valor en hexadecimal */

    let num:u32=16;
    println!("{:x}",num);

    /*3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario 
    ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones 
    and y or. Se deben imprimir ambos resultados. */

    let var1=true;

    println!{"Ingrese un valor booleano"}
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");
    let mut var2:bool = buf.trim().parse().expect("No es boolean");
    var2= var1 & var2;
    println!{"Valor del booleano tras un and"}
    println!{"{}",var2};
    var2= var1 | var2;
    println!{"Valor del booleano tras un or"}
    println!{"{}",var2};

    /* 4- Escribir un programa que defina una tupla que contenga una cadena, un número entero 
 con signo y un valor booleano, y luego imprima cada valor de la tupla */
    let tupla:(String,i32,bool)=("el pepe ".to_string(),-5,true);
    println!("{:?}",tupla);

    /* 5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario 
 ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la 
 cadena en mayúsculas.*/

    let cadena1="hola don Jose";
    println!("Ingrese texto");
    let mut cadena2 = String::new();
    std::io::stdin().read_line(&mut cadena2).expect("Error");
    cadena2+=cadena1;
    println!("{}",cadena2);

    /* 6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al 
 usuario ingresar un número entero por teclado para sumarse con la variable definida. El 
 programa debe imprimir el valor del número elevado al cuadrado */
   let var1:u32=3;
   println!{"Ingrese un valor para sumarlo al 3"}
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");
    let mut var2:u32 = buf.trim().parse().expect("No es entero");
    var2= (var1+var2)*(var1+var2);
    println!("el valor del valor ingresado mas 3 al cuadrado es");
    println!("{}",var2);

  /* 7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números 
 enteros, y luego multiplique cada valor del arreglo por un valor constante definido, 
 modificando el contenido del arreglo. */
   const miConstante:i32 =10;
   let mut arreglo= [1,2,3,4,5,6];
   arreglo[0]=arreglo[0]*miConstante;
   arreglo[1]=arreglo[1]*miConstante;
   arreglo[2]=arreglo[2]*miConstante;
   arreglo[3]=arreglo[3]*miConstante;
   arreglo[4]=arreglo[0]*miConstante;
   arreglo[0]=arreglo[0]*miConstante;


  /*  8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el 
 número de veces que un caracter específico ingresado por el usuario aparece en la cadena. 
 Se debe imprimir el resultado. */
 let mut cadena="El ego es ese pequeño argentino que todos llevamos adentro";
 let char_vec:Vec<char>=cadena.chars().collect();
 println!{"Ingrese un caracter para ver la cantidad de veces que aparece en la cadena"}
 let mut cant=0;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");
    let mut car:char = buf.trim().parse().expect("No es entero");
    let mut i=0;
    while i<char_vec.len(){
      if char_vec[i]==car{
         cant=cant+1;
      }
      i=i+1;
    }
println!("la cantidad de veces que aparece la letra ingresada es");   
println!("{}",cant);

/* 9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la 
 suma de los valores del  arreglo */
 let arreglo=[1,2,3,4,5];
 let mut suma=0;
 let mut i=0;
 for i in 1..arreglo.len()
 {
   suma=suma+arreglo[i];
 }
 println!("la suma es {}",suma);


/* 10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego 
 cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos 
 originales. */
 let arreglo1=[1,2,3,4,5];
 let arreglo2=[9,8,7,6,5];
 let mut arreglo3=[0,0,0,0,0];
 for i in 0..arreglo1.len(){
   arreglo3[i]=arreglo1[i]+arreglo2[i];
 }
 for elemento in arreglo3{
   println!("{}",elemento);
 }

/* 11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario 
 ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena 
 ingresada por el usuario se encuentra en el arreglo.*/
 let vector = ["pepe", "carlos", "pablo", "pedro", "rulo"];

// Solicita al usuario que ingrese una palabra
println!("Ingrese una palabra:");
let mut cadena = String::new();
std::io::stdin().read_line(&mut cadena).expect("Error al leer entrada");

// Elimina cualquier espacio en blanco adicional de la entrada del usuario
let palabra_buscar = cadena.trim();

// Verifica si la palabra ingresada por el usuario está en el vector
let esta = vector.contains(&palabra_buscar);

// Muestra el resultado
if esta {
    println!("La palabra se encuentra en el vector.");
} else {
    println!("La palabra no se encuentra en el vector.");
}


/*  12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de 
 enteros, y luego imprima la cadena y la suma de los valores en el arregl*/
   let tupla:(String,[u8 ;2]) = ("hola".to_string(),[3,5]);
   println!("{}",tupla.0);
   let mut total=0;
   for i in tupla.1{
      total+=i;
   }
   println!("{}",total);
    }