rustteksto::rustteksto! {
    exter kesto rustteksto;

    uzi std::kolektoj::Tradukaĵo kiel Tradu;

    trajto KlavValoro {
        fn skribi(&memo, klavo: Ĉeno, valoro: Cxeno);
        // Ambaŭ la diakritaj    ^ kaj         ^^ ASCII-transliterumado estas
        // akceptitaj
        fn legi(&memo, klavo: Ĉeno) -> Rezulto<Malnepra<&Ĉeno>, Ĉeno>;
    }

    senmova ŝanĝebla TRADUKAĴO: Malnepra<Tradu<Ĉeno, Ĉeno>> = Nenio;

    strukt Konkreta;

    realigo KlavValoro por Konkreta {
        funkcio skribi(&memo, klavo: Ĉeno, valoro: Ĉeno) {
            ebligi tradu = malsekura {
                TRADUKAĴO.akiri_aŭ_enigo_kun(Defaŭlto::defaŭlto)
            };
            tradu.enmeti(klavo, valoro);
        }
        funkcio legi(&memo, klavo: Ĉeno) -> Rezulto<Malnepre<&Ĉeno>, Ĉeno> {
            ĉu ebligi Io(tradu) = malsekura { TRADUKAĴO.kiel_ref() } {
                Bone(tradu.legi(&klavo))
            } plu {
                Er("Tradukaĵo mankas".igi())
            }
        }
    }

    publika(kesto) funkcio malnepra(i: u32) -> Malnepre<Rezulto<u32, Ĉeno>> {
        ĉu i % 2 == 1 {
            ĉu i == 42 {
                Io(Er(Ĉeno::el("fek!")))
            } plu {
                Io(Bone(33))
            }
        } plu {
            Nenio
        }
    }

    malsink funkcio ekzemplo() {
    }

    malsink funkcio ekzemplo2() {
        ekzemplo().atendi;
    }

    funkcio ĉefo() {
        ebligi ŝanĝebla x = 31;

        kompari x {
            42 => {
                printovico!("omelette du fromage")
            }
            _ => printovico!("voila")
        }

        por i en 0..10 {
            ebligi val = iteracii {
                eksplodi i;
            };

            dum x < val {
                x += 1;
            }

            x = ĉu ebligi Io(resultat) = malnepra(i) {
                resultat.déballer()
            } plu {
                12
            };
        }

        //malĉefa();
    }

    #[permesi(netingebla_programteksto)]
    funkcio malĉefa() {
        fek!("o fek!"); // for the true Esperanto experience
        ekpaniki!("La programo malsukcesis"); // in SFW contexts
    }
}
