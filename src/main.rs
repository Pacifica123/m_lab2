use std::{cmp::max, fs::File, io::{self, BufRead, BufReader, Write}, net};

use rand::Rng;

// ======================
// СТАНДАРТНЫЙ МЕТОД
fn factorial(n: f64) -> u128 {
    if n == 0.0 {
        1
    }
    else {
        n as u128 * factorial(n - 1.0)
    }
}

fn phi(p: f64, m: u128, i: u128) -> f64 {
    // i = 0; //для первого отрезка от 0 до p0 (с первой попытки получить m удач)
    let combination = factorial(m as f64 + i as f64 - 1 as f64) / (factorial((m-1) as f64) * factorial(i as f64));
    let pi = p.powi(m as i32) * (1.0 - p).powi(i as i32) * combination as f64;
    pi 
}

fn standart_method(p: f64, m: u128) -> u128{
    let mut i = 0; // количество попыток - номер отрезка за которым присвоенна сама величина
    // let mut h = phi(p, m, i); // p0
    let mut h = p.powi(m as i32);
    let r = rand::thread_rng().gen_range(0.0..1.0);    
    while r > h as f64 {
        i += 1;
        h += phi(p, m, i); //+pi
    }
    i
}

fn get_random_standart(p: f64, m: u128, n: u128) -> Vec<u128> {
    let R: Vec<u128> = (0..n).map(|_| standart_method(p, m)).collect();
    R
}

// ======================
// СПЕЦИАЛЬНЫЙ МЕТОД
fn special_method(p: f64, m: u128) -> u128{
    let mut k = 0; // количество неуспехов
    let mut u: u128 = 0; // количество успехов
    while u < m {
        let r = rand::thread_rng().gen_range(0.0..1.0);
        if r <= p {
            u += 1;
        }
        else {
            k += 1;
        }
    }
    k
}

fn get_random_special(p: f64, m: u128, n: u128) -> Vec<u128> {
    let R: Vec<u128> = (0..n).map(|_| special_method(p, m)).collect();
    R
}

// ======================
// СТАТИСТИЧЕСКИЕ ФУНКЦИИ

fn mean(R: &Vec<u128>) -> f64 {
    let mut sum = 0;
    for i in R.iter() {
        sum += i;
    }
    sum as f64 / R.len() as f64
}
fn dispersion(R: &Vec<u128>) -> f64 {
    let m = mean(R);
    let mut sum_of_squares: f64 = 0.0;
    for x in R {
        let diff = *x as f64 - m;
        sum_of_squares += diff * diff;
    }
    sum_of_squares / R.len() as f64
}
fn max_in_vec(R: &Vec<u128>) -> u128 {
    let mut max_el = 0;
    for el in R{
        if el>&max_el {max_el = *el}
    }
    max_el
}
fn nozero_count_in_vec(R: &Vec<u128>) -> u128 {
    let mut count = 0;
    // for el in R{
    //     if el!=&0 {count+=1}
    // }
    for i in 0..R.len(){
        for el in R {
            if *el as u128 == i as u128 {count+=1; break}
        }
    }
    count
}

fn hi_sqr(R: &Vec<u128>, p: f64, m: u128) -> f64 {
    // критерий пирсона
    let mut ni_count = 0;
    let mut hi_sqr = 0.0;


    for i in 0..nozero_count_in_vec(R) {
        ni_count = 0;
        for j in 0..R.len(){
            if (i == R[j]) {ni_count+=1}
        }
        println!("n{:?} = {:?}", i, ni_count);
        let pi = phi(p, m, i); // TODO можно просто один раз посчитать вероятности и занести их в массив?
        let n_normal = R.len() as f64*pi; //Npi
        hi_sqr += (ni_count as f64 - n_normal).powf(2.0) / n_normal
    }
    hi_sqr
}
// ======================
// РАБОТА С ФАЙЛОМ
fn read_params(filename: &str) -> Result<(f64, u128, u128, Vec<f64>), io::Error>{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let params = lines.next().expect("Первая строка отсутствует!")?;
    let hi_sqr_theory_line = lines.next().expect("Вторая строка отсутствует!")?;
    
    let p: f64 = params.split_whitespace().next().unwrap().parse().expect("Некорректное значение p");
    let m: u128 = params.split_whitespace().nth(1).unwrap().parse().expect("Некорректное значение m");
    let n = params.split_whitespace().nth(2).unwrap().parse().expect("Некорректное значение N");

    //  табличные значения для критерия Пирсона
    let hi_sqr_theory: Vec<f64> = hi_sqr_theory_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    Ok((p, m, n, hi_sqr_theory))
}

fn write_to_file(filename: &str, results: String)
-> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(results.as_bytes())?;
    Ok(())
}


fn main() {
    let filename = "params.txt";
    let (p, m, n, hi_sqr_theory) = read_params(filename).unwrap();
    let R_standart = get_random_standart(p, m, n);
    let R_special = get_random_special(p, m, n);
    println!("Выборка стандартная: {:?}", R_standart);
    println!("Выборка специальная: {:?}", R_special);
    let k_standart = nozero_count_in_vec(&R_standart); // кол-во отрезков 
    let hs_standart = hi_sqr(&R_standart, p, m);

    let k_special = nozero_count_in_vec(&R_special);
    let hs_special = hi_sqr(&R_special, p, m);

    let mut results = String::new();
    results.push_str(&format!("Входные параметры: p = {}, m = {}, N = {}\n", p, m, n));
 
    let mean_theory = m as f64*(1.0-p)/p; 
    let dispr_theory = m as f64*(1.0-p)/(p*p);
    
    results.push_str(&format!("Математическое ожидание (теоретическое): {}\n", mean_theory));
    results.push_str(&format!("Дисперсия (теоретическая): {}\n", dispr_theory));
    results.push_str(&format!("===========================================================\n"));    
    results.push_str(&format!("Выборка стандартная: {:?}\n", &R_standart));
    results.push_str(&format!("Математическое ожидание (расчетное): {}\n", mean(&R_standart)));
    results.push_str(&format!("Дисперсия (расчетная): {}\n", dispersion(&R_standart)));
    results.push_str(&format!("Уровень значимости: {}\n", 0.05));
    results.push_str(&format!("Критерий Пирсона: {}\n", hs_standart));
    results.push_str(&format!("Критерий Пирсона(теор): {}\n", hi_sqr_theory[k_standart as usize -1]));
    results.push_str(&format!("===========================================================\n"));
    results.push_str(&format!("Выборка специальная: {:?}\n", &R_special));
    results.push_str(&format!("Математическое ожидание (расчетное): {}\n", mean(&R_special)));
    results.push_str(&format!("Дисперсия (расчетная): {}\n", dispersion(&R_special)));
    results.push_str(&format!("Уровень значимости: {}\n", 0.05));
    results.push_str(&format!("Критерий Пирсона: {}\n", hs_special));
    results.push_str(&format!("Критерий Пирсона(теор): {}\n", hi_sqr_theory[k_special as usize -1]));
    write_to_file("results.txt", results);
}
