use eframe::egui;

/// Definitions curtas abertas sem deixar a aula.
pub fn show(ui: &mut egui::Ui) {
    ui.collapsing("Não conhece uma palavra ou símbolo?", |ui| {
        for (word, meaning) in [
            (
                "Função de onda · ψ",
                "A onda que descreve um estado quântico. Ela associa uma amplitude a cada posição.",
            ),
            (
                "Operador · H",
                "Uma regra matemática aplicada à onda. H é o operador que representa a energia total.",
            ),
            (
                "Autovalor · E",
                "O número que sai quando o operador devolve a mesma onda multiplicada por um fator. Para H, esse número é uma energia.",
            ),
            (
                "Densidade · |ψ|²",
                "Uma medida de quanto a posição é provável. Some a densidade numa região para obter a chance de encontrar a partícula ali.",
            ),
            (
                "Normalizar",
                "Ajustar a escala da onda para que a chance total de encontrar a partícula seja 1, ou 100%.",
            ),
            (
                "Nó",
                "Posição ou superfície onde a onda vale zero. A chance de encontrar a partícula ali também é zero.",
            ),
            (
                "Fase",
                "Uma posição no ciclo de uma onda. A diferença de fase controla se duas ondas se somam ou se cancelam.",
            ),
            (
                "Valor médio · ⟨x⟩",
                "A média de muitas medidas feitas no mesmo estado. Pode ficar num lugar onde a chance de uma medida isolada é pequena.",
            ),
            (
                "ℏ · h cortado",
                "A constante de Planck dividida por 2π. Ela aparece em energias, momento e outras grandezas quânticas.",
            ),
            (
                "eV, nm e a₀",
                "Unidades de energia e comprimento: 1 eV≈1,60×10⁻¹⁹ J; 1 nm=10⁻⁹ m; a₀≈0,0529 nm.",
            ),
        ] {
            ui.add_space(5.0);
            ui.strong(word);
            ui.label(meaning);
        }
    });
}
