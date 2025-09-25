intro = Hello! I’m Guillaume.

audience = { $audience ->
    [business] In a business setting, I’m direct and strategic — I focus on results and clarity.
    [academic] In an academic setting, I’m reflective and analytical — I enjoy digging into complex ideas.
   *[other]   I adapt my style to the situation.
}

fluency = { $level ->
    [fluent] I can work in { $lang } with ease — meetings, writing, or negotiation.
    [learning] I’m learning { $lang } steadily — enough to explore ideas and hold conversations.
   *[other] I’m still building my { $lang } skills.
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