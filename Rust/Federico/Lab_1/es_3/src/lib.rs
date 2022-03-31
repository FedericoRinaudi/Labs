use std::cmp::{max, min};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let start_x;
    let start_y;
    let end_x;
    let end_y;
    let mut res:Vec<String> = vec![];
    for (i, row) in minefield.iter().enumerate(){
        for (j, el) in row.as_bytes().iter().enumerate(){
            if *el == b'*' {
                start_x = max(0, i-1);
                start_y = max(0, j-1);
                end_x = min(minefield.len(), i+1);
                end_y = min(row.len(), j+1);

                for x in start_x..end_x{
                    for y in start_y..end_y{
                        
                    }
                }

            }
        }
    }

    return vec!["aaaa".to_string(), "bbbbbbb".to_string()];
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}

//fn start_calculator(id)