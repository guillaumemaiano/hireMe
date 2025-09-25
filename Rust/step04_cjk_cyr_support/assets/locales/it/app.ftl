intro = Ciao! Sono Guillaume.

audience = { $audience ->
    [business] In ambito professionale sono diretto e strategico — punto a risultati concreti e chiarezza.
    [academic] In ambito accademico sono riflessivo e analitico — mi piace approfondire idee complesse.
   *[other]   Adatto il mio stile alla situazione.
}

fluency = { $level ->
    [fluent] Posso lavorare in { $langName } senza problemi — riunioni, scrittura o negoziazioni.
    [learning] Sto imparando il { $langName } con costanza — abbastanza per esplorare idee e conversare.
   *[other] Sto ancora sviluppando le mie competenze in { $langName }.
}

language-profile = { $audience ->
    [business] { $lang ->
        [fr] Madrelingua francese, a mio agio anche in inglese in contesti professionali.
        [en] Inglese fluente, allo stesso livello del francese nella vita quotidiana e nel lavoro.
       *[other] (In ambito professionale lavoro principalmente in francese e inglese.)
    }
    [academic] { $lang ->
        [fr] Madrelingua francese, attivo nella scrittura e nell’analisi accademica.
        [en] Inglese fluente, con esperienza in discussioni e ricerche universitarie.
        [ru] Studio il russo per la sua letteratura e tradizione filosofica.
        [zh] Studio il cinese per comprendere la prospettiva asiatica.
        [it] Italiano come interesse per la storia e la cultura.
       *[other] Altra esperienza linguistica.
    }
   *[other] { $lang ->
        [fr] Francofono.
        [en] Anglofono.
       *[other] Competenza linguistica.
    }
}
hireme = Disponibile per una nuova collaborazione!
website = Scopri il mio profilo -> guillaume.maiano.fr
