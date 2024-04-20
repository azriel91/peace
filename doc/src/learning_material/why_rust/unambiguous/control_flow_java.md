# Control Flow - Java

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%;">

```java
public Success download(
    final String path
)
throws
    UnknownHostException,
    ConnectionLostException,
    OutOfDiskSpaceException
{
    //
}

public enum Success {
    DOWNLOADED,
    CACHED;
}
```

<!--

# /*
# {
#     File file = new File(path);
#     if (path.exists()) {
#         // Assume content is unchanged.
#         return Success.CACHED;
#     }
#
#     // ⚠️ throws UnknownHostException
#     // if IP address is of illegal length
#     InetAddress host = InetAddress.getByAddress(
#         new byte[] { 127, 0, 0, 1 }
#     );
#
#     String line = null;
#     try (
#         Socket socket = /* .. */;
#         BufferedReader input = /* .. */;
#         FileOutputStream out = /* .. */;
#     ) {
#         while(true) {
#             try {
#                 line = input.readLine();
#             } catch (IOException e) {
#                 throw new ConnectionLostException(e);
#             }
#
#             if (line == null) { break; }
#
#             try {
#                 out.writeLine(line);
#             } catch (IOException e) {
#                 throw new OutOfDiskSpaceException(e);
#             }
#         }
#     }
#
#     return Success.DOWNLOADED;
# }
# */

-->

</div>

<div style="flex-basis: 50%;">

<object
    type="image/svg+xml"
    data="control_flow.svg"
    width="250"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEWBmkAbaAfQBMB7AdwDsDz5T0sBvAX2y1MgGcBrY7gAd4SEpGrcArrnxIY1AC5M2HJOWrVoSBZHXFY0btCUt2OUhOJ1yvSYOK54hZWc5Va9UsSQjw0RqYcFDR0DMQ2LqCg1OSkJNTwALaGmDj4RGTuoQE4uXlYAETBHgwAFACUBUFZnuG8TPmNhTYAhE1V5jVhPij%2BDe09fqR5HZyW1rb2js5NOAWS1LwxNLkAPABGsAD0AHzg5NwKo2oaWjp6BkYmA%2Bqa2rrUG7BYu3SHo1x8AsKixOJSMkgcmgiiYBXIkgUWHIuCeLx2n14WCEImgVWisRIcW4SG4qTwhCxXQCBQ%2BxLqYLJIVqgz6hQ%2B43INjsDicBEpqluZwe%2BkMxg55h4-BRv3%2B0lk8hMpKiMTixGgiXIACsePj0kTqQwmIBeDcApzvVTVeCJYQCg5AaSl5aQFtYBdnaCjOZUzZTEAPBuABH3Oad7hc%2BSZADLkQSF31Rfwk4qBkp1gD6dqL%2BADmKQ46syhuIKYtFKwAG1k8VsgAaNypmwAXSThPT2TTeZpvjpOYrNYYhablrrpDLaUb5LIDsmrJmDYyrZbfZZ0wInYJw57Jzu52ovKuTCHGothbn3J9VynuZ7iODorDgOBoOze8NLaDIrEx4lIIUZdACj8yXxh1QRDxHBwsqx0EcSQCAUb9GmgAAPBQECYAByJBJFgbhyFgYBBHIcQFDgGCfyLDNjQAMjjAwQRwnBuHAeBBBINQ6FgJgiOgEi8lbbx2yYAAqBimNyCxuCsJl%2BwnJh8IMUhSORCiqO8chaKYUScM3b1F0ufksHY%2BTmOvH5bwBe8z3U-xQCAA)</small>

</div>
</div>

### Usage

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%;">

```java
// Use `download()`
try {
    download("a_file.txt");
    download("b_file.txt");
    upload("a_file.txt");
}
catch (OutOfDiskSpaceException e) {}
catch (UnknownHostException e) {}
catch (ConnectionLostException e) {}
```

</div>

<div style="flex-basis: 50%;">
<details><summary>Concept</summary>

<object
    type="image/svg+xml"
    data="control_flow_java_entangled.svg"
    width="370"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEWAmkBnAawH0CAHeJaEyAOwIFcAzZyJGOgF3SwG8AvtixIA9nTrQkXSOJKxoBaD35CcuBiQA2o0UUbkSzeJC29Bwtlpq5RAdzo74uEgEZzarFZv3Ho5yQATB6WpjQGTrghoHSiuDR08AC2ipg43iS2DpFuvDj5%2BQBEWX7OABQAlIWh1pm%2BOcEFTcX1-riV1elhJBFteU0Dhb3lFTideJo6egZGJmYDRYx0RLEO%2BQA8AEawAPQAfOCiBFzjYhJSMnIKSioLhWeS0rJ0W7BY%2BzrH4-jEZJTUtAYLDYHGg3F4hVEjC4WFEzFe7z2PyIWAoVGg1RicRsiiQBDSXm6JRy7iwhXGGWJbSCEIp3WGUTJ30mun0hmMplpwgeF2e8kUyi56kIpDRAPoTFY7E4KnJoCx8RI0CSogAVoQCZTWgFSYBeDcApzs1HzZamNA1GnrkSK8XWAE53hBoCNpWTMOfNADwbgAR97niR6XOj8m68QAy5A6RX90YDJSCZTbAH078uguAA5qkLVSdSRMizpuy5rwANpak06gA0EydUzZs1MAF109qXK4szynlcBSoi0TG25y63-YHlPWurUM02s8iI%2BKgVLQeCsF3Rz3XOXJ2KaBLgdKwVxh4SlyWXIEJznq27C8XSkfV6fXXM95eGi3fbz20GF4-TX2X22A9chw2h5BBO4brlGW5zp2n4BIEq6gf8G4zjGO4PvSVrUtmlYunmnIfmhkQ3lhuY1loqG1Ayz7nL%2Bg5QfhbTflRA7-ru8pcOAyrQASxyoNY%2BLCDgsSKvExiMFoXB8QM0AAB5cAgvAAORIIwsAEKIsDAOQoj0FwcDyfxWBJqmmTQCJYkSU0xywHoNAEOAzgJGpSTwPMABsAAMbn6SOWhkHZiqxLATnzAArB5%2BmOs6xHnlgABkCi4PwqJ2eQNBiDosC8PFWCeDg-Z8sxvAAFTxeF8GRpus6xlgxVJvp0HjphkVnvmsW4ForiJbZ8ApSQaVqQpqDQFoOh2PJ2V1d2QHNr1P5MR2RVta4E0Hlebggb8YEVch86FYty3Gqtx6NVWd64TFbWBJ1yWpaI6UKTpzljTl%2B4HU%2BM2Mfl83VRd%2B11EBR1rgh4GVTuC1aIEv0Ucd2EkbwMUMld3U3XdWDyeQylWtAT2Q%2BhATvX6n3voVDKgEAA)</small>

</details>
</div>

</div>

### Cleaner

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%;">

```java
try {
    download("a_file.txt");
}
catch (UnknownHostException e) {}
catch (ConnectionLostException e) {}
catch (OutOfDiskSpaceException e) {}

try {
    download("b_file.txt");
}
catch (UnknownHostException e) {}
catch (ConnectionLostException e) {}
catch (OutOfDiskSpaceException e) {}

try {
    upload("a_file.txt");
}
catch (UnknownHostException e) {}
catch (ConnectionLostException e) {}
```

</div>

<div style="flex-basis: 50%;">
<details><summary>Concept</summary>

<object
    type="image/svg+xml"
    data="control_flow_java_organized.svg"
    width="360"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEWAmkBnAawH0CAHeJaEyAOwIFcAzZyJGOgFxIEZ0sAbwC%2B2LEgD2dOtCRdIUkrGgFoPfkNE5cDEgBsJEoo3Ilm8SHr4CRoMfmJlK1WgxZsO0biQBMNreJSMnIKdEoqar7%2B9roGRiZmFlZ%2BmnY4ktKy8orKqjwAzNHasYbGpuaWJIWpYmx6NLgSAO50BvC41qk4dQ3NrRLtUV1YPSQmbbjRoHQSuDR08AC2Kpjdlr0tE504OzsARI2bA7gAFACUe7XrJIf9gym7uwd9E%2BeXa-Vj5BMCj39Ye3Gx3OOHeeBK8XKSW2-0BdCIMxaOwAPAAjWAAegAfOAJAQuGCMsFsmFcpENLCiVlQmjYFhsQZ8WCHKQKFQaPQmKx2Jx1AI9hJGFwsBJmLT6ViWVg2dRLjECPpSgkKslfrDGPDEXQUejsbimWIqSEchEeA8-nsjSTxQy8QT7IRWc4OW5uZ5vCkBUKRWL0RKpTLoHLigq4mVEpVqv8ARqEX0dZicXbCUFqSa8lU1Y9LanjXQbVjGfbprMGiokARViNrrctho9mDRrXjkMG1dPkD2vzmRDwyrOm30rmSeEM-XmY6nOzXFyPLyBz3Q0qoZVPSnMnnR5E1w7HIGZ%2B4eV4zd35YrIRGrNVB4ENyOyQVTyW5iRoIsJAArQhVpsvFsaQBeDcAU532w2O4OhSYDQK%2BH4sAAwATnbPMNlWhDRAB4NwAEfcNYdQi3PksEAGXJdydadOUPd18IAwA%2BnaQ5dLyGTDsLvXCHyGIjtEnfcyLdedIJokNzz7aFqkYodmPTSJqiI0BoFwABzFZoObQZeBIG5exQyoNAAbV-I4VIAGnBJcL37XgAF0lL-FS1KtFjTU6XSa2sjpeCMuyJPUSyPjAus1JZKcXG4udj0cvTwL4IyAq410Qu8CyrP0iD-I0ldVSwJzPmUiCotS%2BifG86sspc3xbJwzyhky3yWx8dzytJByCsSiKfH8zjnQPHjQpSKqbhK2q8Ha0jYqPD1CtGTsOnUkyhMjAResm3KZs0q9xuuSayvEhqM2qBbvmOOqtrwqpLNALhwDfaAq3xVB6krMQcBmF85nMRg9C4e6-mgAAPLgEAEAByJBGFgAgJFgYByAkeguDgAGHqwWSFJuaBXvez7HnxWAjBoAhwHaeZwcWeA9AEAA2AAGCmEY%2BKw8YJkgZlgYnSawABWKmEeykgjAEAAyOTlC8GnpXx8gaEkAxYAEQXoGF3ZuaQKgLsmLAACpZflnYdGWtLOj55RcCEUX4HFkhJfBgRDawAIxOJeyxwENXDa5oagpGijOmd2Suby-sUj58h6CIY36bNi3pawIP4RthGPO27cnejohXb3DrgtGk91eT33dfo6o%2BfofA5IkUOxYliQpYEIvIBL2Pdnj47qjVmuS4R8K-OmwSVv13A9F4MvTYrqusAB1BoD0AwmgB%2Budg7-9NvtiqNDVvveHb5ykr4Nq0%2BG2dM69teN%2BKrfWq75C9YDvufEH8PK8t0fYZJmfbaK6r7kXtME6z1e9B8Y-37JRuG7F0%2B9PYpF-v-XYE19qDHPnRfsBdJq32Hg-AG5AQbfGgC-ABMEWzm3qk3J2k1QBAA)</small>

</details>
</div>

</div>
