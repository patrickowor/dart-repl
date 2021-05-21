pub fn check_brackets(code : &String) -> bool{
    let mut curly = 0;
    let mut circly = 0;
    let mut square = 0;
    let mut b_curly = 0;
    let mut b_square = 0;
    let mut b_circly = 0;
    
    
    for c in code.chars(){
        if c == '('{
            circly += 1;
        } else if c == ')' {
             b_circly += 1;
        } else if c == '[' {
            square += 1;   
        } else if c == ']' {
            b_square += 1;
        } else if c == '}' {
            b_curly += 1;
        } else if c == '{' {
            curly += 1;
        }
    }
    
    if curly == b_curly && circly == b_circly && square == b_square {
        false
    } else {
        true
    }
}
