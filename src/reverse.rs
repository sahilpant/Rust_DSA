pub fn reverse(val: String) -> String {
    let mut sr= Vec::new();
    for x in val.chars() {
        sr.push(x);
    }
    let mut temp:char;
    let mut i = 0; let mut j = sr.len() - 1;
    loop {
        if i >= j {break}
        else {
            println!("{i}{j}");
            temp = sr[i];
            sr[i] = sr[j];
            sr[j] = temp;
            i+=1;j-=1;
        }
    }

    let mut res = String::new();
    for s in sr{
        res.push(s);
    }
    res
}