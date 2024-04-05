<!-- markdownlint-disable MD041 -->
<p align='center'>
    <a herf=''>
        <img src='./docs/nazrin.webp' width='250px' height='250px' alt='nazrin'>
    </a>
</p>

<div align="center">

# Nazrin

<!-- markdownlint-disable-next-line MD036 -->
**中文分词工具 [jieba-rs](https://github.com/messense/jieba-rs) Binding of Python**

</div>

相比纯 Python 实现的 [jieba](https://github.com/fxsjy/jieba)，速度更快，在分词过程中释放了 GIL，可适用于多线程处理

## 安装

```bash
pip install nazrin
```

## 用法

```python
from nazrin import Nazrin

nazrin = Nazrin()
print(nazrin.cut('能找到想找的东西程度的能力'))
# ['能', '找到', '想', '找', '的', '东西', '程度', '的', '能力']

print(nazrin.tag('能找到想找的东西程度的能力'))
# [('能', 'v'), ('找到', 'v'), ('想', 'v'), ('找', 'v'), ('的', 'uj'), ('东西', 'ns'), ('程度', 'n'), ('的', 'uj'), ('能力', 'n')]
```

<details>

<summary>全部方法介绍</summary>

```python
class Nazrin:
    def __init__(self) -> None: ...
    def add_word(
        self, word: str, freq: int | None = None, tag: str | None = None
    ) -> int:
        """
        说明：

            把一个词加进字典。

        参数:

            * ``freq``: 词频，默认为计算值
            * ``tag``: 词性，默认为 None

        """
        ...
    def load_userdict(self, path: str) -> None:
        """
        说明：

            加载用户字典

        参数:

            * ``path``: 字典路径

        """
        ...
    def suggest_freq(self, word: str) -> None:
        """
        说明：

            建议词频，以强制词语中的字符连接或分离。

        参数:

            * ``word``: 词语

        """
        ...
    def cut(self, text: str, hmm: bool = True) -> list[str]:
        """
        说明：

            将包含汉字的整个句子分割成独立的单词，精确模式

        参数:

            * ``text``: 文本
            * ``hmm``: 是否使用隐马尔可夫模型. 默认为 True.

        """
        ...
    def cut_all(self, text: str) -> list[str]:
        """
        说明：

            将包含汉字的整个句子分割成独立的单词，完整模式

        参数:

            * ``text``: 文本

        """
        ...
    def cut_for_search(self, text: str, hmm: bool = True) -> list[str]:
        """
        说明：

            将包含汉字的整个句子分割成独立的单词，搜索引擎模式

        参数:

            * ``text``: 文本
            * ``hmm``: 是否使用隐马尔可夫模型. 默认为 True.

        """
        ...
    def tag(self, text: str, hmm: bool = True) -> list[tuple[str, str]]:
        """
        说明：

            给文本打词性标签

        参数:

            * ``text``: 文本
            * ``hmm``: 是否使用隐马尔可夫模型. 默认为 True.

        """
        ...
    def tokenize(
        self,
        text: str,
        mode: Literal["search", "default"] = "default",
        hmm: bool = True,
    ) -> list[str]:
        """
        说明：

            Tokenize the text

        参数:

            * ``text``: 文本呢
            * ``mode``: 模式. 默认为 "default".
            * ``hmm``: 是否使用隐马尔可夫模型. 默认为 True.

        """

```

</details>

# 性能对比

```python
In [1]: import jieba

In [2]: jieba.initialize()
Building prefix dict from the default dictionary ...
Loading model from cache jieba.cache
Loading model cost 0.647 seconds.
Prefix dict has been built successfully.

In [3]: from nazrin import Nazrin

In [4]: nazrin = Nazrin()

In [5]: with open("./docs/performance-test.txt", "r", encoding="utf-8") as f:
   ...:     data = f.read()
   ...:

In [6]: %timeit list(jieba.cut(data))
3.77 ms ± 109 µs per loop (mean ± std. dev. of 7 runs, 100 loops each)

In [7]: %timeit nazrin.cut(data)
283 µs ± 14.5 µs per loop (mean ± std. dev. of 7 runs, 1,000 loops each)
```

# 鸣谢

naidesu
