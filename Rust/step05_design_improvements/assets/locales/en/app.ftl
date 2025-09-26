intro = Hello! I’m Guillaume.

audience = { $audience ->
    [business] In a business setting, I’m direct and strategic — I focus on results and clarity.
    [academic] In an academic setting, I’m reflective and analytical — I enjoy digging into complex ideas.
   *[other]   I adapt my style to the situation.
}

fluency = { $level ->
    [fluent] I can work in { lang-name } with ease — meetings, writing, or negotiation.
    [learning] I’m learning { lang-name } steadily — enough to explore ideas and hold conversations.
   *[other] I’m still building my { lang-name } skills.
}

language-profile = { $audience ->
    [business] { $lang ->
        [fr] Native French speaker, fluent in English for professional contexts.
        [en] Fluent English, equal to French in my daily and professional life.
       *[other] (In business I work primarily in French and English.)
    }
    [academic] { $lang ->
        [fr] Native French, active in academic writing and analysis.
        [en] Fluent English, experienced in academic discussion and research.
        [ru] Russian studied for its literature and philosophical tradition.
        [zh] Chinese studied to engage with Asian academic perspectives.
        [it] Italian for cultural history and intellectual curiosity.
       *[other] Other language experience.
    }
   *[other] { $lang ->
        [fr] French speaker.
        [en] English speaker.
       *[other] Language skill.
    }
}
hireme = I'm available for hire!
website = Hire me -> guillaume.maiano.fr

table-language = Language
table-level = Level

-lang-name-fr = French
-lang-name-en = English
-lang-name-ru = Russian
-lang-name-zh = Chinese
-lang-name-it = Italian

lang-name = { $lang ->
   [fr] { -lang-name-fr }
   [en] { -lang-name-en }
   [ru] { -lang-name-ru }
   [zh] { -lang-name-zh }
   [it] { -lang-name-it }
  *[other] ???
}