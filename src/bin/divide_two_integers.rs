pub fn divide(dividend: i32, divisor:i32) ->  i32{
    if dividend  == i32::MIN && divisor == -1{
        return  i32::MAX;
    }     
    let negative = (dividend < 0) ^ (divisor < 0);
    let mut dividend = (dividend as i64).abs();
    let divisor = (divisor as i64).abs();
    
    let mut quotient: i64 = 0;
    for i in (0..32).rev(){
        if(divisor << i) <= dividend{
            dividend -= divisor << i;
            quotient +=  1_i64 <<  i;
        }
    }
    if negative{
        quotient =- quotient;
    }    
    quotient as i32 
} 