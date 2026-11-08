# IAM Lab

Laboratório desktop em **Rust + egui/eframe**, em português, para estudar **Interações Atômicas e Moleculares (BCK0104-15) da UFABC**.

Sete laboratórios interativos cobrem os grandes eixos da ementa. Cada tela apresenta parâmetros, unidades, modelo, uma pergunta para exploração e suas limitações. Revisões e anotações são salvas localmente pelo eframe; parâmetros das simulações reiniciam ao abrir.

## Executar

Instale [Rust estável](https://rustup.rs/). No Windows, instale também as ferramentas C++ do Visual Studio (Desktop development with C++ e Windows SDK).

```sh
cargo run --release
```

O primeiro build baixa dependências. Depois, o aplicativo funciona offline. O executável Windows fica em `target/release/iam-lab.exe`. A integração contínua também gera um artefato Windows para download na aba Actions do GitHub.

## Laboratórios

| Tema | Experimento implementado |
|---|---|
| Fundamentos quânticos | Poço infinito, energia versus confinamento, distribuição radial 1s hidrogenoide |
| Ligação de valência | Sobreposição 1s–1s e curva fenomenológica de Morse |
| Orbital molecular | Combinações LCAO ligante/antiligante, nó e ordem de ligação |
| Interações elétricas | Orientação de dipolos e potencial Lennard-Jones |
| Líquidos | Monte Carlo Metropolis 2D, temperatura, densidade e energia |
| Cristais | Redes cúbica simples, CCC e CFC, rotação da projeção e empacotamento |
| Bandas e materiais | Duas bandas parametrizadas, gap, potencial químico e Fermi-Dirac |

Esta versão contém um experimento introdutório para cada eixo, não a totalidade de todos os subtópicos possíveis. Consulte [ementa e cobertura](docs/ementa.md) para fontes, lacunas e extensões. Não é software de química computacional quantitativa nem material oficial da UFABC.

## Organização do código

```text
src/
  main.rs              # Inicialização da janela
  app.rs               # Navegação, progresso e anotações
  lib.rs               # Biblioteca de física testável sem janela
  physics/             # Fórmulas e amostragem, sem dependência da interface
    quantum.rs
    bonding.rs
    interactions.rs
    liquid.rs
    solid.rs
  views/               # Controles e gráficos separados por assunto
tests/physics.rs       # Invariantes e referências analíticas
docs/                  # Fontes, arquitetura e limites
```

Identificadores seguem o padrão Rust em inglês; interface, comentários explicativos e documentação estão em português. As funções físicas recebem unidades explicitadas nos comentários; a interface limita os parâmetros aos domínios válidos.

## Verificar e contribuir

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

Antes de adicionar um experimento, escreva o modelo em `physics`, teste uma propriedade física independente e só então conecte uma tela em `views`. Documente unidades, hipóteses e validade. Veja [arquitetura](docs/arquitetura.md).

## Fontes

- [Plano e ementa do professor Pieter Westera, UFABC, 2025.3](http://professor.ufabc.edu.br/~pieter.westera/IAM_2503.html).
- [Plano de ensino de Michel Mendoza, UFABC, 2022.2](https://ccnh.ufabc.edu.br/arquivos/CENTRAL/4.Ensino/5.Planos-ensino/QS_2022-2/Plano-Ensino-IAM-2022-2-Michel-Mendoza.pdf).
- [Catálogo de disciplinas da graduação 2019–2020, p. 657](https://prograd.ufabc.edu.br/pdf/catalogo_disciplinas_graduacao_2019_2020.pdf).
- [eframe](https://docs.rs/eframe/0.33.0/eframe/) e [egui_plot](https://docs.rs/egui_plot/0.34.0/egui_plot/).

Pesquisa consultada em 08/11/2026. O plano específico da sua turma pode mudar a ordem ou aprofundar outros conteúdos.
