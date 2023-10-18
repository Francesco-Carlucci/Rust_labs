use std::sync::Arc;
use clap::Parser;
use itertools::Itertools;
//parallelization and profiling
use std::thread;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// vector of digits
    #[arg(short, long,num_args = 5, value_delimiter = ' ')]
    cifre: Vec<i8>,
}

fn compute_string(t: (Vec<i8>,Vec<&char>))->(i8,String){
    let mut s= String::new();

    //print!("{:?}",t);
    let (mut cifre,ref ops)=t;

    let mut res=cifre.pop().unwrap();

    s.push_str(&res.to_string());
    for (op,cifra) in ops.iter().zip(cifre){
        //println!("{res}, {cifra}");
        res=match op{
            '+'=> res+ cifra,
            '-'=> res- cifra,
            '*'=> res* cifra,
            '/'=> res/ cifra,
            _ =>panic!()
        };
        s.push(**op);
        s.push_str(&cifra.to_string());
    }
    (res,s)
}

fn search_solutions(permutations: Vec<(Vec<i8>, Vec<&char>)>) ->Vec<String>{
    let mut solutions=Vec::<String>::new();

    for t in permutations.into_iter(){

        let (res,s)=compute_string(t);

        if res==10{
            solutions.push(s);
        }
    }
    solutions
}

fn main() {
    let args=Args::parse();
    let cifre=args.cifre;
    println!("{:?}",cifre);

    let mut solutions=Vec::<String>::new();

    let all_permutations:Vec<(Vec<i8>,Vec<&char>)>= cifre.into_iter().permutations(5).unique().cartesian_product(
        std::iter::repeat(['+', '-', '*', '/'].iter()).take(4).multi_cartesian_product()).collect();

    let cnt=all_permutations.len();

    //let solutions=search_solutions(all_permutations);

    let mut thread_handles=Vec::new();

    /// PROVARE VERSIONE CON Arc!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    //let perm_arc=Arc::new(all_permutations);

    let num_threads=6;
    let start = Instant::now();

    for chunk in all_permutations.chunks(cnt/num_threads){   //i in 0.. num_threads{ //
        //let slice_n:Vec<(Vec<i8>, Vec<&char>)>=all_permutations[i..cnt].to_owned().into_iter().step_by(num_threads).collect(); //interleaved!
        let slice_n=Vec::from(chunk); //i*cnt/num_threads..(i+1)*cnt/num_threads
        thread_handles.push(thread::spawn(move ||{search_solutions(slice_n)}));
    }

    //let slice1=Vec::from(&all_permutations[0..cnt/2]);
    //thread_handles.push(thread::spawn(move ||{search_solutions(slice1)}));
    //let slice2=Vec::from(&all_permutations[cnt/2..cnt]);
    //thread_handles.push(thread::spawn(||{search_solutions(slice2)}));

    for handle in thread_handles{
        solutions.append(&mut handle.join().unwrap());
    }

    println!("Time:{:?}",start.elapsed());
    println!("Ciao mamma! n. of possible permutations: {:?} n: of solutions: {:?} \nsolutions: {:?}",cnt, solutions.len(), solutions);

}
