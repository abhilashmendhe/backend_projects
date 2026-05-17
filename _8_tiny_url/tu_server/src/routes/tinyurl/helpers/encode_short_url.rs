pub fn encode_short_url() -> String {
    // 1. init code
    // let code_int_62 = ['0','1','2','3','4','5','6','7','8','9'];
    // let code_lchar_62 = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    // let code_uchar_62 = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

    let code62 = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let uuid = uuid7::uuid7();
    let uuid_bytes = uuid.as_bytes();
    let mut n = u128::from_be_bytes(*uuid_bytes);

    // println!("In encode_short_url n: {}", n);
    // println!("In encode_short_url short n >> 80: {}", n>>80);
    n = n >> 80;

    let mut buf = Vec::new();
    while n > 0 {
        let r = n % 62;
        buf.push(code62[r as usize]);
        n /= 62;
    }

    buf.into_iter().collect()
}
