# Rusttexto

Aren't you tired from writing Rust programs in English? Do you like saying
"fek" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Do you _hope_ to bring some Esperanto touch to your
programs?

**rustteksto** (Esperanto for _Rust-programming-language_) is here to save your
day, as it allows you to write Rust programs in Esperanto, using Esperanto keywords,
Esperanto function names, Esperanto idioms.

You're from The US and don't feel at ease using only Esperanto words? Don't worry!
Rustoprogramteksto is fully compatible with English-Rust, so you can mix both at
your convenience.

Here's an example of what can be achieved with Rustteksto:

### trait and impl (aka traijto kaj realigo)

```rust
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
}
```

### Support for regional languages

```rust
#[permesi(netingebla_programteksto)]
funkcio malĉefa() {
    fek!("o fek!"); // for the true Esperanto experience
    ekpaniki!("La programo malsukcesis"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Bone, je la fino.

## Kontribuo

First of all, _dankon_ for considering participating to this joke! Feel free to
throw in a few identifiers here and there, and open a pull-request against the
`ĉefa` (Esperanto for `main`) branch.

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)

## por kio, kvankam?
- saw [this french thing](https://github.com/bnjbvr/rouille) and thought it would be funny to expand it to Esperanto.

## la permeso

[WTFPL](http://www.wtfpl.net/).
