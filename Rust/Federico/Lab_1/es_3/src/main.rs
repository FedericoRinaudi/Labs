use minesweeper::*;

/*

let prova: &[&str] = &[&"*...", &"..*.", &"..*.", &"..*."]; così i tipi matchano
let prova: [&str] = [&"*...", &"..*.", &"..*.", &"..*."]; così no!!!! come mai?

 */

fn main(){
    let prova: &[&str] = &["*...", "..*.", "..*.", "..*."];
    let b = annotate(prova);
    println!("{}", b.len());
}