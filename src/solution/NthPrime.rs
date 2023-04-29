pub fn nth(n: u32) -> u32 {
    let n = n+1;
    let mut count = 0;
    let mut num = 2;
    while count<n {
        if isPrime(num) {
            count+=1;
            num+=1;
        } else {
            num+=1;
        }
    }
    return num-1;
}
fn isPrime(n:u32) -> bool {
    if n<=1 {
        false
    } else {
        let t = n/2;
        for i in 2..=t {
            if n%i==0 {
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(nth(5), 13);
    }
}

/*
定义一个函数来判断一个数是否为素数。一个数n是素数，当且仅当它只能被1和它本身整除，即没有其他的因子。

使用一个列表来存储已经找到的素数。从2开始遍历自然数，如果这个数是素数，则加入素数列表中。

遍历自然数，直到素数列表中的元素数量达到n。每次遍历时，判断这个数是否为素数，如果是，则加入素数列表中。

循环结束后，素数列表中的第n个元素就是要求的第n个素数。
**/