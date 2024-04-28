# Compiled vs Interpreted

## Compiled

<object
    type="image/svg+xml"
    data="compilation_cost.svg"
    width="700"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEXoA9okBXAF3gCMAbaTHHfQ0yAewDta76DiSXWB9SK1Zx0WAN4BfbFiTMAtgAdIleLzb8A7gkWLRMnHKUq1fDpyPLV6gUJGwxUmQGdmRZDQNZX7pNEHCohLSoKzMACb%2BrPDy0M4cliY2Wjp6DlgARBkyidZmmbmmbNmGClZFtoHpGQB0dfUNjU3NLa0NJbjcpBTUYhkAPOQAfACiXWRU0P0A9MMdDDz5GQtMxTIryXZBtW0dPh59g0MAym4eg7DTQwDCEVOzQ3tnfgH2fQBmrFjy8EIAFABKbKhO78aDyZgAK0g8RczxoWEAvBuAXZ3QKBoOEAOZxDj7F78Qo2MQAbTx0AANLIykk%2BABdHLUvIaMHjHoI4mEviUjZs%2BmgEjgcGeHDOEioaiwzhhSL8SLveBESgkeISTokBBiADkxFgrlgwEUzCEJDgmqw0k4UHCkXYWAAZFabar8Or4FrHdBWGaLXRKJBMeASGI7X6AyQvDh3ipKPxnOB4DKwrAfpQtQBGAAMGc1EawUcoMbjCf84GYADcgpqAKzZ3P5wvxmXvZjEFWazO1ziR6Oxxv%2BeBIXgV9NZnNd7zq5gAa38RcTzGT8FTWE1ABZR7nRbBp7O%2B-xSxX0pqAMwb8dbne94v8AdDhHVs9di8z2Ni3pYcLMEgm8JeHmTMQACpQ0DLxOQ0bR4F0IJAI9VgwMZCpHG8eM9AJZhKAXMQqCIaBzT-cZ8hwcQUKg-w5Ew9JMVgaBPXwixEM2KogLggjGCYt4sFgyBrU9LwyWDMk6x7OcogXFMR07OgyVefRx36fogKE89JxfTQeIFLVayAA)</small>


## Interpreted

<div style="display: flex; justify-content: center;">

<object
    type="image/svg+xml"
    data="interpreted_cost.svg"
    width="700"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQBMEsCcFMGMAukD2A7AXAAgG62svAIYA2oAFpPkdPOQJ4ahZawAeCArokQEYmwmLFvAD6ABmwBvAL7NhsCdLnCsYgIzL5LRZqyzta0QCYtqxaf0rhYgMxmFo%2B1cNiALA52iPL1WICsnqyigb42ogBsQYpRYSKiAOzRiUFiABzJGXFGAJzJeS6gaCjgimhEALawAM5CrBzw3HwC2ABEADy8AHwAog1N-LCd0AD0XQDKKJy0QyPdrfJiklitC-F6q4smbWtGzpvxPgdGocdisWcpK7vpO1sFx4rLj6IbuxZ3Xvvv3p-Bpz8Lj8ktd5Iosi8HgsiiVFLAKigAFaQWpg-o8QbYQC8G4BdndAoEQ5Hhgnk1UQ9AEqNUxVKolKADMiJwSIhavp6ohoERsAByRrQaooaDAAAOKEgaEQ%2BB5WGsOnAAHNFAymSy2QAySjgUpodnsTncrA8rU6mVy%2BpcDEtQwsemQEgkUTVchEWnFaAVUi89TicQ8m1YO0Op0u2nkFB4aC8-x%2BgNBx3O12KekoRpsnk%2B2OqW32hOhxREJCQPDe33%2B7NYMnQFAAa0UibdQs9JF5bjLAartfr%2BdE4cjvNs7YrnbrIaTokLyBLRpjWezI-r5JaWHAKEQUvAriUWHV8HZiZFingKBIQuw-E4sFlhie2HVV6klZdh9Ex9PUawCrgsF15o02AAKngG9XkA2At0sICQMg8C-CcQDgPMeCsAA2Dwh8KCkIwtD4lCTDHDwnCjFifCvBIoixBBUjgioijRCyajwTArcCkY0RWPAoA)</small>

</div>

<div class="hidden">

1. First we'll look at compiled versus interpreted languages.
2. For compiled languages, source code is processed by a compiler into executable CPU instructions.
3. When you run the application, the only work done is running the compiled logic.
4. For interpreted languages, such as a shell script, you are shipping the source code.
5. When you run the application, the source code is parsed and mapped to values during execution,
6. and then the relevant logic is run.
7. This interleaving of parsing and logic means the overall execution is slower.

8. An analogy is a cake recipe in a foreign language.
9. If I had a recipe given to me in Japanese, and I fully translated it up front,
10. then whenever I bake that cake, I can just use the recipe instructions.
11. If I did not translate it, and had to translate it every time I bake the cake,
12. then the process would be slow each time.

</div>
