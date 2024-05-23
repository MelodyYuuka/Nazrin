use jieba_rs::KeywordExtractConfig;
use std::{fs::File, io::BufReader, path::PathBuf};

use pyo3::{prelude::*, types::PyList};

mod tfidf;

#[pyclass(subclass)]
struct Nazrin {
    jieba: jieba_rs::Jieba,
}

#[pymethods]
impl Nazrin {
    #[new]
    fn new() -> Self {
        Self {
            jieba: jieba_rs::Jieba::new(),
        }
    }

    #[pyo3(signature = (word, freq = None, tag = None))]
    fn add_word(
        &mut self,
        py: Python,
        word: &str,
        freq: Option<usize>,
        tag: Option<&str>,
    ) -> usize {
        py.allow_threads(move || self.jieba.add_word(word, freq, tag))
    }

    fn load_userdict(&mut self, py: Python, path: PathBuf) -> PyResult<()> {
        py.allow_threads(move || {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            match self.jieba.load_dict(&mut reader) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Failed to load userdict: {}",
                        e
                    )));
                }
            }
        })
    }

    fn suggest_freq(&mut self, py: Python, segment: &str) -> usize {
        py.allow_threads(move || self.jieba.suggest_freq(segment))
    }

    /// Cut the input text
    #[pyo3(signature = (text, hmm = true))]
    fn cut<'a>(&self, py: Python, text: &'a str, hmm: bool) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut(text, hmm))
    }

    /// Cut the input text, return all possible words
    #[pyo3(signature = (text,))]
    fn cut_all<'a>(&self, py: Python, text: &'a str) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut_all(text))
    }

    /// Cut the input text in search mode
    #[pyo3(signature = (text, hmm = true))]
    fn cut_for_search<'a>(&self, py: Python, text: &'a str, hmm: bool) -> Vec<&'a str> {
        py.allow_threads(move || self.jieba.cut_for_search(text, hmm))
    }

    /// Tag the input text
    #[pyo3(signature = (text, hmm = true))]
    fn tag<'a>(&'a self, py: Python, text: &'a str, hmm: bool) -> Vec<(&'a str, &'a str)> {
        py.allow_threads(move || {
            self.jieba
                .tag(text, hmm)
                .into_iter()
                .map(|t| (t.word, t.tag))
                .collect()
        })
    }

    /// Tokenize
    #[pyo3(signature = (text, mode = "default", hmm = true))]
    fn tokenize<'a>(
        &self,
        py: Python,
        text: &'a str,
        mode: &str,
        hmm: bool,
    ) -> Vec<(&'a str, usize, usize)> {
        let tokenize_mode = if mode.to_lowercase() == "search" {
            jieba_rs::TokenizeMode::Search
        } else {
            jieba_rs::TokenizeMode::Default
        };
        py.allow_threads(move || {
            self.jieba
                .tokenize(text, tokenize_mode, hmm)
                .into_iter()
                .map(|t| (t.word, t.start, t.end))
                .collect()
        })
    }
}

#[pyclass]
struct TFIDF {
    inner: tfidf::TfIdf,
}

#[pymethods]
impl TFIDF {
    #[new]
    #[pyo3(signature = (path = None))]
    fn new(py: Python, path: Option<PathBuf>) -> PyResult<Self> {
        py.allow_threads(|| {
            let mut buffer = None;
            if let Some(path) = path {
                buffer = Some(BufReader::new(File::open(path)?));
            }
            Ok(Self {
                inner: tfidf::TfIdf::new(buffer.as_mut(), KeywordExtractConfig::default()),
            })
        })
    }

    fn load_dict(&mut self, py: Python, path: PathBuf) -> PyResult<()> {
        py.allow_threads(|| {
            let mut buffer = BufReader::new(File::open(path)?);
            self.inner.load_dict(&mut buffer)?;
            Ok(())
        })
    }

    #[pyo3(signature = (nazrin, sentence, top_k = 20, allow_pos = None))]
    fn extract_tags(
        &self,
        nazrin: &Nazrin,
        sentence: &str,
        top_k: usize,
        allow_pos: Option<&Bound<'_, PyList>>,
    ) -> PyResult<Vec<(String, f64)>> {
        let allow_pos: Vec<String> = match allow_pos {
            Some(allow_pos) => allow_pos.extract()?,
            None => Vec::new(),
        };
        Ok(self
            .inner
            .extract_keywords(&nazrin.jieba, sentence, top_k, allow_pos))
    }
}

#[pymodule]
fn nazrin(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Nazrin>()?;
    m.add_class::<TFIDF>()?;
    Ok(())
}
