
pub mod fileops;
pub mod httpops;


pub fn extract_id_from_url(url: &str,tail_str: &str) -> Option<u32> {
    // 找到最后一个 '/' 字符的位置
    if let Some(index) = url.rfind('/') {
        // 从最后一个 '/' 字符位置开始提取子字符串
        if let Some(substring) = url.get(index + 1..url.len() - tail_str.len()) { // 去除末尾的 ".html" 部分
            if let Ok(id) = substring.parse::<u32>() {
                return Some(id);
            }
        }
    }
    None
}


struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_val = self.curr + self.next;
        self.curr = self.next;
        self.next = new_val;
        Some(self.curr)
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn test_is_empty(){


        let r1 =String::new();
        assert_eq!( true, r1.is_empty());

    }

    #[test]
    fn test_extract_id_from_url(){
        assert_eq!( super::extract_id_from_url("/html/18312/9950547.html",".html").unwrap(),9950547);

    }
  


    #[test]
    fn test_fib() {

        let mut fib =super::Fibonacci::new();


        assert_eq!(fib.next(),Some(1));
    }

}
