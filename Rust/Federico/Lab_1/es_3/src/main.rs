use minesweeper::*;

/*

let prova: &[&str] = &[&"*...", &"..*.", &"..*.", &"..*."]; così i tipi matchano
let prova: [&str] = [&"*...", &"..*.", &"..*.", &"..*."]; così no!!!! come mai?

 */

fn main(){
    let prova: &[&str] = &["*...", "..*.", "..*.", "..*."];
    print_matrix(prova);
    println!("\n\n");
    let b: Vec<String> = annotate(prova);
    print_string_vec(b);
}

fn print_matrix (m: &[&str]) {
    for row in m{
        println!("\n");
        for el in row.chars(){
            print!("{}  ", el);
        }
    }
}
fn print_string_vec (v: Vec<String>) {
    for row in v{
        println!("\n");
        for el in row.chars(){
            print!("{}  ", el);
        }
    }
}
