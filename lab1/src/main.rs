
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    //#[arg(short, long)]
    string: String,
}

const SUBS_I:&str="àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍ
ÿýžźż";

const SUBS_O: &str="aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuu
uwxyyzzz";

fn conv(c:char)->char{
    let char_index = match SUBS_I.chars().position(|curr_c| curr_c==c){
        None=> return c,
        Some(c_idx)=> c_idx
    };
    SUBS_O.chars().nth(char_index).unwrap() //.take(1).collect();
}

fn slugify(s: &str)->String{

    //let mut slug=String::new();  //from(s.chars().map(|c| conv(c)));

    /*    //alternativa con match
    let mut match_slug:String=s.chars().map(|c| {
        let converted=conv(c);
        match converted{
            'A'..='Z'=> converted.to_ascii_lowercase(),
            'a'..='z'|'0'..='9' => converted,
            _ => '-'
        }
    }).collect();

    let re0 = Regex::new(r"--+").unwrap();
    match_slug=re0.replace_all(&match_slug,"-");
    if match_slug.len()>1 {
        let re1 = Regex::new(r"-$").unwrap();
        match_slug=re1.replace(&match_slug,"");
    }
    println!("match method 2: {}",match_slug);
    */

    let mut slug:String=s.chars().map(|c| conv(c)).collect();

    slug=slug.to_ascii_lowercase();

    //println!("slug: {:?}",slug);

    let re1 = Regex::new(r"[^a-z0-9]+").unwrap();
    slug=re1.replace_all(&slug,"-").to_string();
    //println!("slug1: {:?}",slug);

    if slug.len()>1{
        let re2=Regex::new(r"-$").unwrap();
        slug=re2.replace_all(&slug,"").to_string();
        //println!("slug3: {:?}",slug);
    }

    slug
}



fn main() {
    let args=Args::parse();
    let prova: String=args.string; //"?!#@!!!";  //"Wie héißen Sié?!"; //"-"; //"Wie héißen Sié?";
    println!("{}",slugify(&prova));


    /*
    let cli_opts = clap::App::new("My CLI Tool")
            .arg(clap::Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("file")
                .help("Write result to file with the specified name")
                .takes_value(true))
            .arg(clap::Arg::with_name("data")
                .required(true)
                .multiple(true)
                .help("The data that I'll print because I have no more use from them"))
            .get_matches();
    */

}

#[cfg(test)]
mod tests{
    use crate::slugify;

    #[test]
    fn mult_words(){
        let prova: &str="Wie heißen Sie?!"; //"?!#@!!!";  //"Wie héißen Sié?!"; //"-"; //"Wie héißen Sié?";
        assert_eq!("wie-heisen-sie",slugify(prova));
    }
    #[test]
    fn only_invalid(){
        let prova:&str="?!#@!!!";
        assert_eq!("-",slugify(prova));
    }
    #[test]
    fn mult_words_2(){
        let prova:&str="Kannst Du mich hören?";
        assert_eq!("kannst-du-mich-horen",slugify(prova));
    }

    #[test]
    fn accents(){
        let prova:&str="Papa, je l'ai fait jusqu'à la queue, tu me reçois ?";
        assert_eq!("papa-je-l-ai-fait-jusqu-a-la-queue-tu-me-recois",slugify(prova));
    }

    #[test]
    fn empty(){
        let prova:&str="";
        assert_eq!("",slugify(prova));
    }


}
