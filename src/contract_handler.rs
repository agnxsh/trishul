use ethers::types::Bytes;
pub struct Contract {
    pub bytecode: Bytes,
}
///wrapper type around Bytes to deserialize/serialize "0x" prefixed ethereum to strings

impl Contract {
    pub fn new(mut self, bytecode: Bytes) -> Self {
        self.bytecode = bytecode;
        self
    }

    pub fn get_smart_contract_dispatcher(self, design_pattern: Vec<u8>){
        search(design_pattern, self.bytecode);
    }

}

///computing the pattern array first
fn compute_pattern_array(design_pattern: &Vec<u8>, m : usize) -> Vec<i32> {
    let mut len = 0;
    let mut i = 1;
    let mut pattern_array: Vec<i32> = vec![0];

    loop {
        if i >= m {
            return pattern_array;
        }
        if design_pattern[i] == design_pattern[len] {
            len += 1;
            pattern_array.push(len as i32);
            i += 1;
        } else {
            if len != 0 {
                len = pattern_array[len-1 as usize] as usize;
            } else {
                pattern_array.push(len as i32);
                i += 1;
            }
        }
    }
}

///helper function to search for function signatures from the bytecode
fn search (design_pattern: Vec<u8>, bytecode: Bytes) {
    let m = design_pattern.iter().len();
    let n = bytecode.iter().len();

    let mut i = 0;
    let mut j = 0;

    let mut signature : Bytes = Bytes::from(bytecode[0 as usize..1 as usize].to_vec());

    let pattern_array : Vec<i32> = compute_pattern_array(&design_pattern, m);

    loop {
        if n-i < m - j {
            break;
        }

        if design_pattern[j] == bytecode[i]{
            if design_pattern[j] == 0x63 {
                let data_size = (0x63 - 0x60) + 1;
                signature = Bytes::from(bytecode[i + 1 as usize..i + data_size + 1].to_vec());
                i = i + data_size + 1
            } else if design_pattern[j] == 0x61 {
                let data_size = (0x61 - 0x60) + 1;
                i = i + data_size + 1;
            } else {
                i += 1;
            }
            j += 1;
        }

        if j == m {
            println!("Signature {}", signature);
            j = pattern_array[j-1] as usize;
        } else if i < n && design_pattern[j] != bytecode[i] {
            if j != 0 {
                j = pattern_array[j-1] as usize;
            } else {
                i += 1;
            }
        }
    }
}