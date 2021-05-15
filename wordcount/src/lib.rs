//! wordcount はシンプルな文字、単語、行の出現頻度の計数機能を提供します
//! 詳しくは[`count`](fn.count.html)関数のドキュメントを見てください。
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// [`count`](fn.count.html)で使うオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// 文字ごとに頻度を数える
    Char,
    /// 単語ごとに頻度を数える
    Word,
    /// 行ごとに頻度を数える
    Line,
}

/// オプションのデフォルトは[`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// input から一行ずつUTF-8文字列を読み込み、頻度を数える
///
/// 頻度を数える大賞はオプションによって制御される。
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicodeの1文字ごと
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): 正規表現 \w+ にマッチする単語ごと
/// * [`CountOption::Line`](enum.CountOption.html#variant.Lin): \n または \r\n で区切られた一行ごと
///
/// # Panics
///
/// 入力が UTF-8 でフォーマットされていない場合にパニックする
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn word_count_works() {
        use std::io::Cursor;

        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("bb".to_string(), 2);
        exp.insert("cc".to_string(), 1);
        assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
    }
}
