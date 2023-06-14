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
fn compute_pattern_array(design_pattern: &Vec<u8>, m : usize) -> Vec<i32>

///helper function to search for function signatures from the bytecode
fn search (design_pattern: Vec<u8>, bytecode: Bytes) {
    let m = design_pattern.iter().len();
    let n = bytecode.iter().len();

    let mut i = 0;
    let mut j = 0;

    let mut signature : Bytes = Bytes::from(bytecode[0 as usize..1 as usize].to_vec());

    let pattern_array : Vec<i32> = compute_pattern_array(&design_pattern, m);

    loop {
        if n-1 < m - j {
            break;
        }

        if design_pattern[j] = bytecode[i]{
            if design_pattern[j] == 0x63

        }
    }
}