#[derive(Debug)]
enum Var {
    A,
    B,
}

#[derive(Debug)]
enum Const {
    Plus,
    Minus,
}
    
#[derive(Debug)]
enum Command {
    Var(Var),
    Const(Const),
}

fn main() {
    let mut output: Vec<Command> = vec!(Command::Var(Var::A));
    let iterations = 7;
    for _ in 0..iterations {
         output = output.iter().flat_map(conversion).collect()
    }

    for i in output {
        println!("{}", match i {
            Command::Const(a) => match a {
                Const::Plus => "lt 60",
                Const::Minus => "rt 60"
            },
            Command::Var(a) => "fd 4"
        });
    }
}

fn conversion(x: &Command) -> Vec<Command> {
    match x {
        &Command::Const(ref a) => match a {
            &Const::Plus => vec!(Command::Const(Const::Plus)),
            &Const::Minus => vec!(Command::Const(Const::Minus))
        },
        &Command::Var(ref a) => match a {
            &Var::A => vec!(
                Command::Const(Const::Plus),
                Command::Var(Var::B),
                Command::Const(Const::Minus),
                Command::Var(Var::A),
                Command::Const(Const::Minus),
                Command::Var(Var::B),
                Command::Const(Const::Plus)
            ),
            &Var::B => vec!(
                Command::Const(Const::Minus),
                Command::Var(Var::A),
                Command::Const(Const::Plus),
                Command::Var(Var::B),
                Command::Const(Const::Plus),
                Command::Var(Var::A),
                Command::Const(Const::Minus),
            )
        }
    }
}
