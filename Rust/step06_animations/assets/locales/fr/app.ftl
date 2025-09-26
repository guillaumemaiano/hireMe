intro = Bonjour ! Je m’appelle Guillaume.

audience = { $audience ->
    [business] En contexte professionnel, je suis direct et stratégique — je privilégie les résultats et la clarté.
    [academic] En contexte académique, je suis réfléchi et analytique — j’aime approfondir des idées complexes.
   *[other]   J’adapte mon style à la situation.
}

fluency = { $level ->
    [fluent] Je peux travailler en { lang-name} sans difficulté — réunions, rédaction ou négociation.
    [learning] J’apprends le { lang-name } régulièrement — assez pour explorer des idées et tenir une conversation.
   *[other] Je développe encore mes compétences en { lang-name }.
}

language-profile = { $audience ->
    [business] { $lang ->
        [fr] Français natif, également à l’aise en anglais dans un cadre professionnel.
        [en] Anglais courant, au même niveau que le français dans ma vie quotidienne et au travail.
       *[other] (En affaires, je travaille principalement en français et en anglais.)
    }
    [academic] { $lang ->
        [fr] Français natif, actif en rédaction et analyse académiques.
        [en] Anglais courant, expérimenté dans les discussions et recherches universitaires.
        [ru] Le russe, étudié pour sa littérature et sa tradition philosophique.
        [zh] Le chinois, appris pour dialoguer avec les perspectives académiques asiatiques.
        [it] L’italien, par intérêt pour l’histoire et la culture intellectuelle.
       *[other] Autre expérience linguistique.
    }
   *[other] { $lang ->
        [fr] Francophone.
        [en] Anglophone.
       *[other] Compétence linguistique.
    }
}
hireme = Je suis disponible pour une nouvelle mission !
website = Engagez-moi -> guillaume.maiano.fr

table-language = Langue
table-level = Niveau

-lang-name-fr = Français
-lang-name-en = Anglais
-lang-name-ru = Russe
-lang-name-zh = Chinois
-lang-name-it = Italien

lang-name = { $lang ->
   [fr] { -lang-name-fr }
   [en] { -lang-name-en }
   [ru] { -lang-name-ru }
   [zh] { -lang-name-zh }
   [it] { -lang-name-it }
  *[other] ???
}