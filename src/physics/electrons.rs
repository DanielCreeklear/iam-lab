//! Preenchimento Aufbau + Hund para átomos neutros H a Ar, estado fundamental.
pub struct Subshell {
    pub label: &'static str,
    pub l: u32,
    pub electrons: u32,
}
pub fn configuration(z: u32) -> Vec<Subshell> {
    let mut remaining = z.min(18);
    let mut result = Vec::new();
    for (label, l) in [("1s", 0), ("2s", 0), ("2p", 1), ("3s", 0), ("3p", 1)] {
        let electrons = remaining.min(2 * (2 * l + 1));
        result.push(Subshell {
            label,
            l,
            electrons,
        });
        remaining -= electrons;
    }
    result
}
/// Pares [up,down] em cada orbital; primeiro ocupa spins paralelos.
pub fn occupancy(l: u32, electrons: u32) -> Vec<[bool; 2]> {
    let count = (2 * l + 1) as usize;
    (0..count)
        .map(|i| {
            [
                i < (electrons as usize).min(count),
                i + count < electrons as usize,
            ]
        })
        .collect()
}
pub fn unpaired(z: u32) -> u32 {
    configuration(z)
        .iter()
        .flat_map(|s| occupancy(s.l, s.electrons))
        .filter(|o| o[0] != o[1])
        .count() as u32
}
