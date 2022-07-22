extern crate rand;

use std::io;
use rand::Rng;

//정답 자릿수 설정
const NUM_COUNT: usize = 3;

#[derive(PartialEq)]
enum InputError {
    ArgumentNumberError,
    FormatError,
}

//유저의 입력값을 받아오는 함수
fn get_user_input() -> Result<Vec<u8>, InputError> {
    let mut user_input: String = String::new();
    //값 입력
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    //입력값을 공백 기준으로 나누어 벡터에 넣음
    let input_number: Vec<&str> = user_input.trim().split(' ').collect();

    //벡터에 들어있는 값을 확인
    if input_number.len() == NUM_COUNT {
        let mut num_list:Vec<u8> = Vec::new();
        for num_str in input_number {
            //str 타입을 10진수로 바꿔줌
            match u8::from_str_radix(num_str,10) {
                Ok(num) => { num_list.push(num); },
                //입력값이 10진수로 바꿀 수 없는 값이라면 FormatError
                Err(_) => { return Err(InputError::FormatError); },
            };
        }
        Ok(num_list)
    }
    else {
        //10진수로 바꿀 수 있지만 벡터에 들어있는 값의 개수가 NUM_COUNT와 다르다면
        //인자의 개수를 잘못 입력한 것이므로 ArgumentNumberError
        Err(InputError::ArgumentNumberError)
    }
}

//유저의 입력값과 정답을 비교하는 함수
fn compare_input(answer: &Vec<u8>, input_numbers: &Vec<u8>) -> (u8, u8) {
	let mut strike = 0;
	let mut ball = 0;

	for (idx, num) in input_numbers.iter().enumerate() {
		match answer.iter().find(|&&x| x == *num) {
			Some(_) => {
                //숫자가 정답 안에도 있고 자리까지 정확하면 strike +1
				if answer[idx] == *num { strike += 1; }
                //숫자가 정답 안에 있지만 자리는 다르다면 ball +1
				else { ball += 1; }
			},
			None => {},
		}
	}
	(strike, ball)
}

fn main() {
    let mut answer: Vec<u8> = Vec::new();
    //랜덤으로 정답 생성
    while answer.len() < NUM_COUNT {
        let number = rand::thread_rng().gen_range(0, 9);
        match answer.iter().find(|&&x| x == number) {
            Some(_) => {},
            None => answer.push(number),
        }
    }

    let mut attempts = 0;

    //정답을 맞출때까지 loop 반복
    loop {
        println!("Guess the number");
        let user_input = match get_user_input() {
			Ok(input) => input,
            Err(InputError::FormatError) => {
                println!("invalid number format");
				continue;
            }
			Err(InputError::ArgumentNumberError) => {
                println!("input {} numbers", NUM_COUNT);
				continue;
			}
		};

        //시도횟수 +1
        attempts += 1;

        //맞추기를 시도할때마다 ball/strike 수 출력
        let (strike, ball) = compare_input(&answer, &user_input);
		println!("{} strike {} ball", strike, ball);
        
        //정답이라면 정답 메시지 출력
        if strike == NUM_COUNT as u8 { 
			println!("You guessed right in {} attempts", attempts);
			return; 
		}
    }
}