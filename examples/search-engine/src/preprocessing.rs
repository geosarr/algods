use crate::collection::Document;
use crate::constant::PUNCTUATION;
use std::collections::{HashMap, HashSet};

pub fn character_ngram(word: &str, size: usize) -> HashSet<String> {
    let _word = word.trim();
    let len_chars: Vec<_> = _word.chars().map(|ch| ch.len_utf8()).collect();
    let n = len_chars.len();
    if n <= size {
        return HashSet::from([word.to_string()]);
    }
    let mut ngrams = HashSet::new();
    let mut pos = 0;
    let mut chars = _word.chars();
    let mut i = 0;
    for i in 0..n - size + 1 {
        let mut len = 0;
        for j in i..i + size {
            len += len_chars[j]
        }
        let ngram = (&_word[pos..pos + len]).to_string();
        ngrams.insert(ngram);
        pos += len_chars[i];
    }
    ngrams
}

pub fn is_not_punct(character: char) -> bool {
    !PUNCTUATION.contains(character)
}

pub fn preprocess(doc: &Document) -> HashMap<String, usize> {
    let mut content = doc.content().to_lowercase();
    content.retain(is_not_punct);
    let content = content.split_whitespace().collect();
    let mut counter = Counter::new();
    counter.count(content)
}

pub struct Counter {}

impl Counter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn count(&mut self, content: Vec<&str>) -> HashMap<String, usize> {
        let mut counter = HashMap::with_capacity(content.len());
        for word in content {
            if let Some(count) = counter.get_mut(word) {
                *count += 1;
            } else {
                counter.insert(word.to_string(), 1);
            }
        }
        counter
    }
}

// Adapted from https://iq.opengenus.org/porter-stemmer/
pub struct PorterStemmer {}
impl PorterStemmer {
    pub fn new() -> Self {
        Self {}
    }

    fn isCons(&self, letter: &char) -> bool {
        // function that returns true if a letter is a consonant otherwise false
        if letter == &'a' || letter == &'e' || letter == &'i' || letter == &'o' || letter == &'u' {
            false
        } else {
            true
        }
    }

    fn isConsonant(&self, word: &str, i: usize) -> bool {
        // function that returns true only if the letter at i th position
        // in the argument 'word' is a consonant.But if the letter is 'y' and the letter at i-1 th position
        // is also a consonant, then it returns false.
        let letter = word.chars().nth(i).expect("Failed to get char.");
        if self.isCons(&letter) {
            let char_before = word.chars().nth(i - 1).expect("Failed to get char.");
            if letter == 'y' && self.isCons(&char_before) {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    fn isVowel(&self, word: &str, i: usize) -> bool {
        // function that returns true if the letter at i th position in the argument 'word'
        // is a vowel
        return !self.isConsonant(word, i);
    }
    // # *S
    fn endsWith(&self, stem: &str, letter: &str) -> bool {
        // returns true if the word 'stem' ends with 'letter'
        stem.ends_with(letter)
    }
    // # *v*
    fn containsVowel(&self, stem: &str) -> bool {
        // returns true if the word 'stem' contains a vowel
        for letter in stem.chars() {
            if !self.isCons(&letter) {
                return true;
            }
        }
        false
    }

    // # *d
    fn doubleCons(&self, stem: &str) -> bool {
        // returns true if the word 'stem' ends with 2 consonants
        if stem.len() >= 2 {
            if self.isConsonant(stem, stem.len() - 1) && self.isConsonant(stem, stem.len() - 2) {
                return true;
            } else {
                return false;
            }
        }
        false
    }

    fn getForm(&self, word: &str) -> String {
        // This function takes a word as an input, and checks for vowel and consonant sequences in that word.
        // vowel sequence is denoted by V and consonant sequences by C
        // For example, the word 'balloon' can be divived into following sequences:
        // 'b' : C
        // 'a' : V
        // 'll': C
        // 'oo': V
        // 'n' : C
        // So form = [C,V,C,V,C] and form_str = CVCVC
        let mut form = Vec::new();
        let mut form_str = String::new();
        for i in 0..word.len() {
            if self.isConsonant(word, i) {
                if i != 0 {
                    let prev = form[form.len() - 1];
                    if prev != 'C' {
                        form.push('C');
                    }
                } else {
                    form.push('C');
                }
            } else {
                if i != 0 {
                    let prev = form[form.len() - 1];
                    if prev != 'V' {
                        form.push('V');
                    }
                } else {
                    form.push('V');
                }
            }
        }
        for j in &form {
            form_str.push(*j);
        }
        return form_str;
    }
    // def getM(self, word):
    //     #returns value of M which is equal to number of 'VC' in formstr
    //     #So in above example of word 'balloon', we have 2 'VC'

    //     form = self.getForm(word)
    //     m = form.count('VC')
    //     return m

    // # *o
    // def cvc(self, word):
    //     #returns true if the last 3 letters of the word are of the following pattern: consonant,vowel,consonant
    //     #but if the last word is either 'w','x' or 'y', it returns false
    //     if len(word) >= 3:
    //         f = -3
    //         s = -2
    //         t = -1
    //         third = word[t]
    //         if self.isConsonant(word, f) and self.isVowel(word, s)
    //         and self.isConsonant(word, t):
    //             if third != 'w' and third != 'x' and third != 'y':
    //                 return True
    //             else:
    //                 return False
    //         else:
    //             return False
    //     else:
    //         return False

    // def replace(self, orig, rem, rep):
    //     #this function checks if string 'orig' ends with 'rem' and
    //     #replaces 'rem' by the substring 'rep'. The resulting string 'replaced'
    //     #is returned.

    //     result = orig.rfind(rem)
    //     base = orig[:result]
    //     replaced = base + rep
    //     return replaced

    // def replaceM0(self, orig, rem, rep):
    //     #same as the function replace(), except that it checks the value of M for the
    //     #base string. If it is >0 , it replaces 'rem' by 'rep', otherwise it returns the
    //     #original string

    //     result = orig.rfind(rem)
    //     base = orig[:result]
    //     if self.getM(base) > 0:
    //         replaced = base + rep
    //         return replaced
    //     else:
    //         return orig

    // def replaceM1(self, orig, rem, rep):
    //     #same as replaceM0(), except that it replaces 'rem' by 'rep', only when M>1 for
    //     #the base string

    //     result = orig.rfind(rem)
    //     base = orig[:result]
    //     if self.getM(base) > 1:
    //         replaced = base + rep
    //         return replaced
    //     else:
    //         return orig

    // def step1a(self, word):
    //     #In a given word, this function replaces 'sses' by 'ss', 'ies' by 'i',
    //     #'ss' by 'ss' and 's' by ''

    //     """step1a() gets rid of plurals. e.g.

    //        caresses  ->  caress
    //        ponies    ->  poni
    //        ties      ->  ti
    //        caress    ->  caress
    //        cats      ->  cat
    //     """
    //     if word.endswith('sses'):
    //         word = self.replace(word, 'sses', 'ss')
    //     elif word.endswith('ies'):
    //         word = self.replace(word, 'ies', 'i')
    //     elif word.endswith('ss'):
    //         word = self.replace(word, 'ss', 'ss')
    //     elif word.endswith('s'):
    //         word = self.replace(word, 's', '')
    //     else:
    //         pass
    //     return word

    // def step1b(self, word):
    //     #this function checks if a word ends with 'eed','ed' or 'ing' and replces these substrings by
    //     #'ee','' and ''. If after the replacements in case of 'ed' and 'ing', the resulting word
    //     # -> ends with 'at','bl' or 'iz' : add 'e' to the end of the word
    //     # -> ends with 2 consonants and its last letter isn't 'l','s' or 'z': remove last letter of the word
    //     # -> has 1 as value of M and the cvc(word) returns true : add 'e' to the end of the word

    //     '''
    //     step1b gets rid of -eed -ed or -ing. e.g.

    //     feed      ->  feed
    //        agreed    ->  agree
    //        disabled  ->  disable

    //        matting   ->  mat
    //        mating    ->  mate
    //        meeting   ->  meet
    //        milling   ->  mill
    //        messing   ->  mess

    //        meetings  ->  meet
    //     '''
    //     flag = False
    //     if word.endswith('eed'):
    //         result = word.rfind('eed')
    //         base = word[:result]
    //         if self.getM(base) > 0:
    //             word = base
    //             word += 'ee'
    //     elif word.endswith('ed'):
    //         result = word.rfind('ed')
    //         base = word[:result]
    //         if self.containsVowel(base):
    //             word = base
    //             flag = True
    //     elif word.endswith('ing'):
    //         result = word.rfind('ing')
    //         base = word[:result]
    //         if self.containsVowel(base):
    //             word = base
    //             flag = True
    //     if flag:
    //         if word.endswith('at') or word.endswith('bl')
    //         or word.endswith('iz'):
    //             word += 'e'
    //         elif self.doubleCons(word) and not self.endsWith(word, 'l')
    //         and not self.endsWith(word, 's') and not self.endsWith(word, 'z'):
    //             word = word[:-1]
    //         elif self.getM(word) == 1 and self.cvc(word):
    //             word += 'e'
    //         else:
    //             pass
    //     else:
    //         pass
    //     return word

    // def step1c(self, word):
    //     #In words ending with 'y' this function replaces 'y' by 'i'

    //     """step1c() turns terminal y to i when there is another vowel in the stem."""

    //     if word.endswith('y'):
    //         result = word.rfind('y')
    //         base = word[:result]
    //         if self.containsVowel(base):
    //             word = base
    //             word += 'i'
    //     return word

    // def step2(self, word):
    //     #this function checks the value of M, and replaces the suffixes accordingly

    //     """step2() maps double suffices to single ones.
    //     so -ization ( = -ize plus -ation) maps to -ize etc. note that the
    //     string before the suffix must give m() > 0.
    //     """

    //     if word.endswith('ational'):
    //         word = self.replaceM0(word, 'ational', 'ate')
    //     elif word.endswith('tional'):
    //         word = self.replaceM0(word, 'tional', 'tion')
    //     elif word.endswith('enci'):
    //         word = self.replaceM0(word, 'enci', 'ence')
    //     elif word.endswith('anci'):
    //         word = self.replaceM0(word, 'anci', 'ance')
    //     elif word.endswith('izer'):
    //         word = self.replaceM0(word, 'izer', 'ize')
    //     elif word.endswith('abli'):
    //         word = self.replaceM0(word, 'abli', 'able')
    //     elif word.endswith('alli'):
    //         word = self.replaceM0(word, 'alli', 'al')
    //     elif word.endswith('entli'):
    //         word = self.replaceM0(word, 'entli', 'ent')
    //     elif word.endswith('eli'):
    //         word = self.replaceM0(word, 'eli', 'e')
    //     elif word.endswith('ousli'):
    //         word = self.replaceM0(word, 'ousli', 'ous')
    //     elif word.endswith('ization'):
    //         word = self.replaceM0(word, 'ization', 'ize')
    //     elif word.endswith('ation'):
    //         word = self.replaceM0(word, 'ation', 'ate')
    //     elif word.endswith('ator'):
    //         word = self.replaceM0(word, 'ator', 'ate')
    //     elif word.endswith('alism'):
    //         word = self.replaceM0(word, 'alism', 'al')
    //     elif word.endswith('iveness'):
    //         word = self.replaceM0(word, 'iveness', 'ive')
    //     elif word.endswith('fulness'):
    //         word = self.replaceM0(word, 'fulness', 'ful')
    //     elif word.endswith('ousness'):
    //         word = self.replaceM0(word, 'ousness', 'ous')
    //     elif word.endswith('aliti'):
    //         word = self.replaceM0(word, 'aliti', 'al')
    //     elif word.endswith('iviti'):
    //         word = self.replaceM0(word, 'iviti', 'ive')
    //     elif word.endswith('biliti'):
    //         word = self.replaceM0(word, 'biliti', 'ble')
    //     return word

    // def step3(self, word):
    //     #this function checks the value of M, and replaces the suffixes accordingly

    //     """step3() dels with -ic-, -full, -ness etc. similar strategy to step2."""

    //     if word.endswith('icate'):
    //         word = self.replaceM0(word, 'icate', 'ic')
    //     elif word.endswith('ative'):
    //         word = self.replaceM0(word, 'ative', '')
    //     elif word.endswith('alize'):
    //         word = self.replaceM0(word, 'alize', 'al')
    //     elif word.endswith('iciti'):
    //         word = self.replaceM0(word, 'iciti', 'ic')
    //     elif word.endswith('ful'):
    //         word = self.replaceM0(word, 'ful', '')
    //     elif word.endswith('ness'):
    //         word = self.replaceM0(word, 'ness', '')
    //     return word

    // def step4(self, word):
    //     #this function checks the value of M, and replaces the suffixes accordingly

    //     """step4() takes off -ant, -ence etc., in context <c>vcvc<v>{meaning, M >1 for the word}."""

    //     if word.endswith('al'):
    //         word = self.replaceM1(word, 'al', '')
    //     elif word.endswith('ance'):
    //         word = self.replaceM1(word, 'ance', '')
    //     elif word.endswith('ence'):
    //         word = self.replaceM1(word, 'ence', '')
    //     elif word.endswith('er'):
    //         word = self.replaceM1(word, 'er', '')
    //     elif word.endswith('ic'):
    //         word = self.replaceM1(word, 'ic', '')
    //     elif word.endswith('able'):
    //         word = self.replaceM1(word, 'able', '')
    //     elif word.endswith('ible'):
    //         word = self.replaceM1(word, 'ible', '')
    //     elif word.endswith('ant'):
    //         word = self.replaceM1(word, 'ant', '')
    //     elif word.endswith('ement'):
    //         word = self.replaceM1(word, 'ement', '')
    //     elif word.endswith('ment'):
    //         word = self.replaceM1(word, 'ment', '')
    //     elif word.endswith('ent'):
    //         word = self.replaceM1(word, 'ent', '')
    //     elif word.endswith('ou'):
    //         word = self.replaceM1(word, 'ou', '')
    //     elif word.endswith('ism'):
    //         word = self.replaceM1(word, 'ism', '')
    //     elif word.endswith('ate'):
    //         word = self.replaceM1(word, 'ate', '')
    //     elif word.endswith('iti'):
    //         word = self.replaceM1(word, 'iti', '')
    //     elif word.endswith('ous'):
    //         word = self.replaceM1(word, 'ous', '')
    //     elif word.endswith('ive'):
    //         word = self.replaceM1(word, 'ive', '')
    //     elif word.endswith('ize'):
    //         word = self.replaceM1(word, 'ize', '')
    //     elif word.endswith('ion'):
    //         result = word.rfind('ion')
    //         base = word[:result]
    //         if self.getM(base) > 1
    //         and (self.endsWith(base, 's') or self.endsWith(base, 't')):
    //             word = base
    //         word = self.replaceM1(word, '', '')
    //     return word

    // def step5a(self, word):
    //     #this function checks if the word ends with 'e'. If it does, it checks the value of
    //     #M for the base word. If M>1, OR, If M = 1 and cvc(base) is false, it simply removes 'e'
    //     #ending.

    //     """step5() removes a final -e if m() > 1.
    //     """

    //     if word.endswith('e'):
    //         base = word[:-1]
    //         if self.getM(base) > 1:
    //             word = base
    //         elif self.getM(base) == 1 and not self.cvc(base):
    //             word = base
    //     return word

    // def step5b(self, word):
    //     #this function checks if the value of M for the word is greater than 1 and it ends with 2 consonants
    //     # and it ends with 'l', it removes 'l'

    //     #step5b changes -ll to -l if m() > 1
    //     if self.getM(word) > 1 and self.doubleCons(word)
    //     and self.endsWith(word, 'l'):
    //         word = word[:-1]
    //     return word

    // def stem(self, word):
    //     #steps to be followed for stemming according to the porter stemmer :)

    //     word = self.step1a(word)
    //     word = self.step1b(word)
    //     word = self.step1c(word)
    //     word = self.step2(word)
    //     word = self.step3(word)
    //     word = self.step4(word)
    //     word = self.step5a(word)
    //     word = self.step5b(word)
    //     return word
}
