/*pub fn find(array: &[i32], key: i32) -> Option<usize> {       //iterativa
    /*unimplemented!(
        "Using the binary search algorithm, find the element '{}' in the array '{:?}' and return its index.",
        key,
        array
    );*/
    let mut inizio = 0;
    let mut fine = array.len();
    while inizio!=fine {
        let medio = inizio+(fine-inizio)/2;
        if key==array[medio] {
            return Some(medio);
        }
        else if key> array[medio] {
            inizio=medio+1;
        }
        else{
            fine = medio;
        }
    }
    None
}*/

pub fn find(array: &[i32], key: i32) -> Option<usize> {       //ricorsiva
    let lunghezza = array.len();
    let medio = lunghezza/2;
    if lunghezza < 2 {
        return if lunghezza == 1 && array[0] == key {
            Some(medio)
        } else {
            None
        }
    }
    return if key == array[medio] {
        Some(medio)
    } else if key > array[medio] {
        let array_slice = &array[medio + 1..lunghezza];
        //aumento risultato di lunghezza/2 quando vado a dx, e aggiungo 1 perché l'indice parte da 0 (?)
        let trovato = find(array_slice, key);
        match trovato {
            Some(x) => Some(x+lunghezza/2+1),
            None => None
        }

    } else {
        let array_slice = &array[0..medio];
        find(array_slice, key)
    }
}
