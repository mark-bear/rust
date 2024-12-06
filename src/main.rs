fn is_prime(x:u128)->bool{
    if x==0||x==1||x==4{
        return false;
    }
    if x==2||x==3{
        return true;
    }
    if x%6!=1&&x%6!=5{
        return false;
    }
    let mut i:u128=5;
    while i*i<=x{
        if x%i==0||x%(i+2)==0{
            return false;
        }
        i+=6;
    }
    return true;
}

fn meisen(x:u128)->u128{
    let mut i=1;
    let mut meisen=2;
    while i<x {
        meisen*=2;
        i+=1;
    }
    return meisen-1;
}

fn main(){
    let mut i=0;
    while i<1024{
        let j=meisen(i);
        if is_prime(j){
            println!("{}:{}",i,j);
        }
        i+=1;
    }
    println!("END");
}

