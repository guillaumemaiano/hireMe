intro = 你好！我是 Guillaume。

audience = { $audience ->
    [business] 在商务场合，我直接而有战略眼光 —— 注重结果与清晰。
    [academic] 在学术环境中，我思考深入、善于分析 —— 喜欢探讨复杂的想法。
   *[other]   我会根据不同情况调整自己的风格。
}

fluency = { $level ->
    [fluent] 我能自如地使用 { $langName } 工作 —— 会议、写作或谈判。
    [learning] 我正在持续学习 { $langName } —— 足以探索思想并进行交流。
   *[other] 我还在提升自己的 { $langName } 水平。
}

language-profile = { $audience ->
    [business] { $lang ->
        [fr] 法语母语者，同时能熟练使用英语进行专业工作。
        [en] 英语流利，与法语同等水平，适用于日常与工作。
       *[other] （在商务中，我主要使用法语和英语。）
    }
    [academic] { $lang ->
        [fr] 法语母语者，活跃于学术写作与分析。
        [en] 英语流利，能够参与学术讨论与研究。
        [ru] 学习俄语，出于对文学和哲学传统的兴趣。
        [zh] 学习中文，是为了理解亚洲的整体视角，不想做井底之蛙。
        [it] 学习意大利语，出于对历史和文化的兴趣。
       *[other] 其他语言经验。
    }
   *[other] { $lang ->
        [fr] 法语使用者。
        [en] 英语使用者。
       *[other] 语言技能。
    }
}
hireme = 我可以参与新的项目！
website = 查看我的简介 -> guillaume.maiano.fr
