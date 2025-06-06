# Aesthetics and Clarity

Aesthetics helps reduce the "ick" in a diagram, reducing the impedance that a user experiences when viewing a diagram. Examples of icks are:

1. Visual clutter.
2. A node's dimensions being significantly different compared to other nodes.
3. Oversaturated colour.
4. Colours that are excessively solid for a less relevant concept.

Clarity is how well a user understands the information that a diagram is presenting. Examples of adding clarity are:

1. (Understandable) visual cues such as emojis in place of text.
2. Reducing detail to what is most relevant.

Consider the last diagram from [Interaction Merging](interaction_merging.md#5-animate-and-show-edges-iteminteractions-for-each-step):

<object
    type="image/svg+xml"
    data="interaction_merging/item_locations_tagged_animated.svg"
    ></object>
<br />
<small><a href="https://azriel.im/dot_ix/#src=CYSwTgpgxgLiD2A7AXAAgG4THKBDANgFAAWIWuYUxAnsoaqsfAM4wD6H+8e+TrdDBn3aduBYRzYAHXDGIcY8DgHcIAIza4pUtgC8QUtAG8AvvUYsRbAOYg5AVw1R4AWwGCJHWw6evJxGBgpZi87Ykc2ZxcOXF0wMnwATgBGFXVNbQ5IfAhcZggQtmB4ZUQuXGAOAAY2VNS2VQ0tHX1DVFNzazB4ex0Y5WZ3Bi6evs0ByRHeyVwJjLZmHrlc1jYAJiHBKbHZwptu6f7CrQWl4hX2NckZOUlmAGY2NXsoAGsIKyrktfuAFgBWABsAHYAByJL7zRb2ZZ5S5sbIrAqbQSobYzOb7UYY446aGw1ZXDg3eQcB5PF7vT7fP5AsEQ1InfHnOHrBEQHJ5AqSRoZPo1OppJqZVrGMyERDwYAQNiIXAuZHmTxsLg8YRoVGarXahgAIlV4ksuqVlkkBt4puJslJiiFfL0BjQuoA9CTnYpnbzmgA6VrGoSWmxhCJRDU68MRyNR6Mx1C67zhNTeqL+iyrUI+SJ+DgBIKFBMh7OxeIclJ25pZDlIwrFUrlSpsAW1csix1x3PBZDO50FpNRZ3FhIpT3pZrOxFc5jO2tleAVZ1Vb3JJcj4VSX0GVPoo5h2PR3UAQQA6gBlLcHHaY7fjXGnGEswlOrQAWmZF2fa3P2KOkwvOJieJnBcbJWrcZKPM8bwfJIXw-ACILgpCTJAayVwTvkgx7rGuqwbSCEMs+L5vnCH7PuhBRfocN6-t+1FQihhLXNadwQZS0HVDS8H0khgH3sBaFVpOPKjpkjbNvUXqtm0LqSVIC5LiuskblIxqEBAwDWIqAbpiqYgWjpbDKr2WbRDmgTBBmiYmTEcRDhJIl9ORNYlLOFTVOJLYtAY7IAI72AUMBoAA2kZwa+KZhnmfmYXWZotmlvZa6VpyGEcDO9buYKDQOQ6UgADRplY5rCAAuiaOnGVEOaBsVgYkgoSjZUlrTsswUhIPkwWhZmUQFcqtU6fVbC2k19qtGV2lFXpypDSNsm5V4f4-ottG7ABd4EvCoGkgsrFQdScF0ohjK8ZtbJOcJSViVl82iqgIU1dNdXMcNjW3QYBXXmtK1Ud9yF8ahTFgbtFL7TBnFHQy9EA4xF2eZlzajRW42EIQXRaMQ6AgLomiBGAgzmOpmlZogrBgLgICIDABOov1T0Gd1VlVWZeaWYWpmDgl8Psil3JFC5GXXYj706JAfkBWgABmBD5ITGkyqA+ObHTaqBoZgaVdmkWs0GPVFvFSSJfaPPVmlAtzg2TZG8j3li-5-CoGouBvOYMhvGwLhShAT5gOT1DDapywKu4rDUDkNOCETCsQNL9j4NTKIYCAzAgGoID4HYtCoJTWMp2oOTmAwGfWAEaAAGTFwEheCJL6f4AsxAVDKkpgC4BBoAA5MkVRVB31cMLX+D18wjfSurmBgJ3-y9-3qCD8Po8ypL3D2JhXc933Wrzw3TeaLAICYJ33cz1qZPwO8O9jy3bf4J3vwb7PZ8XyPu9MBPnf3A-p8wN0z+L3vcBD6oA7tPE+mon4ylDjkNAxRAjqVnjIYAoBEDWA-pvDwj1Vb8FngAHhwWgAAVJXGAj9G5SBlM4Lgk9Hb4H8tXL6AxE54MIcQ0hWgKHwCoWgagHIuDKHoUtOiWJfpzH+mdDYuD8GoCISAEuJDT5kI4Vw1A8o1BYGrozdmTCpEyLkWw8hWZlGU1ANYeAqMYC4GsBHEAXwnRKTugAMXThAVAAARc2FRUw2I2HGexbYTz3FQAAeTUAAK2gDAQOli2B2AgC4axtjq7PkKmaemVh1YVRiszbWFldZM31iWQ23M4bpQtgja2UlfL23kYIZJmjwrVR0gNKws03o5RapANqHUICowYN4zYdTMGGkGi9Oa7TvLCMvHsBht5iKMW2ixUGVJwaHXwjxDaD54RwyRqJK28NWjmOiVA7ky8oCr3cDY5Iyshn6XSfU2KOTop6w5gbMsOzHKCVSvzOsZShYVK8qLCA4tsFalwIgEAbcYDe2AUFCBz5gB5CYJLSW+QYDPk9pgMiQLqm1BCBnRAuQwAxMQLXcFUKSroNRC-AxlD4DUPznQ7+v9IEwDDtC3UCKX6+1wLQKoeVP55W+HlNYeVfjCvFSK5I4qpVrCqKmKlP9z4ymUCAYAchO5BWSFIAAHhS2eudU7p0zmgA1+cem0w1lk7MKthktNGW05qtsCjtVJt7WeYKIWyGhR3WFir3jwsRfAZFqL0XwExZ0l1+RcUqkpoS4lpLYl6oUewwxdK0AMvNeAv1LK2VOk5Y3blvLhV8plYKiV4qxUitBBKvln95WCAgQ0VV6qYVrB1UmzUpqjWspNcnVOBdzD9I0Tcma9r9kTJmTRERsyGJbWkC9EGkFlkcVWdxE6Gz+ImyEtzPZ7zcqJw9ZC71vrmUBpHkGlFHxQ2YHWHi2NFB42U0TZShtijU30toZmhVzKFisugXGfN6l60MEbSqtVxANVtt1S+kD2bL7esBF-TU28X7Sk7rKsBqIu0Zx7UnPOBctQQG1T-XAaAAA+z5Z4MCCmXAAfLwgw+QSrIBPUq58+LoBaGQAcRAwAyqECgMwTCtHKMMAAALvGoJLcmCpmCoDhfmi9IaMUuKMNXKoABSQQRh5PZrPUiy9gVUCfwANyoDMKiY+WmdMKcDcGj4aAqhmYs+Z8wEmIBSZkwUXTp7FP2bRSprFwKYDtHU1phgNm9N+cM455z1crPtB82x6LqK0BgjiwwFz7nPPym87Z89-nr0QCxV011oXUSae00l-1KWHOoCc65yzPdrPVeK7Vozz5Eiggy65oAA">source</a></small>

The following things that could make the diagram more digestable:

1. **Aesthetic:** Reduce the visual length of the presented URL.

    1. For a github repository, separating the `username/repo` into a separate group can be informative.
    2. Showing the initial and last characters of the URL, while hiding the middle using ellipses may make it more aesthetic at a glance, though it may hinder if a user wants to see the full URL.

2. **Clarity:** Add an emoji indicating that `012345678901-ap-southeast-2-releases` is an S3 bucket.

Compare the above with the following:

<object
    type="image/svg+xml"
    data="aesthetics_and_clarity/item_locations_improved.svg"
    ></object>
<br />
<small><a href="https://azriel.im/dot_ix/#src=CYSwTgpgxgLiD2A7AXAAgG4THKBDANgFAAWIWuYUxAnsoaqsfAM4wD6H+8e+TrdDBn3aduBYRzYAHXDGIcY8DgHcIAIza4pUtgC8QUtAG8AvvUYsRbAOYg5AVw1R4AWwGCJHWw6evJuXTAyfABOAEYVdU1td0ELVklvYkc2ZxdJYhgYKWYvO2TfdM1A4PDIjS0dNkh8CFxmCFy2YHhlRC5cYA4ABjYIiLZVCu09A2MzBmsweHsqzWVmWNQpmbncBcTp2f8N6LZmGbk6hIAmJcmttd2bS52mrX3D4mP2E8kZOUlmAGY2NXsoABrCBWbphE7fAAsAFYAGwAdgAHCEwXsDvYjvVXtUILV6o1zoIVtsOOsmjdVndSTp0ZjTu9ZPIOD8-gDgaDwVC4UiUREHrTnli2G8asdGpIhtEqr1+uUpaNDKhTIRCIh4MAIGxELgXATzJ42FweMI0HEzeaLQwAERG8SWK36yySW28J0cD5MxRyyoKtBWgD0Hv9in9ksqADp9FIHUI3TZ8ik0qbLSnU2n0xnM6grUlHOG0jH4lZc4V-CVcWVBlFKsms3X6ymrQEghWwqHq9pCwaS6k-BxMtkmj20qTy6EBmGRji8Q0mi02h0umwZX1vSMo37-aL8cx-fP2vBOv7uuGwqf28MpJGDIXiVdFg2M1aAIIAdQAyrfbqTrneqWinheYU-S0ABaAUXlAk4v0pH9yT-ODqUeDFBXpd1GS+X5-iBEFJDBCEYQRZFUX5QChRFXExQfR80ytfCuSI3lQLAiCsSg0Dt1nGCSXmeDv14pDWLQ6QMOZLC2VwnpOUInkSJpMj6U48U12lVcJw7HQN2zC8pWPU9z0nK8owdVV1U1CAXHgAArEBFkdBJRGNSw0EAXg3AG6d+yrBdA0PQUJQq0vX1UBcwBBnc8xIE0KNBAB4NwAEffCvIfF7IpihbccVNcwAR3cAeD+EvjJKR37LIckSgpktHNLK0MjglLnVoD06Ho1JUoLQvMBDeLQQBAMlyi5YIErx+LJQSFNeGLAHh99qht-ab7nklCgLedDPjE1kcI5AjuWIvl5rpbFarQQA+DcAYr2pv64bBvO3ZSIW8iGRW-ZxPWvDpK23kANuxTKJ3CUNKa2UAvlLS2sICBgGsPVYwcw0xFdaG2G7SLyqKwdSsTPtm1KdTAunKiOH3Rd-tXQGfSjHEAEd7EaGA0AAbURgrXAAGiLZ1YeEABdPLhz7A1vLjXy2C9En1wMHFmCkJAGjphmysKhHiqHJHCsx1sWtxn7mnqwnl2akXNIMFm+fZgXROF6qoy5qGvJN6HBfNv6yYpHiLud+8Rs+7FlqZR61vZF7NsYuTkL24UNdnX6cZXbGgbGVB6bjfm7bN-yLcN5ZZs2K65pD1CvZEh6WWw-2pMD2Sdtzxbw+U-WiZj0mDC5wgpi0Yh0BAXRNCyMA7IYMGId7RBWDAXAQEQGBe48RPbasBG4x5lKBxK-K5Yxscqr+6u6oXQ8l2jlqycgSnqbQAAzAgGnMfvNVAHulmNpz4dl9HF8VtHS1SrH1dq-Htd3uuD5iyPlTfgqA1C4CBOYGQQI2CWQ1CBMAI9qBCxMkcXU7hWDUFqJPPu4Mb4QHPvYfAE9CTt2YCANQIB8B2FoKgMeZCKG1HMAwah1hMhoAAGSsMyMwwQp8qH4H2MQTomo1RgBcAQNAAByMI3RuhSN4Qwfh+BBHMGERqOemAwDSOhPIxRqBlGqPUZqU+3B7APhkXIhR5pDFCJEZoWAIBMDSNkXo80w94DAjsRosREj8DSMhFY-RHivFqPsUwLR0jvhBPcTAaYoTjEOLgM41AUjdFuLNCEzUmDahoBaFkMG+iZDAFAIgawUTrFT2hknGm+iAA8dS0AACpuEwGCcIqQmpnBcG0WA-AVNeEdTJISBpzTWntK0F0+APS0DUFxFwZQgzM6XRdtdXaedgL1MaagFpIA2FtPcR0qZMzUA6jUFgXhz8opbLGXsnhhzJm9hOWPUA1h4CXPnsrPsn81b6xGds3Z+yJmdKefAXpLy9nvMIDAXA1hJ4gDBH6Qy15FQADEqEQFQAAET-p0QsCKzjZmRVpd83xUAAHk1BWWgDAVBsK2B2AsvCxFvDQKs0cnaJ+nzGav1RivF+FUv61y3r-HejVdYAzTjoYB1NWXsv5R-B+nKrD21To7IBjRJZDwgCqBgBKlhsqVXDFVKdAFVCGTNbOHtQ5LQLj7IuEkNoMXLh9G1Ir1b71rlGFUMLrD7BgFg8UpioDmPcAisI99p6P1nlc5GCs+UL0Fb86q7qCb-wlcTKVFMQG1PNLgRAIAJEwAgNI2mWTQLAHqEwU+p8GgwFApZTAHEIDHwSGEXI1DEB1DAAyxA-CC3Fo5pUuIYSQXdLBWgNQ-SdWxPidkgNuTsyVrCYg3AtBuhM2iUzcETMThM0hLuw9e6wiHpPScbohYR1xM8ZqZQIBgByFLWEKQAAPId+iGGUOoQGtAn6mFxFjYVI1PlTVeo1RLKWJb9H5sLbIEtqSy3XuBBWqt8Aa11obfAJtkAIPar6B2se3be39sZe+h5Y7pkTr6QM2dN7-WBr9Mu4Rq7127o3We7dR7D0Hr3YiI9G7omXsEFkwY97H0IZOK+sjZpP1UJob+2yjCdXmH1R86pM97qejVYFJ2Fq+JWtdRs21gtfbF0ksuV6QcK5CX2t9COHq9ZSsJDBot8GpGIbnShtRaHa0gkw5gYUBGu0UGI2PUjw7hNHNBb0qdNHMlIfnQxpdVawZCYYCJu9D7iClsk2+iL6WEveLc7CGJZpbFhPgak89GS4iye-bQv9M6zQQBfXE3AaAAA+oF9EMFphwgAfPMgwDQObIA8ze0CnboBaGQFsRAwAm5QGYA+fr3WGAAAFgTUFPiPXUzBUDlqYz5jDjbMVGF4d0AApIIIwB2Eteerb5mmqBokAG5UATEEK467t3DuofQyCNA3R3ufc+5tiA23duNDu55o7AP62neba2mASoLvXYYL9+7cOntA5B7w77SoYcTex3WtASI8cMDB1tnbOpod-e8-D-zEBm24YaKjuIV2btE+QyTwHqBgcffx3In73Pme8+e6BEIiIKeC6AA">source</a></small>

Notes:

1. The URL is shortened into the path after the `username/repo`  This requires the `FileDownload` item to know that the first 2 segments is a group namespace.
2. The 🪣 bucket emoji clarified that the `012345678901-ap-southeast-2-releases` node represents an S3 bucket.
