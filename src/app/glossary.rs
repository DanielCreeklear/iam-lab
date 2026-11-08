use eframe::egui;

/// Vocabulário de apoio disponível sem sair da aula.
pub fn show(ui: &mut egui::Ui) {
    ui.collapsing("Preciso de uma base: símbolos e palavras",|ui|{
        for (word,meaning) in [
            ("Função","Uma regra que associa um valor a cada entrada. ψ(x) associa uma amplitude a cada posição x."),
            ("Derivada e curvatura","A derivada mede como uma função muda. A segunda derivada mede como essa mudança varia; no Hamiltoniano ela está ligada à energia cinética."),
            ("Integral","Soma contínua. No gráfico de densidade, integrar em um intervalo significa calcular a área correspondente à probabilidade."),
            ("Módulo ao quadrado","Se ψ=a+ib, então |ψ|²=a²+b². O resultado é real e não negativo. Para ψ real, basta elevá-la ao quadrado."),
            ("Fase","A orientação de uma amplitude complexa. Uma fase global não muda |ψ|²; fases relativas podem mudar a interferência."),
            ("Valor esperado <A>","Média de muitas medidas em sistemas preparados no mesmo estado. Não precisa ser um resultado possível de uma única medida."),
            ("ℏ (h cortado)","Constante de Planck dividida por 2π. Relaciona escalas quânticas de energia, tempo e momento angular."),
            ("eV, nm e a0","eV é unidade de energia (1 eV≈1,602×10^−19 J). nm é 10^−9 m. a0 é o raio de Bohr, cerca de 0,0529 nm."),
        ] {ui.strong(word);ui.label(meaning);}
    });
}
