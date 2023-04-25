use std::any::Any;
use std::io;
use std::error::Error;
use rand::prelude::*;

// The `main` function is where your program starts executing.

type Record = (f64, f64);


fn get_data() -> Result<Vec<Record>,Box<dyn Error>> {
    let mut data = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() { let record: Record = result?; data.push(record); }
    // println!("{:?}",&data);
    Ok(data)
}



fn datos(data : &Result<Vec<Record>,Box<dyn Error>>)->Vec<Record> {
    let mut functions = Vec::new();
    if let Ok(dato) = data {
        for (i,el) in dato.iter().enumerate() {
            let second_data = segundo_dato(dato, i);
            let m = (second_data.1-el.1)/(second_data.0-el.0);
            let b = el.1-m*el.0;
            functions.push((m,b));
        }
    }
    println!("{:?}",functions);
    return functions;
}

// fn montecarlo(data : &Result<Vec<Record>,Box<dyn Error>>, functions : &Vec<Record>) {
//     let total = 1_000_000;
//     let mut count : i32 = 0;
//     let mut rng = thread_rng();
//     if let Ok(dato) = data{
//         let len_dato = dato.len()-1;
//         for _ in 1..total {
//             let mut cruces : i32 = 0;
//             let x = 2.0*rng.gen::<f64>();
//             let y = 2.0*rng.gen::<f64>();
//             let m = (2.0-y)/(2.0-x);
//             let b = y-m*x;
//             println!("({:?},{:?})",x,y);
//             for (i,el) in dato.iter().enumerate(){
//                 let mut second_data = dato[0];
//                 if i < len_dato{
//                     second_data = dato[i+1];
//                 }
//                 let fun = functions[i];
//                 let x1 = (b-fun.1)/(fun.0-x);
//                 let y1 = fun.0*x1+fun.1;
//             }
//             println!("{:?}",count);
//         }
//     }
// }

fn segundo_dato(dato:&Vec<Record>,ind : usize ) -> Record {
   let mut second_data = dato[0];
    if ind < dato.len()-1 {
        second_data = dato[ind+1];
    }
    second_data
}

fn prueba(dato: &Vec<Record>,fun: &Vec<Record>) {
    let total = 1_000_000;
    let mut count = 0;
    let mut rng = thread_rng();
    for _ in 1..total {
        let x = 2.0 * rng.gen::<f64>();
        let y = 2.0 * rng.gen::<f64>();
        let mut cruces = 0;
        let m = (2.0-y)/(2.0-x);
        let b = y-m*x;
        for (i,el) in fun.iter().enumerate(){
            let next = segundo_dato(dato, i);
            let x_corte = (b-el.1)/(el.0-m);
            let y_corte = el.0*x_corte + el.1;
        }
    }
}

fn main() {
    let data = get_data();
    let functions = datos(&data);
    // montecarlo(&data , &functions);
    if let Ok(dato) = data {
        // prueba(&dato, &functions)
        println!("La funcion es {:?} y los datos son {:?}",functions,dato);
        prueba(&dato,&functions);
    }
}
