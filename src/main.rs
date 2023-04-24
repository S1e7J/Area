use std::io;
use std::error::Error;
use rand::prelude::*;

// The `main` function is where your program starts executing.

type Record = (f64, f64);


fn get_data() -> Result<Vec<Record>,Box<dyn Error>> {
    let mut data = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        data.push(record);
    }
    // println!("{:?}",&data);
    Ok(data)
}


fn Datos(data : &Result<Vec<Record>,Box<dyn Error>>)->Vec<Record> {
    let mut functions = Vec::new();
    if let Ok(dato) = data {
        let len_Dato = dato.len()-1;
        for (i,el) in dato.iter().enumerate() {
            let mut secondData = dato[0];
            if i < len_Dato {
                secondData = dato[i+1];
            }
            let m = (secondData.1-el.1)/(secondData.0-el.0);
            let b = el.1-m*el.0;
            functions.push((m,b));
        }
    }
    println!("{:?}",functions);
    return functions;
}

fn montecarlo(data : &Result<Vec<Record>,Box<dyn Error>>, functions : &Vec<Record>) {
    let total = 10_000_000;
    let mut count : i32 = 0;
    let mut rng = thread_rng();
    if let Ok(dato) = data{
        let len_Dato = dato.len()-1;
        for _ in 1..total {
            let mut cruces : i32 = 0;
            let x = (2.0*rng.gen::<f64>());
            let y = (2.0*rng.gen::<f64>());
            let m = (2.0-y)/(2.0-x);
            let b = y-m*x;
            println!("({:?},{:?})",x,y);
            for (i,el) in dato.iter().enumerate(){
                let mut secondData = dato[0];
                if i < len_Dato{
                    secondData = dato[i+1];
                }
                let fun = functions[i];
                let x1 = (b-fun.1)/(fun.0-x);
                if (fun.0-m != 0.0 && x1 <= el.0 && x1 >= secondData.0){
                    println!("Se cruzo con {:?}",fun);
                    cruces += 1;
                }
            }
            if (cruces).rem_euclid(2) != 0 {
                count += 1;
            }
            println!("{:?}",count);
        }
    }
}

fn main() {
    let data = get_data();
    let functions = Datos(&data);
    montecarlo(&data , &functions)
}
