const N: usize = 10;

fn main() {
    let mut state: [u8;N] = [0;N];
    state[N-1] = 1;
    for _ in 0..5 {
        state = next_state(state);
        println!("{:?}",state)
    }
}


fn calcul(b1: u8, b2: u8, b3: u8) -> u8 {
    match (b1, b2, b3) {
        (1, 1, 1) => 0,
        (1, 1, 0) => 1,
        (1, 0, 1) => 1,
        (1, 0, 0) => 0,
        (0, 1, 1) => 1,
        (0, 1, 0) => 1,
        (0, 0, 1) => 1,
        (0, 0, 0) => 0,
        _ => panic!("Les bits doivent être 0 ou 1"),
    }
}

fn next_state(tab:[u8;N]) -> [u8;N] {
    let mut next_tab: [u8;N]=[0;N];
    next_tab[0] = calcul(tab[0],tab[0],tab[1]);
    for i in 1..(N-1) {
        next_tab[i] = calcul(tab[i-1],tab[i],tab[i+1])
    }
    next_tab[N-1] = calcul(tab[N-2],tab[N-1],tab[N-1]);
    next_tab
}//choix aux bords a revoir ??

/* erreurs a retenir 
range ..
[0,n] 2 elements [0;N] N zéros
et type usize
*/