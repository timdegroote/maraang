# maraang

Get help with creating an anagram. The tool will return whether an anagram for a
given string is valid. If it's not valid it will return which characters are
missing or superfluous.

```bash
$ maraang anagram maraang
+ "maraang" is an anagram of "anagram".

$ maraang anagram maraang anamarg maranga gnarama
+ "maraang" is an anagram of "anagram".
+ "anamarg" is an anagram of "anagram".
+ "maranga" is an anagram of "anagram".
+ "gnarama" is an anagram of "anagram".

$ maraang anagram foobar
- "foobar" is no anagram of "anagram".
	- Missing characters: ['a', 'a', 'g', 'm', 'n']
	- Leftover characters: ['b', 'f', 'o', 'o']
```
