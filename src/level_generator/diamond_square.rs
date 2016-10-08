use ::rand::random;

pub fn generate_ds(size: usize) -> Vec<Vec<f64>> {
    // Первая цифра - столбец, вторая - строка
    let mut array = Vec::new();
    for _ in 0..size {
        let mut tmp_array = Vec::new();
        for _ in 0..size {
            tmp_array.push(random::<f64>());
        }
        array.push(tmp_array);
    }

    let mut step_size = size - 1;
    let mut x = 0;
    let mut y = 0;
    while step_size > 1 {
        while y < size - 1 {
            while x < size - 1 {
                step_square(x, y, step_size, &mut array);
                x += step_size;
            }
            y += step_size;
            x = 0;
        }
        x = 0;
        y = 0;
        while y < size - 1 {
            while x < size - 1 {
                step_diamond(x as isize - (step_size / 2) as isize,
                             y as isize,
                             step_size,
                             &mut array);
                step_diamond(x as isize,
                             y as isize - (step_size / 2) as isize,
                             step_size,
                             &mut array);
                step_diamond(x as isize + (step_size / 2) as isize,
                             y as isize,
                             step_size,
                             &mut array);
                step_diamond(x as isize,
                             y as isize + (step_size / 2) as isize,
                             step_size,
                             &mut array);
                x += step_size;
            }
            y += step_size;
            x = 0;
        }
        step_size = step_size / 2;
        y = 0;
        x = 0;
    }
    array
}

fn step_square(x: usize, y: usize, size: usize, array: &mut Vec<Vec<f64>>) {
    // square
    // a     b
    //    x
    // c     d

    let a = array[x][y];
    let b = array[x][y + size];
    let c = array[x + size][y];
    let d = array[x + size][y + size];

    array[x + (size / 2)][y + (size / 2)] += (a + b + c + d) / 4.0;
}

fn step_diamond(x: isize, y: isize, size: usize, array: &mut Vec<Vec<f64>>) {
    let hs = size / 2;
    // diamond
    //    b
    // a  x  c
    //    d


    let a = get_element(x, y + hs as isize, &array);
    let b = get_element(x + hs as isize, y, &array);
    let c = get_element(x + size as isize, y + hs as isize, &array);
    let d = get_element(x + hs as isize, y + size as isize, &array);

    array[(x + hs as isize) as usize][(y + hs as isize) as usize] += (a + b + c + d) / 4.0;
}

fn get_element(x: isize, y: isize, array: &Vec<Vec<f64>>) -> f64 {
    if x < 0 || y < 0 {
        return 0.0;
    }
    let x = x as usize;
    let y = y as usize;
    match array.get(x) {
        Some(e) => {
            match e.get(y) {
                Some(f) => f.clone(),
                None => 0.0,
            }
        }
        None => 0.0,
    }
}