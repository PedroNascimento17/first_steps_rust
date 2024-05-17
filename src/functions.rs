fn _count(num: i32) {
    for x in 1..=num{
        println!("Contando {}", x);
    }
}

fn _count_down(mut num: i32) {
    while num > 0 {
        println!("Contando {}", num);
        num -= 1;
    }
}

fn _maior_numero(nums: &[i32]) -> i32 {
    let mut maior: i32 = nums[0];

    for num in nums {
        if num > &maior {
            maior = *num;
        }
    }

    maior
}

fn _eh_primo(num: i32) -> bool {
    match num {
        ..=1 => false,
        _ => {
            let limit: i32 = (num as f32).sqrt() as i32 + 1;
            for x in 2..limit {
                if num % x == 0 {
                    return false;
                }
            }
            true
        }
    }  
}

fn _multiplication(num: i32) {
    for x in 0..=10 {
        println!("{} x {} = {}", num, x, num * x)
    }
}

fn _media(grades: &[f32]) -> f32 {
    let mut sum: f32 = 0.0;

    for grade in grades {
        sum += sum;
    }

    sum / grades.len() as f32
}