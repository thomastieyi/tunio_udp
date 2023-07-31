// 参数容器 
#[derive(Debug)]
  struct ParamContainer {
    _container_id: u16,
    _container_len: u8,
    _container_content: Vec<u8>
  }

#[derive(Debug)]
pub struct ExtProtoCfgOpts {
  _length: u16,
//   config_proto: u8,
  _pco_units: Vec<ParamContainer>
}

impl Default for ParamContainer {
    fn default() -> Self {
        Self {
            _container_id: 0,
            _container_len: 0,
            _container_content: Vec::new(),
        }
    }
}

impl Default for ExtProtoCfgOpts {
    fn default() -> Self {
        Self {
            _length: 0,
            _pco_units: Vec::new(),
        }
    }
}
  
  // 解析函数
pub fn parse_extended_pco(data: &[u8]) -> Option<ExtProtoCfgOpts> {
  
    let mut params = vec![];
    
    let mut _i = 3; // 前4字节是类型和长度
    let length = u16::from_be_bytes([data[1], data[2]]);
    // print!("{:#?}\n",length);
    let mut i = 4;
    // 解析附加参数列表
    while i < data.len() {
      let container_id = u16::from_be_bytes([data[i], data[i+1]]);
    // print!("{:#?}\n",container_id);

      let container_len = data[i+2];
      let container_content = &data[i+3..i+3+container_len as usize];
  
      let container = ParamContainer {
        _container_id: container_id,
        _container_len: container_len,
        _container_content: container_content.to_vec(),
      };
  
      params.push(container);
  
      i += 3 + container_len as usize;
    }
    let ext: ExtProtoCfgOpts = ExtProtoCfgOpts{
        _length:length,
        _pco_units:params
    };
    Some(ext)
    // 输出解析结果
  
  }