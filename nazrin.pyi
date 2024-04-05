from typing import Literal

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
