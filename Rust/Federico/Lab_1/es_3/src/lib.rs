use std::cmp::{max, min};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut start_x;
    let mut start_y;
    let mut end_x;
    let mut end_y;
    let mut discarded: u16 = 0;
    let mut res:Vec<String> = vec![];
    for (i, row) in minefield.iter().enumerate(){
        let mut new_row: String = String::new();
        for (j, el) in row.as_bytes().iter().enumerate(){
            match *el{
                b'.'  => {
                    start_x = max(0, i as i8 - 1) as usize;
                    start_y = max(0, j as i8 - 1) as usize;
                    end_x = min(row.len(), i + 2);
                    end_y = min(minefield.len(), j + 2);

                    let mut count: u8 = 0;

                    //perché per iterare ho una ref di una ref??

                    for r in &minefield[start_x..end_x] {
                        for c_to_check in &r.as_bytes()[start_y..end_y] {
                            match c_to_check {
                                b'*' => { count += 1; }
                                _ => {}
                            }
                        }
                    }
                    /*
                    PERCHE' &*?
                    PERCHE' IL PRIMO MODO NON COMPILA E IL SECONDO SI? (PER IL FATTO CHE NEL RITORNARE IL VALORE DAL MATCH NON E' POI PIU' IN SCOPE LA STRINGA CHE CREO NELLA CONVERSIONE?)
                    new_row.push_str( match count {
                        0 => {"."}
                        _ => {&*count.to_string()}
                    });
                    new_row.push_str(&*count.to_string());*/
                    match count {
                        0 => {new_row.push_str(".")}
                        _ => {new_row.push_str(&*count.to_string())}
                    }
                }
                b'*' => {
                    new_row.push_str("*");
                }
                _ => {
                    discarded+=1;
                }
            };
        }
        res.push(new_row);
    }
    if discarded>0{
        println!("In input matrix there are {} characters different from '.' and '*'...\nThese characters has been discarded, are you sure that the input is correct?", discarded);
    }
    return res;
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}

//fn start_calculator(id)