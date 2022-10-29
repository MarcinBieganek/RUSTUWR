fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|c| {
            match c {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => c
            }
        })
        .collect()
}

mod tests {
    use super::dna_strand;

    fn dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert!(actual == expected, 
            "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }
    
    #[test]
    fn fixed_tests() {
        dotest("AAAA","TTTT");
        dotest("ATTGC","TAACG");
        dotest("GTAT","CATA");
        dotest("CCAA", "GGTT");
        dotest("", "");
    }
}