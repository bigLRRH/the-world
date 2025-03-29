#[derive(Debug, Clone)]
struct RLEChunk {
    data: Vec<(u8, u32)>, // (体素类型, 连续长度)
}

impl RLEChunk {
    // 压缩原始体素数据
    fn compress(raw_data: &[u8]) -> Self {
        let mut compressed = Vec::new();
        let mut current = raw_data[0];
        let mut count = 1;

        for &v in raw_data.iter().skip(1) {
            if v == current {
                count += 1;
            } else {
                compressed.push((current, count));
                current = v;
                count = 1;
            }
        }
        compressed.push((current, count)); // 处理最后一组
        Self { data: compressed }
    }

    // 解压缩数据
    fn decompress(&self) -> Vec<u8> {
        let mut decompressed = Vec::new();
        for &(value, count) in &self.data {
            decompressed.extend(std::iter::repeat(value).take(count as usize));
        }
        decompressed
    }
}
