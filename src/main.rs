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

fn comprobar_corte(x_corte: &f64, fun : &Record, fdata : &Record, sdata : &Record) -> bool{
    let mut ret = false;
    if fun.0 >= 0.0 && x_corte >= &fdata.0 && x_corte <= &sdata.0 {
        ret = true;
    } else if fun.0 <=0.0 && x_corte <= &fdata.0 && x_corte >=&sdata.0 {
        ret = true;
    }
    return ret;
}

fn comprobar_altura(cortes : &Vec<f64>, altura : &f64) -> bool{
    let mut ret = false;
    if cortes.len() >= 2 && &cortes[0] <= altura && &cortes[1] >= altura{
        ret = true;
    } else if cortes.len() >= 2 && &cortes[0] >= altura && &cortes[1] <= altura {
        ret = true;
    }
    return ret;

}

fn prueba(dato: &Vec<Record>,fun: &Vec<Record>) {
    let total = 1_000_000;
    let mut count = 0;
    let mut rng = thread_rng();
    for _ in 1..total {
        let x = (2.0 * rng.gen::<f64>())-1.0;
        let y = (2.0 * rng.gen::<f64>())-1.0;
        let m = (1.0-y)/(-1.0-x);
        let b = y-m*x;
        let mut cortes = Vec::new();
        for (i,el) in fun.iter().enumerate(){
            let first_data = dato[i];
            let second_data = segundo_dato(dato, i);
            let x_corte = (b-el.1)/(el.0-m);
            let y_corte = el.0*x_corte + el.1;
            if comprobar_corte(&x_corte, &el, &first_data, &second_data) && cortes.len()< 2{
                cortes.push((x_corte,y_corte));
            }
        }
        if cortes.len() >= 2 && comprobar_altura(&vec![cortes[0].1,cortes[1].1], &y){
            // println!("El punto es {:?} y corto en {:?}",(x,y),cortes);
            println!("ax.plot([{:?},{:?},{:?}],[{:?},{:?},{:?}],'o')",cortes[0].0,cortes[1].0,x,cortes[0].1,cortes[1].1,y);
            count += 1;
        }
    }
    println!("{:?}",count as f32/total as f32);
}

fn main() {
    let data = get_data();
    let functions = datos(&data);
    // montecarlo(&data , &functions);
    if let Ok(dato) = data {
        // prueba(&dato, &functions)
        prueba(&dato,&functions);
    }
}
