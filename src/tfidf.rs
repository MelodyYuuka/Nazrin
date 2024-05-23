// 对 https://github.com/messense/jieba-rs/blob/main/src/keywords/tfidf.rs 的改写以符合使用习惯

use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};
use std::io::{self, BufRead, BufReader};

use ordered_float::OrderedFloat;

use jieba_rs::Jieba;
use jieba_rs::KeywordExtractConfig;

use ahash::AHashMap as HashMap;

static DEFAULT_IDF: &str = include_str!("../data/idf.txt");

#[derive(Debug, Clone, Eq, PartialEq)]
struct HeapNode<'a> {
    tfidf: OrderedFloat<f64>,
    word: &'a str,
}

impl<'a> Ord for HeapNode<'a> {
    fn cmp(&self, other: &HeapNode) -> Ordering {
        other
            .tfidf
            .cmp(&self.tfidf)
            .then_with(|| self.word.cmp(other.word))
    }
}

impl<'a> PartialOrd for HeapNode<'a> {
    fn partial_cmp(&self, other: &HeapNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// TF-IDF keywords extraction
///
/// Require `tfidf` feature to be enabled
#[derive(Debug)]
pub struct TfIdf {
    idf_dict: HashMap<String, f64>,
    median_idf: f64,
    config: KeywordExtractConfig,
}

/// Implementation of JiebaKeywordExtract using a TF-IDF dictionary.
///
/// This takes the segments produced by Jieba and attempts to extract keywords.
/// Segments are filtered for stopwords and short terms. They are then matched
/// against a loaded dictionary to calculate TF-IDF scores.
impl TfIdf {
    pub fn new(opt_dict: Option<&mut impl BufRead>, config: KeywordExtractConfig) -> Self {
        let mut instance = TfIdf {
            idf_dict: HashMap::default(),
            median_idf: 0.0,
            config,
        };
        if let Some(dict) = opt_dict {
            instance.load_dict(dict).unwrap();
        } else {
            instance
                .load_dict(&mut BufReader::new(DEFAULT_IDF.as_bytes()))
                .unwrap();
        }
        instance
    }

    pub fn load_dict(&mut self, dict: &mut impl BufRead) -> io::Result<()> {
        let mut buf = String::new();
        let mut idf_heap = BinaryHeap::new();
        while dict.read_line(&mut buf)? > 0 {
            let parts: Vec<&str> = buf.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let word = parts[0];
            if let Some(idf) = parts.get(1).and_then(|x| x.parse::<f64>().ok()) {
                self.idf_dict.insert(word.to_string(), idf);
                idf_heap.push(OrderedFloat(idf));
            }

            buf.clear();
        }

        let m = idf_heap.len() / 2;
        for _ in 0..m {
            idf_heap.pop();
        }

        self.median_idf = idf_heap.pop().unwrap().into_inner();

        Ok(())
    }
}

/// TF-IDF keywords extraction.
///
/// Require `tfidf` feature to be enabled.
impl Default for TfIdf {
    /// Creates TfIdf with DEFAULT_STOP_WORDS, the default TfIdf dictionary,
    /// 2 Unicode Scalar Value minimum for keywords, and no hmm in segmentation.
    fn default() -> Self {
        let mut default_dict = BufReader::new(DEFAULT_IDF.as_bytes());
        TfIdf::new(Some(&mut default_dict), KeywordExtractConfig::default())
    }
}

impl TfIdf {
    #[inline]
    fn filter_word(&self, s: &str) -> bool {
        s.chars().count() >= self.config.min_keyword_length()
            && !self.config.stop_words().contains(&s.to_lowercase())
    }

    pub fn extract_keywords(
        &self,
        jieba: &Jieba,
        sentence: &str,
        top_k: usize,
        allowed_pos: Vec<String>,
    ) -> Vec<(String, f64)> {
        let tags = jieba.tag(sentence, self.config.use_hmm());
        let mut allowed_pos_set = BTreeSet::new();

        for s in allowed_pos {
            allowed_pos_set.insert(s);
        }

        let mut term_freq: HashMap<String, u64> = HashMap::default();
        for t in &tags {
            if !allowed_pos_set.is_empty() && !allowed_pos_set.contains(t.tag) {
                continue;
            }

            if !self.filter_word(t.word) {
                continue;
            }

            let entry = term_freq.entry(String::from(t.word)).or_insert(0);
            *entry += 1;
        }

        let total: u64 = term_freq.values().sum();
        let mut heap = BinaryHeap::new();
        for (cnt, (k, tf)) in term_freq.iter().enumerate() {
            let idf = self.idf_dict.get(k).unwrap_or(&self.median_idf);
            let node = HeapNode {
                tfidf: OrderedFloat(*tf as f64 * idf / total as f64),
                word: k,
            };
            heap.push(node);
            if top_k != 0 && cnt >= top_k {
                heap.pop();
            }
        }

        let mut res = Vec::new();
        if top_k != 0 {
            for _ in 0..top_k {
                if let Some(w) = heap.pop() {
                    res.push((String::from(w.word), w.tfidf.into_inner()));
                } else {
                    break;
                }
            }
        } else {
            res.extend(
                heap.into_iter()
                    .map(|w| (String::from(w.word), w.tfidf.into_inner())),
            );
        }

        res.reverse();
        res
    }
}
